//! Frankfurt Carbon Surveillance — production successor to frankfurt-shadow.
//!
//! Subscribes to Helius Atlas WebSocket (matches production
//! SURVEILLANCE_INGEST=ews) for the per-user wallet allowlist defined in
//! Supabase `surveillance_wallet_sessions`. Decodes via Carbon's full decoder
//! set. Emits per-program SurveillanceEvent rows.
//!
//! OUTPUT_MODE controls write target:
//!   - "parity":     writes to carbon_surveillance_parity_events table only.
//!                   No Redis publish. For soak-phase comparison vs
//!                   frankfurt-node's surveillance_events. (default)
//!   - "production": writes to surveillance_events + publishes to Redis
//!                   user:<userId>:events. After frankfurt-node's surveillance
//!                   role is retired, flip this to production.

mod attribution;
mod coordinator;
mod event;
mod processors;
mod state;
mod taxonomy;
mod watch_list_sync;
mod writer;
mod writer_clickhouse;

use crate::processors::{
    AggWatch, PumpSwapWatch, PumpfunWatch, SplTransferWatch, Token2022TransferWatch,
    TransferSolWatch,
};
use crate::state::{ATLAS_WS_ACCOUNTS, WatchedWallet};
use axum::{
    extract::Json, http::StatusCode, http::HeaderMap, routing::{get, post}, Router,
};
use carbon_helius_atlas_ws_datasource::{Filters, HeliusWebsocket};
use carbon_pump_swap_decoder::PumpSwapDecoder;
use carbon_pumpfun_decoder::PumpfunDecoder;
use helius::types::{
    Cluster, RpcTransactionsConfig, TransactionCommitment, TransactionDetails,
    TransactionSubscribeFilter, TransactionSubscribeOptions, UiEnhancedTransactionEncoding,
};
use serde::Deserialize;
use solana_pubkey::Pubkey;
use std::collections::HashSet;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .json()
        .with_target(false)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    log::info!(
        "frankfurt-carbon-surveillance starting; output_mode={}",
        env::var("OUTPUT_MODE").unwrap_or_else(|_| "parity".into())
    );

    // Bounded channel + writer task (Postgres + Redis publish).
    writer::spawn();

    // Optional ClickHouse writer — independent failure domain. No-op if
    // CLICKHOUSE_URL isn't set, so this can be enabled per-environment.
    writer_clickhouse::spawn();

    // Per-tx coordinator flusher: drains the candidate buffer every ~50ms,
    // collapses per-(sig, wallet, family) winners, ships final events.
    coordinator::spawn_flusher();

    // Recover watch list from surveillance_wallet_sessions on startup.
    // This is the bootstrap snapshot — NOT polling. After this, the
    // watch_list_sync task takes over via Supabase Realtime push and
    // never re-snapshots unless it has to reconnect.
    if let Err(e) = recover_watch_list().await {
        log::warn!("recover_watch_list failed (continuing with empty list): {}", e);
    }
    log::info!("initial watch list: {} wallets", state::watch_count());

    // Live watch-list sync via Supabase Realtime. INSERT/UPDATE on
    // surveillance_wallet_sessions → state::add (Helius re-subs).
    // DELETE → state::remove. No polling.
    watch_list_sync::spawn();

    // Heartbeat task — refreshes surveillance_wallet_sessions.heartbeat_at for
    // rows we own (Phase 3 reaper uses this).
    tokio::spawn(heartbeat_loop());

    // Control HTTP server (port from SURVEILLANCE_HEALTH_PORT). Endpoints:
    //   GET  /health
    //   GET  /stats
    //   POST /surveillance/wallet   (auth: bearer FRANKFURT_CONTROL_SECRET)
    //   POST /surveillance/start
    //   POST /surveillance/stop
    let port: u16 = env::var("SURVEILLANCE_HEALTH_PORT")
        .unwrap_or_else(|_| "9099".into())
        .parse()
        .unwrap_or(9099);
    tokio::spawn(run_http_server(port));

    // Carbon Pipeline.
    let api_key = env::var("HELIUS_API_KEY").expect("HELIUS_API_KEY not set");
    let dynamic_set: Arc<RwLock<HashSet<Pubkey>>> = Arc::clone(&ATLAS_WS_ACCOUNTS);
    // Helius EWS requires accountInclude at subscribe time. Carbon's atlas-ws
    // example reads the dynamic set to mutate later, but the INITIAL subscribe
    // must include a non-empty list. Snapshot the recovered watch list now.
    let initial_accounts: Vec<String> = {
        let set = dynamic_set.read().await;
        set.iter().map(|pk| pk.to_string()).collect()
    };
    log::info!(
        "atlas-ws initial account_include count = {}",
        initial_accounts.len()
    );
    let helius_ws = HeliusWebsocket::new(
        api_key,
        Filters {
            accounts: vec![],
            transactions: Some(RpcTransactionsConfig {
                filter: TransactionSubscribeFilter {
                    account_include: if initial_accounts.is_empty() {
                        None
                    } else {
                        Some(initial_accounts)
                    },
                    account_exclude: None,
                    account_required: None,
                    vote: Some(false),
                    failed: Some(false),
                    signature: None,
                },
                options: TransactionSubscribeOptions {
                    commitment: Some(TransactionCommitment::Confirmed),
                    encoding: Some(UiEnhancedTransactionEncoding::Base64),
                    transaction_details: Some(TransactionDetails::Full),
                    show_rewards: None,
                    max_supported_transaction_version: Some(0),
                },
            }),
        },
        dynamic_set,
        Cluster::MainnetBeta,
    )
    .with_ping_interval_secs(10)
    .with_pong_timeout_secs(10)
    .with_transaction_idle_timeout_secs(60);

    log::info!("connecting to Helius Atlas WS");

    use carbon_core::pipeline::Pipeline;
    Pipeline::builder()
        .datasource(helius_ws)
        .instruction(PumpfunDecoder, PumpfunWatch)
        .instruction(PumpSwapDecoder, PumpSwapWatch)
        // Aggregator/router/DEX programs we care about — emit on match.
        .instruction(carbon_dflow_aggregator_v4_decoder::SwapOrchestratorDecoder, AggWatch::<carbon_dflow_aggregator_v4_decoder::instructions::SwapOrchestratorInstruction>::new("dflow", true))
        .instruction(carbon_jupiter_swap_decoder::JupiterSwapDecoder, AggWatch::<carbon_jupiter_swap_decoder::instructions::JupiterSwapInstruction>::new("jupiter_swap", true))
        .instruction(carbon_meteora_damm_v2_decoder::MeteoraDammV2Decoder, AggWatch::<carbon_meteora_damm_v2_decoder::instructions::MeteoraDammV2Instruction>::new("meteora_damm_v2", true))
        .instruction(carbon_meteora_dbc_decoder::DynamicBondingCurveDecoder, AggWatch::<carbon_meteora_dbc_decoder::instructions::DynamicBondingCurveInstruction>::new("meteora_dbc", true))
        .instruction(carbon_meteora_dlmm_decoder::MeteoraDlmmDecoder, AggWatch::<carbon_meteora_dlmm_decoder::instructions::MeteoraDlmmInstruction>::new("meteora_dlmm", true))
        .instruction(carbon_meteora_pools_decoder::MeteoraPoolsDecoder, AggWatch::<carbon_meteora_pools_decoder::instructions::MeteoraPoolsProgramInstruction>::new("meteora_pools", true))
        .instruction(carbon_raydium_amm_v4_decoder::RaydiumAmmV4Decoder, AggWatch::<carbon_raydium_amm_v4_decoder::instructions::RaydiumAmmV4Instruction>::new("raydium_amm_v4", true))
        .instruction(carbon_raydium_clmm_decoder::RaydiumClmmDecoder, AggWatch::<carbon_raydium_clmm_decoder::instructions::RaydiumClmmInstruction>::new("raydium_clmm", true))
        .instruction(carbon_raydium_cpmm_decoder::RaydiumCpmmDecoder, AggWatch::<carbon_raydium_cpmm_decoder::instructions::RaydiumCpmmInstruction>::new("raydium_cpmm", true))
        .instruction(carbon_raydium_launchpad_decoder::RaydiumLaunchpadDecoder, AggWatch::<carbon_raydium_launchpad_decoder::instructions::RaydiumLaunchpadInstruction>::new("raydium_launchpad", true))
        .instruction(carbon_raydium_stable_swap_decoder::RaydiumStableSwapAmmDecoder, AggWatch::<carbon_raydium_stable_swap_decoder::instructions::RaydiumStableSwapAmmInstruction>::new("raydium_stable_swap", true))
        .instruction(carbon_orca_whirlpool_decoder::OrcaWhirlpoolDecoder, AggWatch::<carbon_orca_whirlpool_decoder::instructions::WhirlpoolInstruction>::new("orca_whirlpool", true))
        .instruction(carbon_lifinity_amm_v2_decoder::LifinityAmmV2Decoder, AggWatch::<carbon_lifinity_amm_v2_decoder::instructions::LifinityAmmV2Instruction>::new("lifinity_amm_v2", true))
        .instruction(carbon_phoenix_v1_decoder::PhoenixDecoder, AggWatch::<carbon_phoenix_v1_decoder::instructions::PhoenixInstruction>::new("phoenix_v1", true))
        .instruction(carbon_openbook_v2_decoder::OpenbookV2Decoder, AggWatch::<carbon_openbook_v2_decoder::instructions::OpenbookV2Instruction>::new("openbook_v2", true))
        .instruction(carbon_fluxbeam_decoder::FluxbeamDecoder, AggWatch::<carbon_fluxbeam_decoder::instructions::FluxbeamInstruction>::new("fluxbeam", true))
        .instruction(carbon_bonkswap_decoder::BonkswapDecoder, AggWatch::<carbon_bonkswap_decoder::instructions::BonkswapInstruction>::new("bonkswap", true))
        .instruction(carbon_boop_decoder::BoopDecoder, AggWatch::<carbon_boop_decoder::instructions::BoopInstruction>::new("boop", true))
        .instruction(carbon_heaven_decoder::HeavenDecoder, AggWatch::<carbon_heaven_decoder::instructions::HeavenInstruction>::new("heaven", true))
        .instruction(carbon_moonshot_decoder::MoonshotDecoder, AggWatch::<carbon_moonshot_decoder::instructions::MoonshotInstruction>::new("moonshot", true))
        .instruction(carbon_vertigo_decoder::VertigoDecoder, AggWatch::<carbon_vertigo_decoder::instructions::VertigoInstruction>::new("vertigo", true))
        .instruction(carbon_stabble_stable_swap_decoder::StableSwapDecoder, AggWatch::<carbon_stabble_stable_swap_decoder::instructions::StableSwapInstruction>::new("stabble_stable_swap", true))
        .instruction(carbon_stabble_weighted_swap_decoder::WeightedSwapDecoder, AggWatch::<carbon_stabble_weighted_swap_decoder::instructions::WeightedSwapInstruction>::new("stabble_weighted_swap", true))
        .instruction(carbon_pancake_swap_decoder::PancakeSwapDecoder, AggWatch::<carbon_pancake_swap_decoder::instructions::AmmV3Instruction>::new("pancake_swap", true))
        // Additional DEX/router programs we depend on but hadn't wired —
        // surfaced when sigs slipped past the parity diff and `proVF4…`
        // (Onchain Labs DEX V2) wasn't decoded. Same coverage for siblings.
        .instruction(carbon_onchain_labs_dex_v1_decoder::OnchainLabsDexV1Decoder, AggWatch::<carbon_onchain_labs_dex_v1_decoder::instructions::OnchainLabsDexV1Instruction>::new("onchain_labs_dex_v1", true))
        .instruction(carbon_onchain_labs_dex_v2_decoder::OnchainLabsDexV2Decoder, AggWatch::<carbon_onchain_labs_dex_v2_decoder::instructions::OnchainLabsDexV2Instruction>::new("onchain_labs_dex_v2", true))
        .instruction(carbon_okx_dex_decoder::OkxDexDecoder, AggWatch::<carbon_okx_dex_decoder::instructions::OkxDexInstruction>::new("okx_dex", true))
        .instruction(carbon_gavel_decoder::GavelDecoder, AggWatch::<carbon_gavel_decoder::instructions::GavelInstruction>::new("gavel", true))
        .instruction(carbon_wavebreak_decoder::WavebreakDecoder, AggWatch::<carbon_wavebreak_decoder::instructions::WavebreakInstruction>::new("wavebreak", true))
        // System + token programs: dedicated transfer processors emit
        // transfer_in / transfer_out events. Mirrors frankfurt-node's transfer
        // surveillance.
        .instruction(carbon_system_program_decoder::SystemProgramDecoder, TransferSolWatch)
        .instruction(carbon_token_program_decoder::TokenProgramDecoder, SplTransferWatch::new("token_program"))
        .instruction(carbon_token_2022_decoder::Token2022Decoder, Token2022TransferWatch::new("token_2022"))
        // Other infrastructure programs — register so noise doesn't fall to a
        // default decoder, but emit=false so we don't surface them.
        .instruction(carbon_associated_token_account_decoder::SplAssociatedTokenAccountDecoder, AggWatch::<carbon_associated_token_account_decoder::instructions::SplAssociatedTokenAccountInstruction>::new("ata", false))
        .instruction(carbon_memo_program_decoder::MemoProgramDecoder, AggWatch::<carbon_memo_program_decoder::instructions::MemoProgramInstruction>::new("memo", false))
        .instruction(carbon_address_lookup_table_decoder::AddressLookupTableDecoder, AggWatch::<carbon_address_lookup_table_decoder::instructions::AddressLookupTableInstruction>::new("alt", false))
        .build()?
        .run()
        .await?;
    Ok(())
}

