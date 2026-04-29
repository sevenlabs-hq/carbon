//! Bounded mpsc → batched JSONEachRow inserts into ClickHouse Cloud.
//!
//! Independent failure domain from `writer.rs` (Postgres + Redis):
//!   - separate channel, separate background task
//!   - if CH is slow/unreachable: events queue then drop on full,
//!     Postgres path is unaffected
//!   - if Postgres is slow: same in reverse
//!
//! Schema target: `trailblaze_surveillance.events_raw` — populated for every
//! emission. The 4 materialized views in CH fan to specialized typed tables
//! (`pumpfun_trades`, `swap_transactions`, `transfers`, `program_activity`,
//! `token_creations`) automatically; we never write to those directly.
//!
//! Env vars (no-op if `CLICKHOUSE_URL` is unset — feature is opt-in):
//!   - CLICKHOUSE_URL          (e.g. https://x2o…clickhouse.cloud:8443)
//!   - CLICKHOUSE_USER         (default: "default")
//!   - CLICKHOUSE_PASSWORD
//!   - CLICKHOUSE_DATABASE     (default: "trailblaze_surveillance")
//!   - CLICKHOUSE_TABLE        (default: "events_raw")

use crate::event::SurveillanceEventOut;
use crate::taxonomy::Activity;
use once_cell::sync::OnceCell;
use serde::Serialize;
use std::env;
use std::time::Duration;
use tokio::sync::mpsc;

const CHANNEL_CAPACITY: usize = 5000;
const FLUSH_INTERVAL_MS: u64 = 1000;
const FLUSH_BATCH_SIZE: usize = 1000;

static SENDER: OnceCell<mpsc::Sender<SurveillanceEventOut>> = OnceCell::new();

pub async fn send_event(event: SurveillanceEventOut) {
    if let Some(tx) = SENDER.get() {
        if let Err(e) = tx.try_send(event) {
            log::warn!("clickhouse channel full or closed: {}", e);
        }
    }
}

pub fn spawn() {
    if env::var("CLICKHOUSE_URL").is_err() {
        log::info!(
            "clickhouse writer disabled (CLICKHOUSE_URL not set); events stay in Postgres only"
        );
        return;
    }
    let (tx, rx) = mpsc::channel(CHANNEL_CAPACITY);
    SENDER.set(tx).ok();
    tokio::spawn(async move { writer_loop(rx).await });
}

async fn writer_loop(mut rx: mpsc::Receiver<SurveillanceEventOut>) {
    let url = env::var("CLICKHOUSE_URL").expect("CLICKHOUSE_URL");
    let user = env::var("CLICKHOUSE_USER").unwrap_or_else(|_| "default".into());
    let pass = env::var("CLICKHOUSE_PASSWORD").unwrap_or_default();
    let db = env::var("CLICKHOUSE_DATABASE")
        .unwrap_or_else(|_| "trailblaze_surveillance".into());
    let table = env::var("CLICKHOUSE_TABLE").unwrap_or_else(|_| "events_raw".into());
    let insert_url = format!(
        "{}/?database={}&query=INSERT+INTO+{}+FORMAT+JSONEachRow",
        url.trim_end_matches('/'),
        urlencoding::encode(&db),
        urlencoding::encode(&table),
    );
    log::info!("clickhouse writer starting → {} (db={}, table={})", url, db, table);

    let http = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("reqwest client");

    let mut batch: Vec<SurveillanceEventOut> = Vec::with_capacity(FLUSH_BATCH_SIZE);
    let flush_interval = Duration::from_millis(FLUSH_INTERVAL_MS);

    loop {
        let collect_deadline = tokio::time::sleep(flush_interval);
        tokio::pin!(collect_deadline);
        loop {
            tokio::select! {
                biased;
                maybe_event = rx.recv() => match maybe_event {
                    Some(event) => {
                        batch.push(event);
                        if batch.len() >= FLUSH_BATCH_SIZE { break; }
                    }
                    None => return,
                },
                _ = &mut collect_deadline => break,
            }
        }
        if batch.is_empty() {
            continue;
        }

        // Build newline-delimited JSON body
        let mut body = String::with_capacity(batch.len() * 512);
        for e in &batch {
            let row = ChRow::from_event(e);
            match serde_json::to_string(&row) {
                Ok(s) => {
                    body.push_str(&s);
                    body.push('\n');
                }
                Err(err) => log::warn!("clickhouse serialize: {}", err),
            }
        }

        match http
            .post(&insert_url)
            .basic_auth(&user, Some(&pass))
            .header("Content-Type", "application/x-ndjson")
            .body(body)
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {}
            Ok(resp) => {
                crate::metrics::inc_clickhouse_failures();
                let status = resp.status();
                let body_resp = resp.text().await.unwrap_or_default();
                log::warn!(
                    "clickhouse insert non-2xx ({}): {}",
                    status,
                    &body_resp[..body_resp.len().min(300)]
                );
            }
            Err(e) => {
                crate::metrics::inc_clickhouse_failures();
                log::warn!("clickhouse insert error: {}", e);
            }
        }

        batch.clear();
    }
}

