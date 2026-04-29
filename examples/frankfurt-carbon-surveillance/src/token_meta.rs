//! Global in-memory `mint → TokenMeta` cache.
//!
//! Two paths populate the cache:
//!
//!   1. PumpFun `Create` / `CreateV2` ix args carry `name` + `symbol` —
//!      captured for free, real-time, the moment Carbon sees a launch.
//!      Doesn't include `image` (that lives at the metadata URI, which
//!      Carbon doesn't fetch).
//!
//!   2. For mints we never observed launching (older tokens, non-pump.fun
//!      mints), the coordinator's flush path triggers a fire-and-forget
//!      Helius DAS `getAsset` lookup the first time it sees an unknown
//!      mint. The miss event itself ships immediately with `token_symbol:
//!      None` (no critical-path delay); ~50–200 ms later DAS responds and
//!      every subsequent event for that mint is a cache hit.
//!
//! Cache scope: process lifetime. No TTL — token metadata is effectively
//! immutable once assigned. After hours of soak, cache covers ~all
//! actively-traded mints regardless of when they were minted.
//!
//! Misses are negatively cached (`TokenMeta::default()` with all fields
//! `None` and `fetched: true`) so we don't keep re-DAS-ing dead mints.
//! Concurrent fetches for the same mint dedup via the `IN_FLIGHT` set.

use dashmap::{DashMap, DashSet};
use once_cell::sync::Lazy;
use std::time::Duration;

#[derive(Clone, Debug, Default)]
pub struct TokenMeta {
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub image: Option<String>,
    /// True once we've attempted to fully resolve via DAS (or captured
    /// from a Create ix). Stops repeat DAS fetches for the same mint
    /// even when DAS came back empty.
    pub fetched: bool,
}

impl TokenMeta {
    fn merged_with(self, other: TokenMeta) -> TokenMeta {
        TokenMeta {
            symbol: self.symbol.or(other.symbol),
            name: self.name.or(other.name),
            image: self.image.or(other.image),
            fetched: self.fetched || other.fetched,
        }
    }
}

pub static MINT_META: Lazy<DashMap<String, TokenMeta>> = Lazy::new(DashMap::new);

/// Mints with a DAS fetch currently in flight. Read-then-insert under
/// DashSet's per-shard lock prevents 50 concurrent miss events from
/// triggering 50 concurrent DAS calls for the same mint.
static IN_FLIGHT: Lazy<DashSet<String>> = Lazy::new(DashSet::new);

/// Insert a partial mint→meta from a Create ix observation. Merges with
/// any existing entry rather than overwriting (a later DAS fetch may
/// have filled in `image`; we don't want to clobber it with `None`).
/// The `fetched` flag stays whatever it was — Create-only entries are
/// considered "partial" and DAS may still fill in `image`.
pub fn record_from_create(mint: &str, symbol: Option<&str>, name: Option<&str>) {
    let new = TokenMeta {
        symbol: symbol
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from),
        name: name
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(String::from),
        image: None,
        fetched: false,
    };
    MINT_META
        .entry(mint.to_string())
        .and_modify(|existing| {
            // Keep already-set fields; only fill in missing ones.
            *existing = existing.clone().merged_with(new.clone());
        })
        .or_insert(new);
}

/// Insert (or replace) a fully-resolved mint→meta from a DAS fetch.
/// Sets `fetched: true` so we won't re-fetch even if some fields are
/// None (negative cache for dead mints).
pub fn record_from_das(
    mint: &str,
    symbol: Option<String>,
    name: Option<String>,
    image: Option<String>,
) {
    let trim = |s: Option<String>| s.and_then(|x| {
        let t = x.trim().to_string();
        if t.is_empty() { None } else { Some(t) }
    });
    let new = TokenMeta {
        symbol: trim(symbol),
        name: trim(name),
        image: trim(image),
        fetched: true,
    };
    MINT_META
        .entry(mint.to_string())
        .and_modify(|existing| {
            // Keep already-set fields; only fill in missing ones.
            *existing = existing.clone().merged_with(new.clone());
        })
        .or_insert(new);
}

/// Look up the cached meta for a mint, if any.
pub fn lookup(mint: &str) -> Option<TokenMeta> {
    MINT_META.get(mint).map(|v| v.value().clone())
}

/// Number of cached mint→meta entries. Exposed as a Prometheus gauge.
pub fn cache_size() -> usize {
    MINT_META.len()
}