// ─── Watch list recovery ─────────────────────────────────────────────────

async fn recover_watch_list() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let supabase_url = env::var("SUPABASE_URL")?;
    let supabase_key = env::var("SUPABASE_SERVICE_ROLE_KEY")?;
    // RECOVERY_SOURCE_SERVER_ID = where we READ the watch list from on startup.
    // During soak, this is `frankfurt-prod` (frankfurt-node's claim) so we
    // surveil the same wallets without competing for ownership. After cutover
    // it can be set to the same value as SESSION_PRIMITIVE_SERVER_ID.
    // This is INTENTIONALLY DIFFERENT from SESSION_PRIMITIVE_SERVER_ID, which
    // is our own claim ID for heartbeat writes.
    let recovery_source = env::var("RECOVERY_SOURCE_SERVER_ID")
        .unwrap_or_else(|_| "frankfurt-prod".into());

    let url = format!(
        "{}/rest/v1/surveillance_wallet_sessions?server_id=eq.{}&status=eq.running&select=user_id,target_id,wallet_address,wallet_label,surveillance_targets(name)",
        supabase_url, recovery_source
    );
    let rows: Vec<SessionRow> = reqwest::Client::new()
        .get(&url)
        .header("apikey", &supabase_key)
        .header("Authorization", format!("Bearer {}", supabase_key))
        .send()
        .await?
        .json()
        .await?;
    let mut count = 0;
    for r in rows {
        let target_name = r
            .surveillance_targets
            .and_then(|t| t.name)
            .unwrap_or_else(|| "Target".into());
        state::add(
            r.wallet_address.clone(),
            WatchedWallet {
                user_id: r.user_id,
                target_id: r.target_id,
                target_name,
                wallet_label: r.wallet_label.unwrap_or_default(),
            },
        )
        .await;
        count += 1;
    }
    log::info!("recovered {} watched wallets", count);
    Ok(())
}