/// JSONEachRow row matching the `events_raw` schema. Fields the
/// `SurveillanceEventOut` struct doesn't carry directly (`fee_payer`,
/// `activity_family`, `stack_height`, `ix_variant`) are derived here so
/// no upstream struct change is required for v1 — every processor's
/// existing `raw_data` JSON already carries `fee_payer` and `ix_variant`,
/// and the family follows from the event_type.
#[derive(Serialize)]
struct ChRow<'a> {
    slot: u64,
    signature: &'a str,
    block_time: &'a str,
    program: &'a str,
    event_type: &'a str,
    activity_family: &'static str,
    ix_variant: Option<String>,
    stack_height: u32,
    fee_payer: String,
    wallet_address: &'a str,
    counterparty: &'a str,
    user_id: &'a str,
    target_id: &'a str,
    target_name: &'a str,
    wallet_label: &'a str,
    token_address: Option<&'a str>,
    token_symbol: Option<&'a str>,
    sol_amount: f64,
    token_amount: f64,
    price_sol: f64,
    raw_data: String,
}

impl<'a> ChRow<'a> {
    fn from_event(e: &'a SurveillanceEventOut) -> Self {
        let raw_obj = e.raw_data.as_object();
        let fee_payer = raw_obj
            .and_then(|o| o.get("fee_payer").and_then(|v| v.as_str()))
            .unwrap_or(&e.wallet_address)
            .to_string();
        let ix_variant = raw_obj
            .and_then(|o| o.get("ix_variant").and_then(|v| v.as_str()))
            .map(|s| s.to_string());
        ChRow {
            slot: e.slot,
            signature: &e.signature,
            block_time: &e.block_time,
            program: e.program,
            event_type: e.event_type,
            activity_family: family_for(e.event_type),
            ix_variant,
            stack_height: 1, // outer-only after filters in TransferWatch / AggWatch
            fee_payer,
            wallet_address: &e.wallet_address,
            counterparty: &e.counterparty,
            user_id: &e.user_id,
            target_id: &e.target_id,
            target_name: &e.target_name,
            wallet_label: &e.wallet_label,
            token_address: e.token_address.as_deref(),
            token_symbol: e.token_symbol.as_deref(),
            sol_amount: e.sol_amount,
            token_amount: e.token_amount,
            price_sol: e.price_sol,
            raw_data: e.raw_data.to_string(),
        }
    }
}

fn family_for(event_type: &str) -> &'static str {
    match event_type {
        "swap_buy" | "swap_sell" => "Swap",
        "mint_create" => "Mint",
        "liquidity_add" | "liquidity_remove" => "Liquidity",
        "limit_order_create" | "limit_order_fill" => "LimitOrder",
        "vault_deposit" | "vault_withdraw" => "Vault",
        "stake" | "unstake" => "Staking",
        "dca_create" | "dca_close" => "Dca",
        "nft_mint" | "nft_sale" => "Nft",
        "transfer_in" | "transfer_out" => "Transfer",
        "program_activity" => "Fallback",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::SurveillanceEventOut;

    fn ev(event_type: &'static str, raw: serde_json::Value) -> SurveillanceEventOut {
        SurveillanceEventOut {
            user_id: "u".into(),
            target_id: "t".into(),
            target_name: "n".into(),
            wallet_address: "WALLET_FALLBACK".into(),
            wallet_label: "".into(),
            signature: "sig".into(),
            event_type,
            token_address: None,
            token_symbol: None,
            token_name: None,
            token_image: None,
            sol_amount: 0.0,
            token_amount: 0.0,
            price_sol: 0.0,
            program: "pumpfun",
            counterparty: "".into(),
            block_time: "2026-04-29T00:00:00Z".into(),
            slot: 1,
            raw_data: raw,
        }
    }

    #[test]
    fn family_mapping_covers_all_activity_variants() {
        // Every Activity::as_event_type() value should map to a family —
        // catch typos when new activities are added.
        let activities = [
            Activity::SwapBuy,
            Activity::SwapSell,
            Activity::MintCreate,
            Activity::LiquidityAdd,
            Activity::LiquidityRemove,
            Activity::LimitOrderCreate,
            Activity::LimitOrderFill,
            Activity::VaultDeposit,
            Activity::VaultWithdraw,
            Activity::Stake,
            Activity::Unstake,
            Activity::DcaCreate,
            Activity::DcaClose,
            Activity::NftMint,
            Activity::NftSale,
            Activity::TransferIn,
            Activity::TransferOut,
            Activity::ProgramActivity,
        ];
        for a in activities {
            assert_ne!(family_for(a.as_event_type()), "Unknown",
                "activity {:?} maps to Unknown — add it to family_for()", a);
        }
    }

    #[test]
    fn fee_payer_extracted_from_raw_data_when_present() {
        let e = ev(
            "swap_buy",
            serde_json::json!({"fee_payer": "ROUTER_BOT_WALLET"}),
        );
        let row = ChRow::from_event(&e);
        assert_eq!(row.fee_payer, "ROUTER_BOT_WALLET");
    }

    #[test]
    fn fee_payer_falls_back_to_wallet_address_when_absent() {
        let e = ev("swap_buy", serde_json::json!({}));
        let row = ChRow::from_event(&e);
        assert_eq!(row.fee_payer, "WALLET_FALLBACK");
    }

    #[test]
    fn ix_variant_extracted_from_raw_data() {
        let e = ev(
            "swap_sell",
            serde_json::json!({"ix_variant": "CpiEvent::TradeEvent"}),
        );
        let row = ChRow::from_event(&e);
        assert_eq!(row.ix_variant.as_deref(), Some("CpiEvent::TradeEvent"));
    }

    #[test]
    fn family_for_swap_buy_is_swap() {
        assert_eq!(family_for("swap_buy"), "Swap");
        assert_eq!(family_for("transfer_in"), "Transfer");
        assert_eq!(family_for("program_activity"), "Fallback");
    }
}