/// Trigger a fire-and-forget DAS `getAsset` lookup for `mint` if (a) we
/// don't already have a fully-resolved entry, and (b) no fetch is
/// already in flight. Returns immediately; the cache is populated when
/// the response lands.
pub fn spawn_das_fetch(mint: String) {
    // Fast skip: fully resolved already (fetched=true means DAS has
    // answered, regardless of whether the answer was Some or None).
    if let Some(existing) = MINT_META.get(&mint) {
        if existing.fetched {
            return;
        }
    }
    // Dedup concurrent fetches.
    if !IN_FLIGHT.insert(mint.clone()) {
        return;
    }
    tokio::spawn(async move {
        let _drop_guard = InFlightGuard(mint.clone());
        if let Some((symbol, name, image)) = das_get_asset(&mint).await {
            record_from_das(&mint, symbol.clone(), name.clone(), image.clone());
            // Backfill historical rows: when an event ships with null
            // metadata because DAS hadn't resolved yet (the typical
            // BUY-then-SELL race where the BUY is the first sighting),
            // patch the row in-place so the user's UI updates via
            // Realtime UPDATE. Scoped to last 30 min — the active feed
            // window. Older rows aren't visible; backfilling them
            // would just churn writes.
            if symbol.is_some() || name.is_some() || image.is_some() {
                backfill_recent_rows(&mint, symbol, name, image).await;
            }
        } else {
            // Negative-cache the miss so we don't keep retrying.
            record_from_das(&mint, None, None, None);
        }
    });
}

/// PATCH surveillance_events (and the parity mirror) for any row in the
/// last 30 minutes with this `mint` and a null token_symbol. Realtime
/// fires UPDATE per affected row → the user's dashboard re-renders that
/// row with the resolved metadata.
async fn backfill_recent_rows(
    mint: &str,
    symbol: Option<String>,
    name: Option<String>,
    image: Option<String>,
) {
    let url = match std::env::var("SUPABASE_URL") {
        Ok(u) => u,
        Err(_) => return,
    };
    let key = match std::env::var("SUPABASE_SERVICE_ROLE_KEY") {
        Ok(k) => k,
        Err(_) => return,
    };
    let mut patch = serde_json::Map::new();
    if let Some(s) = symbol {
        patch.insert("token_symbol".into(), serde_json::Value::String(s));
    }
    if let Some(n) = name {
        patch.insert("token_name".into(), serde_json::Value::String(n));
    }
    if let Some(i) = image {
        patch.insert("token_image".into(), serde_json::Value::String(i));
    }
    if patch.is_empty() {
        return;
    }
    let body = serde_json::Value::Object(patch);
    let cutoff = (chrono::Utc::now() - chrono::Duration::minutes(30)).to_rfc3339();
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build();
    let client = match client {
        Ok(c) => c,
        Err(_) => return,
    };
    for table in &["surveillance_events", "carbon_surveillance_parity_events"] {
        let endpoint = format!(
            "{}/rest/v1/{}?token_address=eq.{}&token_symbol=is.null&block_time=gte.{}",
            url.trim_end_matches('/'),
            table,
            urlencoding::encode(mint),
            urlencoding::encode(&cutoff),
        );
        let _ = client
            .patch(&endpoint)
            .header("apikey", &key)
            .header("Authorization", format!("Bearer {}", key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=minimal")
            .json(&body)
            .send()
            .await;
    }
}

struct InFlightGuard(String);
impl Drop for InFlightGuard {
    fn drop(&mut self) {
        IN_FLIGHT.remove(&self.0);
    }
}

/// Single Helius DAS getAsset call. Returns `Some((symbol, name, image))`
/// on success (any of the three may be None individually), `None` on
/// network/parse failure (so the caller can negatively cache).
async fn das_get_asset(
    mint: &str,
) -> Option<(Option<String>, Option<String>, Option<String>)> {
    let api_key = std::env::var("HELIUS_API_KEY").ok()?;
    if api_key.is_empty() {
        return None;
    }
    let url = format!("https://mainnet.helius-rpc.com/?api-key={}", api_key);
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getAsset",
        "params": { "id": mint },
    });
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .ok()?;
    let resp = client.post(&url).json(&body).send().await.ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let json: serde_json::Value = resp.json().await.ok()?;
    let content = json.get("result")?.get("content")?;
    let metadata = content.get("metadata");
    let symbol = metadata
        .and_then(|m| m.get("symbol"))
        .and_then(|v| v.as_str())
        .map(String::from);
    let name = metadata
        .and_then(|m| m.get("name"))
        .and_then(|v| v.as_str())
        .map(String::from);
    let image = content
        .get("links")
        .and_then(|l| l.get("image"))
        .and_then(|v| v.as_str())
        .or_else(|| {
            content
                .get("files")
                .and_then(|f| f.as_array())
                .and_then(|arr| arr.first())
                .and_then(|f| f.get("uri"))
                .and_then(|v| v.as_str())
        })
        .map(normalize_image_url);
    Some((symbol, name, image))
}