#[derive(Deserialize)]
struct SessionRow {
    user_id: String,
    target_id: String,
    wallet_address: String,
    wallet_label: Option<String>,
    surveillance_targets: Option<TargetEmbedded>,
}

#[derive(Deserialize)]
struct TargetEmbedded {
    name: Option<String>,
}

// ─── Heartbeat ───────────────────────────────────────────────────────────

async fn heartbeat_loop() {
    let supabase_url = match env::var("SUPABASE_URL") {
        Ok(u) => u,
        Err(_) => return,
    };
    let supabase_key = match env::var("SUPABASE_SERVICE_ROLE_KEY") {
        Ok(k) => k,
        Err(_) => return,
    };
    // Heartbeat refreshes rows we OWN — claimed under our own server_id, not
    // the recovery source. During soak we typically own zero rows, so this is
    // a no-op. After cutover, the backend inserts new sessions with our
    // server_id and this loop keeps them alive for the Phase 3 reaper.
    let server_id = env::var("SESSION_PRIMITIVE_SERVER_ID")
        .unwrap_or_else(|_| "frankfurt-carbon-surveillance".into());
    let http = reqwest::Client::new();
    loop {
        tokio::time::sleep(Duration::from_secs(30)).await;
        let url = format!(
            "{}/rest/v1/surveillance_wallet_sessions?server_id=eq.{}&status=eq.running",
            supabase_url, server_id
        );
        let _ = http
            .patch(&url)
            .header("apikey", &supabase_key)
            .header("Authorization", format!("Bearer {}", supabase_key))
            .header("Content-Type", "application/json")
            .header("Prefer", "return=minimal")
            .body(r#"{"heartbeat_at":"now()"}"#.to_string())
            .send()
            .await;
    }
}

// ─── HTTP control + health ───────────────────────────────────────────────

async fn run_http_server(port: u16) {
    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/stats", get(stats_handler))
        .route("/surveillance/wallet", post(handle_wallet))
        .route("/surveillance/start", post(handle_start))
        .route("/surveillance/stop", post(handle_stop));

    let bind = format!("127.0.0.1:{}", port);
    match tokio::net::TcpListener::bind(&bind).await {
        Ok(l) => {
            log::info!("control + health listening on {}", bind);
            let _ = axum::serve(l, app).await;
        }
        Err(e) => log::error!("failed to bind {}: {}", bind, e),
    }
}

async fn stats_handler() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "watch_wallets": state::watch_count(),
        "output_mode": env::var("OUTPUT_MODE").unwrap_or_else(|_| "parity".into()),
        "server_id": env::var("SESSION_PRIMITIVE_SERVER_ID")
            .unwrap_or_else(|_| "frankfurt-carbon-surveillance".into()),
        "recovery_source": env::var("RECOVERY_SOURCE_SERVER_ID")
            .unwrap_or_else(|_| "frankfurt-prod".into()),
    }))
}

