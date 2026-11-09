use reqwest::Client;

use crate::jetstreamer_firehose::{
    SharedError, archive,
    epochs::{epoch_to_slot_range, latest_epoch_at},
};

/// First retry delay when an epoch check fails; it doubles on each retry.
const EPOCH_CHECK_RETRY_BASE: std::time::Duration = std::time::Duration::from_secs(1);

/// Creates a new HTTP client with the Jetstreamer user agent header.
///
/// The user agent is set to `jetstreamer/v{version}` where version comes from
/// the crate version. If the `JETSTREAMER_VERSION` environment variable is set
/// at build time (e.g., via CI/CD), it will be used instead, allowing git
/// commit information to be included (e.g., `jetstreamer/v0.3.0+24d5efe-dirty`).
///
/// HTTP/1.1 is forced because the firehose runs hundreds of concurrent
/// long-lived streams; HTTP/2 multiplexing on a few shared TCP connections
/// suffers from per-stream flow-control and head-of-line blocking under that
/// load profile, leading to `read_until_block` timeouts.
pub fn create_http_client() -> Client {
    let version = option_env!("JETSTREAMER_VERSION").unwrap_or(env!("CARGO_PKG_VERSION"));
    let user_agent = format!("jetstreamer/v{}", version);
    Client::builder()
        .user_agent(user_agent)
        .http1_only()
        .build()
        .expect("failed to create HTTP client")
}

/// Queries the current epoch from mainnet using the Solana RPC API.
pub async fn current_epoch(client: &Client) -> Result<u64, SharedError> {
    let url = "https://api.mainnet-beta.solana.com";
    let request_body = r#"{"jsonrpc":"2.0","id":1,"method":"getEpochInfo","params":[]}"#;
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()
        .await?
        .error_for_status()?;
    let text = response.text().await?;
    let epoch_info: serde_json::Value = serde_json::from_str(&text)?;
    epoch_info["result"]["epoch"]
        .as_u64()
        .ok_or_else(|| "getEpochInfo response has no result.epoch".into())
}

/// Finds the most recent epoch with a compact archive hosted on Old Faithful.
///
/// If `epoch` is `None`, the search starts from [`current_epoch`]. The returned
/// tuple is `(epoch, first_slot, last_slot)`. Only a 404 moves the search to the previous
/// epoch; a failed check (for example HTTP 429) is retried, then returned as an error.
pub async fn latest_old_faithful_epoch(
    client: &Client,
    epoch: Option<u64>,
) -> Result<(u64, u64, u64), SharedError> {
    let start = if let Some(epoch) = epoch {
        epoch
    } else {
        current_epoch(client).await?
    };
    let epoch = latest_epoch_at(
        start,
        client,
        archive::car_location(),
        EPOCH_CHECK_RETRY_BASE,
    )
    .await?;
    let (start_slot, end_slot) = epoch_to_slot_range(epoch);
    Ok((epoch, start_slot, end_slot))
}