/// Rewrite known-flaky image gateways to more reliable equivalents.
/// `ipfs.io` is notorious for timeouts and hotlink failures —
/// `cf-ipfs.com` (Cloudflare) is the same content with a CDN in front
/// and works reliably from browsers. Same `ipfs://` scheme handling
/// for completeness — DAS rarely returns it but some token URIs do.
fn normalize_image_url(raw: &str) -> String {
    let url = raw.trim();
    // ipfs.io/ipfs/<cid> → cf-ipfs.com/ipfs/<cid>
    if let Some(rest) = url
        .strip_prefix("https://ipfs.io/")
        .or_else(|| url.strip_prefix("http://ipfs.io/"))
    {
        return format!("https://cf-ipfs.com/{}", rest);
    }
    // ipfs://<cid> → https://cf-ipfs.com/ipfs/<cid>
    if let Some(rest) = url.strip_prefix("ipfs://") {
        let rest = rest.trim_start_matches("ipfs/");
        return format!("https://cf-ipfs.com/ipfs/{}", rest);
    }
    url.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn unique(suffix: &str) -> String {
        format!("Mint{}", suffix.repeat(40 / suffix.len().max(1)))
            .chars()
            .take(43)
            .collect()
    }

    #[test]
    fn record_from_create_stores_symbol_and_name() {
        let mint = unique("a");
        record_from_create(&mint, Some("FOO"), Some("Foo Token"));
        let meta = lookup(&mint).unwrap();
        assert_eq!(meta.symbol.as_deref(), Some("FOO"));
        assert_eq!(meta.name.as_deref(), Some("Foo Token"));
        assert_eq!(meta.image, None);
        assert!(!meta.fetched, "Create-only entries are not 'fetched'");
        MINT_META.remove(&mint);
    }

    #[test]
    fn record_from_das_sets_fetched_and_all_fields() {
        let mint = unique("b");
        record_from_das(
            &mint,
            Some("BAR".into()),
            Some("Bar Token".into()),
            Some("https://cdn/image.png".into()),
        );
        let meta = lookup(&mint).unwrap();
        assert_eq!(meta.symbol.as_deref(), Some("BAR"));
        assert_eq!(meta.name.as_deref(), Some("Bar Token"));
        assert_eq!(meta.image.as_deref(), Some("https://cdn/image.png"));
        assert!(meta.fetched);
        MINT_META.remove(&mint);
    }

    #[test]
    fn das_does_not_clobber_create_fields() {
        // Create captures symbol+name. DAS comes back later with image
        // but maybe a different name format. The merge keeps the first
        // non-None value per field — Create wins on symbol/name,
        // DAS fills in image.
        let mint = unique("c");
        record_from_create(&mint, Some("BAZ"), Some("Baz Token"));
        record_from_das(
            &mint,
            Some("baz".into()),
            None,
            Some("https://cdn/baz.png".into()),
        );
        let meta = lookup(&mint).unwrap();
        assert_eq!(meta.symbol.as_deref(), Some("BAZ"));
        assert_eq!(meta.name.as_deref(), Some("Baz Token"));
        assert_eq!(meta.image.as_deref(), Some("https://cdn/baz.png"));
        assert!(meta.fetched);
        MINT_META.remove(&mint);
    }

    #[test]
    fn empty_strings_are_filtered() {
        let mint = unique("d");
        record_from_create(&mint, Some(""), Some("   "));
        assert!(lookup(&mint).is_none() || lookup(&mint).unwrap().symbol.is_none());
        MINT_META.remove(&mint);
    }

    #[test]
    fn normalize_rewrites_ipfs_io_to_cloudflare() {
        // The exact failure case from the user report:
        // ipfs.io/ipfs/bafkreiamwxu6dcvvrnc7z32bjifgtjcxspejigtdt5qzf2o2cu4vb2eskq
        // returning net::ERR_FAILED in the browser. cf-ipfs.com serves
        // the same content with a CDN in front.
        assert_eq!(
            normalize_image_url(
                "https://ipfs.io/ipfs/bafkreiamwxu6dcvvrnc7z32bjifgtjcxspejigtdt5qzf2o2cu4vb2eskq"
            ),
            "https://cf-ipfs.com/ipfs/bafkreiamwxu6dcvvrnc7z32bjifgtjcxspejigtdt5qzf2o2cu4vb2eskq"
        );
    }

    #[test]
    fn normalize_rewrites_ipfs_scheme() {
        assert_eq!(
            normalize_image_url("ipfs://bafkreitest"),
            "https://cf-ipfs.com/ipfs/bafkreitest"
        );
        // ipfs://ipfs/<cid> redundancy stripped
        assert_eq!(
            normalize_image_url("ipfs://ipfs/bafkreitest"),
            "https://cf-ipfs.com/ipfs/bafkreitest"
        );
    }

    #[test]
    fn normalize_passes_through_non_ipfs() {
        assert_eq!(
            normalize_image_url("https://pump.mypinata.cloud/ipfs/abc"),
            "https://pump.mypinata.cloud/ipfs/abc"
        );
        assert_eq!(
            normalize_image_url("https://cdn.example.com/logo.png"),
            "https://cdn.example.com/logo.png"
        );
    }

    #[test]
    fn negative_cache_blocks_repeat_spawns() {
        // Simulate an entry that DAS already tried and got nothing for.
        let mint = unique("e");
        record_from_das(&mint, None, None, None);
        // A subsequent spawn should be a no-op (we can't directly observe
        // the spawn itself, but `fetched=true` is the flag the spawn
        // function checks — assert that's set).
        let meta = lookup(&mint).unwrap();
        assert!(meta.fetched);
        assert!(meta.symbol.is_none());
        MINT_META.remove(&mint);
    }
}