fn check_bearer(headers: &HeaderMap) -> bool {
    let expected = match env::var("FRANKFURT_CONTROL_SECRET") {
        Ok(s) if !s.is_empty() => s,
        _ => return false, // refuse if not configured
    };
    let header = match headers.get("authorization").and_then(|h| h.to_str().ok()) {
        Some(h) => h,
        None => return false,
    };
    header.strip_prefix("Bearer ").map(|t| t == expected).unwrap_or(false)
}

#[derive(Deserialize)]
struct WalletReq {
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "targetId")]
    target_id: String,
    action: String,
    address: String,
    label: Option<String>,
}

async fn handle_wallet(headers: HeaderMap, Json(req): Json<WalletReq>) -> StatusCode {
    if !check_bearer(&headers) {
        return StatusCode::UNAUTHORIZED;
    }
    let label = req.label.unwrap_or_else(|| req.address.chars().take(8).collect());
    match req.action.as_str() {
        "add" => {
            let target_name = lookup_target_name(&req.target_id).await;
            state::add(
                req.address.clone(),
                WatchedWallet {
                    user_id: req.user_id,
                    target_id: req.target_id,
                    target_name,
                    wallet_label: label,
                },
            )
            .await;
            StatusCode::OK
        }
        "remove" => {
            state::remove(&req.address).await;
            StatusCode::OK
        }
        _ => StatusCode::BAD_REQUEST,
    }
}

#[derive(Deserialize)]
struct StartReq {
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "targetId")]
    target_id: String,
    #[serde(rename = "targetName", default)]
    target_name: Option<String>,
    wallets: Vec<WalletEntry>,
}

#[derive(Deserialize)]
struct WalletEntry {
    address: String,
    label: Option<String>,
}

async fn handle_start(headers: HeaderMap, Json(req): Json<StartReq>) -> StatusCode {
    if !check_bearer(&headers) {
        return StatusCode::UNAUTHORIZED;
    }
    let target_name = match req.target_name {
        Some(n) if !n.is_empty() => n,
        _ => lookup_target_name(&req.target_id).await,
    };
    for w in req.wallets {
        let label = w
            .label
            .unwrap_or_else(|| w.address.chars().take(8).collect());
        state::add(
            w.address,
            WatchedWallet {
                user_id: req.user_id.clone(),
                target_id: req.target_id.clone(),
                target_name: target_name.clone(),
                wallet_label: label,
            },
        )
        .await;
    }
    StatusCode::OK
}

#[derive(Deserialize)]
struct StopReq {
    #[serde(rename = "userId", default)]
    _user_id: String,
    #[serde(rename = "targetId")]
    target_id: String,
}

async fn handle_stop(headers: HeaderMap, Json(req): Json<StopReq>) -> StatusCode {
    if !check_bearer(&headers) {
        return StatusCode::UNAUTHORIZED;
    }
    let removed = state::remove_target(&req.target_id).await;
    log::info!("stop target {} removed {} wallets", req.target_id, removed.len());
    StatusCode::OK
}

async fn lookup_target_name(target_id: &str) -> String {
    let url = match env::var("SUPABASE_URL") {
        Ok(u) => u,
        Err(_) => return target_id.to_string(),
    };
    let key = match env::var("SUPABASE_SERVICE_ROLE_KEY") {
        Ok(k) => k,
        Err(_) => return target_id.to_string(),
    };
    let q = format!(
        "{}/rest/v1/surveillance_targets?id=eq.{}&select=name",
        url, target_id
    );
    let rows: Vec<serde_json::Value> = match reqwest::Client::new()
        .get(&q)
        .header("apikey", &key)
        .header("Authorization", format!("Bearer {}", key))
        .send()
        .await
    {
        Ok(r) => r.json().await.unwrap_or_default(),
        Err(_) => return target_id.to_string(),
    };
    rows.first()
        .and_then(|v| v.get("name"))
        .and_then(|n| n.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| target_id.to_string())
}
