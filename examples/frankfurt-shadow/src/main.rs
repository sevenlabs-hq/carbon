// Frankfurt Carbon Shadow — standalone surveillance pipeline.
//
// Runs as its own process (systemd unit `frankfurt-shadow.service`) parallel to
// the production `frankfurt-rust.service`. NO shared state, NO shared keypairs,
// NO shared ports with production. Reads SHRED_GRPC_URL from its own env file
// (typically the same local Jito shredstream-proxy at 127.0.0.1:9999), sets up
// a Carbon Pipeline, registers PumpFun/PumpSwap/aggregator decoders, and logs
// WATCH_BUY/WATCH_SELL/WATCH_AGG lines whenever a wallet from CARBON_WATCH_WALLETS
// trades.
//
// Pure observation — never calls executor, dflow, sell, outbox. Production
// process keeps doing real execution; this one just watches and reports.
//
// Health endpoint at $SHADOW_HEALTH_PORT (default 3004) for liveness checks:
//   curl http://127.0.0.1:3004/health   -> 200 "ok"
//   curl http://127.0.0.1:3004/stats    -> JSON counters

use {
    async_trait::async_trait,
    axum::{routing::get, Json, Router},
    carbon_core::{
        error::CarbonResult,
        instruction::InstructionProcessorInputType,
        metrics::MetricsCollection,
        pipeline::Pipeline,
        processor::Processor,
    },
    carbon_jito_shredstream_grpc_datasource::JitoShredstreamGrpcClient,
    carbon_pumpfun_decoder::{
        instructions::{CpiEvent, PumpfunInstruction},
        PumpfunDecoder,
    },
    carbon_pump_swap_decoder::{instructions::PumpSwapInstruction, PumpSwapDecoder},
    once_cell::sync::Lazy,
    serde_json::json,
    std::{
        collections::HashSet,
        marker::PhantomData,
        sync::{
            atomic::{AtomicU64, Ordering},
            Arc,
        },
    },
};

static WATCH: Lazy<HashSet<String>> = Lazy::new(|| {
    std::env::var("CARBON_WATCH_WALLETS")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
});

static COUNT_PF_BUY: AtomicU64 = AtomicU64::new(0);
static COUNT_PF_SELL: AtomicU64 = AtomicU64::new(0);
static COUNT_PF_TRADE_EVT: AtomicU64 = AtomicU64::new(0);
static COUNT_PS_BUY: AtomicU64 = AtomicU64::new(0);
static COUNT_PS_SELL: AtomicU64 = AtomicU64::new(0);
static COUNT_AGG: AtomicU64 = AtomicU64::new(0);

fn watched(signer: &str) -> bool {
    WATCH.contains(signer)
}

// ─── PumpFun watch processor ──────────────────────────────────────────

struct PumpfunWatch;

#[async_trait]
impl Processor for PumpfunWatch {
    type InputType = InstructionProcessorInputType<PumpfunInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, decoded, _nested, _raw) = data;
        let signer = metadata.transaction_metadata.fee_payer.to_string();
        if !watched(&signer) {
            return Ok(());
        }
        let signature = metadata.transaction_metadata.signature.to_string();
        let slot = metadata.transaction_metadata.slot;

        match decoded.data {
            PumpfunInstruction::Buy(_) | PumpfunInstruction::BuyExactSolIn(_) => {
                COUNT_PF_BUY.fetch_add(1, Ordering::Relaxed);
                let mint = decoded
                    .accounts
                    .get(2)
                    .map(|a| a.pubkey.to_string())
                    .unwrap_or_default();
                log::info!(
                    "WATCH_BUY signer={} program=pumpfun mint={} slot={} sig={}",
                    signer, mint, slot, signature
                );
            }
            PumpfunInstruction::Sell(_) => {
                COUNT_PF_SELL.fetch_add(1, Ordering::Relaxed);
                let mint = decoded
                    .accounts
                    .get(2)
                    .map(|a| a.pubkey.to_string())
                    .unwrap_or_default();
                log::info!(
                    "WATCH_SELL signer={} program=pumpfun mint={} slot={} sig={}",
                    signer, mint, slot, signature
                );
            }
            PumpfunInstruction::CpiEvent(boxed) => {
                if let CpiEvent::TradeEvent(ev) = *boxed {
                    COUNT_PF_TRADE_EVT.fetch_add(1, Ordering::Relaxed);
                    log::info!(
                        "WATCH_TRADE_EVT signer={} program=pumpfun mint={} is_buy={} sol={} ix={} slot={} sig={}",
                        signer, ev.mint, ev.is_buy, ev.sol_amount, ev.ix_name, slot, signature
                    );
                }
            }
            _ => {}
        }
        Ok(())
    }
}

// ─── PumpSwap watch processor ─────────────────────────────────────────

struct PumpSwapWatch;

#[async_trait]
impl Processor for PumpSwapWatch {
    type InputType = InstructionProcessorInputType<PumpSwapInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, decoded, _nested, _raw) = data;
        let signer = metadata.transaction_metadata.fee_payer.to_string();
        if !watched(&signer) {
            return Ok(());
        }
        let signature = metadata.transaction_metadata.signature.to_string();
        let slot = metadata.transaction_metadata.slot;

        match decoded.data {
            PumpSwapInstruction::Buy(_) | PumpSwapInstruction::BuyExactQuoteIn(_) => {
                COUNT_PS_BUY.fetch_add(1, Ordering::Relaxed);
                let mint = decoded
                    .accounts
                    .get(3)
                    .map(|a| a.pubkey.to_string())
                    .unwrap_or_default();
                log::info!(
                    "WATCH_BUY signer={} program=pumpswap mint={} slot={} sig={}",
                    signer, mint, slot, signature
                );
            }
            PumpSwapInstruction::Sell(_) => {
                COUNT_PS_SELL.fetch_add(1, Ordering::Relaxed);
                let mint = decoded
                    .accounts
                    .get(3)
                    .map(|a| a.pubkey.to_string())
                    .unwrap_or_default();
                log::info!(
                    "WATCH_SELL signer={} program=pumpswap mint={} slot={} sig={}",
                    signer, mint, slot, signature
                );
            }
            _ => {}
        }
        Ok(())
    }
}

// ─── Aggregator surveillance processor ────────────────────────────────

struct AggWatch<T> {
    program: &'static str,
    _m: PhantomData<T>,
}

impl<T> AggWatch<T> {
    fn new(program: &'static str) -> Self {
        Self { program, _m: PhantomData }
    }
}

#[async_trait]
impl<T: Send + Sync + 'static> Processor for AggWatch<T> {
    type InputType = InstructionProcessorInputType<T>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (metadata, _decoded, _nested, _raw) = data;
        let signer = metadata.transaction_metadata.fee_payer.to_string();
        if !watched(&signer) {
            return Ok(());
        }
        let signature = metadata.transaction_metadata.signature.to_string();
        let slot = metadata.transaction_metadata.slot;
        COUNT_AGG.fetch_add(1, Ordering::Relaxed);
        log::info!(
            "WATCH_AGG signer={} program={} slot={} sig={}",
            signer, self.program, slot, signature
        );
        Ok(())
    }
}

// ─── Health/stats HTTP server ─────────────────────────────────────────

async fn health_server(port: u16) {
    async fn health() -> &'static str {
        "ok"
    }
    async fn stats() -> Json<serde_json::Value> {
        Json(json!({
            "watch_wallets": WATCH.iter().collect::<Vec<_>>(),
            "counts": {
                "pumpfun_buy": COUNT_PF_BUY.load(Ordering::Relaxed),
                "pumpfun_sell": COUNT_PF_SELL.load(Ordering::Relaxed),
                "pumpfun_trade_event": COUNT_PF_TRADE_EVT.load(Ordering::Relaxed),
                "pumpswap_buy": COUNT_PS_BUY.load(Ordering::Relaxed),
                "pumpswap_sell": COUNT_PS_SELL.load(Ordering::Relaxed),
                "aggregator_route": COUNT_AGG.load(Ordering::Relaxed),
            }
        }))
    }
    let app = Router::new()
        .route("/health", get(health))
        .route("/stats", get(stats));
    let bind = format!("127.0.0.1:{}", port);
    match tokio::net::TcpListener::bind(&bind).await {
        Ok(l) => {
            log::info!("shadow health endpoint listening on {}", bind);
            let _ = axum::serve(l, app).await;
        }
        Err(e) => log::error!("failed to bind health endpoint on {}: {}", bind, e),
    }
}

// ─── Entry point ──────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Env vars are loaded by systemd's EnvironmentFile directive (or shell).
    // No dotenv crate dependency — keep this binary's deps minimal.
    tracing_subscriber::fmt()
        .json()
        .with_target(false)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    log::info!(
        "frankfurt-shadow starting; watch_wallets={} health_port={}",
        WATCH.len(),
        std::env::var("SHADOW_HEALTH_PORT").unwrap_or_else(|_| "3004".into()),
    );

    if WATCH.is_empty() {
        log::warn!("CARBON_WATCH_WALLETS is empty — no signers will match, nothing will be logged");
    }

    let health_port: u16 = std::env::var("SHADOW_HEALTH_PORT")
        .unwrap_or_else(|_| "3004".into())
        .parse()
        .unwrap_or(3004);
    tokio::spawn(health_server(health_port));

    let shred_url = std::env::var("SHRED_GRPC_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:9999".into());
    log::info!("connecting to shred gRPC at {}", shred_url);
    let shred = JitoShredstreamGrpcClient::new(shred_url);

    // Full Carbon decoder set: 64 decoders. PumpFun + PumpSwap have dedicated
    // processors that label BUY vs SELL and extract the mint. Everything else
    // uses the generic AggWatch surveillance processor (counts + log line).
    // Watch list filtering happens inside each Processor.
    Pipeline::builder()
        .datasource(shred)
        .instruction(PumpfunDecoder, PumpfunWatch)
        .instruction(PumpSwapDecoder, PumpSwapWatch)
        .instruction(carbon_address_lookup_table_decoder::AddressLookupTableDecoder, AggWatch::<carbon_address_lookup_table_decoder::instructions::AddressLookupTableInstruction>::new("address_lookup_table"))
        .instruction(carbon_associated_token_account_decoder::SplAssociatedTokenAccountDecoder, AggWatch::<carbon_associated_token_account_decoder::instructions::SplAssociatedTokenAccountInstruction>::new("associated_token_account"))
        .instruction(carbon_bonkswap_decoder::BonkswapDecoder, AggWatch::<carbon_bonkswap_decoder::instructions::BonkswapInstruction>::new("bonkswap"))
        .instruction(carbon_boop_decoder::BoopDecoder, AggWatch::<carbon_boop_decoder::instructions::BoopInstruction>::new("boop"))
        .instruction(carbon_bubblegum_decoder::BubblegumDecoder, AggWatch::<carbon_bubblegum_decoder::instructions::BubblegumInstruction>::new("bubblegum"))
        .instruction(carbon_circle_message_transmitter_v2_decoder::MessageTransmitterV2Decoder, AggWatch::<carbon_circle_message_transmitter_v2_decoder::instructions::MessageTransmitterV2Instruction>::new("circle_message_transmitter_v2"))
        .instruction(carbon_circle_token_messenger_v2_decoder::TokenMessengerMinterV2Decoder, AggWatch::<carbon_circle_token_messenger_v2_decoder::instructions::TokenMessengerMinterV2Instruction>::new("circle_token_messenger_v2"))
        .instruction(carbon_dflow_aggregator_v4_decoder::SwapOrchestratorDecoder, AggWatch::<carbon_dflow_aggregator_v4_decoder::instructions::SwapOrchestratorInstruction>::new("dflow_aggregator_v4"))
        .instruction(carbon_drift_v2_decoder::DriftDecoder, AggWatch::<carbon_drift_v2_decoder::instructions::DriftInstruction>::new("drift_v2"))
        .instruction(carbon_fluxbeam_decoder::FluxbeamDecoder, AggWatch::<carbon_fluxbeam_decoder::instructions::FluxbeamInstruction>::new("fluxbeam"))
        .instruction(carbon_gavel_decoder::GavelDecoder, AggWatch::<carbon_gavel_decoder::instructions::GavelInstruction>::new("gavel"))
        .instruction(carbon_heaven_decoder::HeavenDecoder, AggWatch::<carbon_heaven_decoder::instructions::HeavenInstruction>::new("heaven"))
        .instruction(carbon_jupiter_dca_decoder::JupiterDcaDecoder, AggWatch::<carbon_jupiter_dca_decoder::instructions::JupiterDcaInstruction>::new("jupiter_dca"))
        .instruction(carbon_jupiter_lend_decoder::LiquidityDecoder, AggWatch::<carbon_jupiter_lend_decoder::instructions::LiquidityInstruction>::new("jupiter_lend"))
        .instruction(carbon_jupiter_limit_order_2_decoder::JupiterLimitOrder2Decoder, AggWatch::<carbon_jupiter_limit_order_2_decoder::instructions::JupiterLimitOrder2Instruction>::new("jupiter_limit_order_2"))
        .instruction(carbon_jupiter_limit_order_decoder::JupiterLimitOrderDecoder, AggWatch::<carbon_jupiter_limit_order_decoder::instructions::JupiterLimitOrderInstruction>::new("jupiter_limit_order"))
        .instruction(carbon_jupiter_perpetuals_decoder::PerpetualsDecoder, AggWatch::<carbon_jupiter_perpetuals_decoder::instructions::PerpetualsInstruction>::new("jupiter_perpetuals"))
        .instruction(carbon_jupiter_swap_decoder::JupiterSwapDecoder, AggWatch::<carbon_jupiter_swap_decoder::instructions::JupiterSwapInstruction>::new("jupiter_swap"))
        .instruction(carbon_kamino_farms_decoder::KaminoFarmsDecoder, AggWatch::<carbon_kamino_farms_decoder::instructions::KaminoFarmsInstruction>::new("kamino_farms"))
        .instruction(carbon_kamino_lending_decoder::KaminoLendingDecoder, AggWatch::<carbon_kamino_lending_decoder::instructions::KaminoLendingInstruction>::new("kamino_lending"))
        .instruction(carbon_kamino_limit_order_decoder::KaminoLimitOrderDecoder, AggWatch::<carbon_kamino_limit_order_decoder::instructions::KaminoLimitOrderInstruction>::new("kamino_limit_order"))
        .instruction(carbon_kamino_vault_decoder::KaminoVaultDecoder, AggWatch::<carbon_kamino_vault_decoder::instructions::KaminoVaultInstruction>::new("kamino_vault"))
        .instruction(carbon_lifinity_amm_v2_decoder::LifinityAmmV2Decoder, AggWatch::<carbon_lifinity_amm_v2_decoder::instructions::LifinityAmmV2Instruction>::new("lifinity_amm_v2"))
        .instruction(carbon_marginfi_v2_decoder::MarginfiV2Decoder, AggWatch::<carbon_marginfi_v2_decoder::instructions::MarginfiV2Instruction>::new("marginfi_v2"))
        .instruction(carbon_marinade_finance_decoder::MarinadeFinanceDecoder, AggWatch::<carbon_marinade_finance_decoder::instructions::MarinadeFinanceInstruction>::new("marinade_finance"))
        .instruction(carbon_memo_program_decoder::MemoProgramDecoder, AggWatch::<carbon_memo_program_decoder::instructions::MemoProgramInstruction>::new("memo_program"))
        .instruction(carbon_meteora_damm_v2_decoder::MeteoraDammV2Decoder, AggWatch::<carbon_meteora_damm_v2_decoder::instructions::MeteoraDammV2Instruction>::new("meteora_damm_v2"))
        .instruction(carbon_meteora_dbc_decoder::DynamicBondingCurveDecoder, AggWatch::<carbon_meteora_dbc_decoder::instructions::DynamicBondingCurveInstruction>::new("meteora_dbc"))
        .instruction(carbon_meteora_dlmm_decoder::MeteoraDlmmDecoder, AggWatch::<carbon_meteora_dlmm_decoder::instructions::MeteoraDlmmInstruction>::new("meteora_dlmm"))
        .instruction(carbon_meteora_pools_decoder::MeteoraPoolsDecoder, AggWatch::<carbon_meteora_pools_decoder::instructions::MeteoraPoolsProgramInstruction>::new("meteora_pools"))
        .instruction(carbon_meteora_vault_decoder::MeteoraVaultDecoder, AggWatch::<carbon_meteora_vault_decoder::instructions::MeteoraVaultInstruction>::new("meteora_vault"))
        .instruction(carbon_moonshot_decoder::MoonshotDecoder, AggWatch::<carbon_moonshot_decoder::instructions::MoonshotInstruction>::new("moonshot"))
        .instruction(carbon_mpl_core_decoder::MplCoreProgramDecoder, AggWatch::<carbon_mpl_core_decoder::instructions::MplCoreProgramInstruction>::new("mpl_core"))
        .instruction(carbon_mpl_token_metadata_decoder::TokenMetadataDecoder, AggWatch::<carbon_mpl_token_metadata_decoder::instructions::TokenMetadataInstruction>::new("mpl_token_metadata"))
        .instruction(carbon_name_service_decoder::NameDecoder, AggWatch::<carbon_name_service_decoder::instructions::NameInstruction>::new("name_service"))
        .instruction(carbon_okx_dex_decoder::OkxDexDecoder, AggWatch::<carbon_okx_dex_decoder::instructions::OkxDexInstruction>::new("okx_dex"))
        .instruction(carbon_onchain_labs_dex_v1_decoder::OnchainLabsDexV1Decoder, AggWatch::<carbon_onchain_labs_dex_v1_decoder::instructions::OnchainLabsDexV1Instruction>::new("onchain_labs_dex_v1"))
        .instruction(carbon_onchain_labs_dex_v2_decoder::OnchainLabsDexV2Decoder, AggWatch::<carbon_onchain_labs_dex_v2_decoder::instructions::OnchainLabsDexV2Instruction>::new("onchain_labs_dex_v2"))
        .instruction(carbon_openbook_v2_decoder::OpenbookV2Decoder, AggWatch::<carbon_openbook_v2_decoder::instructions::OpenbookV2Instruction>::new("openbook_v2"))
        .instruction(carbon_orca_whirlpool_decoder::OrcaWhirlpoolDecoder, AggWatch::<carbon_orca_whirlpool_decoder::instructions::WhirlpoolInstruction>::new("orca_whirlpool"))
        .instruction(carbon_pancake_swap_decoder::PancakeSwapDecoder, AggWatch::<carbon_pancake_swap_decoder::instructions::AmmV3Instruction>::new("pancake_swap"))
        .instruction(carbon_phoenix_v1_decoder::PhoenixDecoder, AggWatch::<carbon_phoenix_v1_decoder::instructions::PhoenixInstruction>::new("phoenix_v1"))
        .instruction(carbon_pump_fees_decoder::PumpFeesDecoder, AggWatch::<carbon_pump_fees_decoder::instructions::PumpFeesInstruction>::new("pump_fees"))
        .instruction(carbon_raydium_amm_v4_decoder::RaydiumAmmV4Decoder, AggWatch::<carbon_raydium_amm_v4_decoder::instructions::RaydiumAmmV4Instruction>::new("raydium_amm_v4"))
        .instruction(carbon_raydium_clmm_decoder::RaydiumClmmDecoder, AggWatch::<carbon_raydium_clmm_decoder::instructions::RaydiumClmmInstruction>::new("raydium_clmm"))
        .instruction(carbon_raydium_cpmm_decoder::RaydiumCpmmDecoder, AggWatch::<carbon_raydium_cpmm_decoder::instructions::RaydiumCpmmInstruction>::new("raydium_cpmm"))
        .instruction(carbon_raydium_launchpad_decoder::RaydiumLaunchpadDecoder, AggWatch::<carbon_raydium_launchpad_decoder::instructions::RaydiumLaunchpadInstruction>::new("raydium_launchpad"))
        .instruction(carbon_raydium_liquidity_locking_decoder::RaydiumLiquidityLockingDecoder, AggWatch::<carbon_raydium_liquidity_locking_decoder::instructions::RaydiumLiquidityLockingInstruction>::new("raydium_liquidity_locking"))
        .instruction(carbon_raydium_stable_swap_decoder::RaydiumStableSwapAmmDecoder, AggWatch::<carbon_raydium_stable_swap_decoder::instructions::RaydiumStableSwapAmmInstruction>::new("raydium_stable_swap"))
        .instruction(carbon_sharky_decoder::SharkyDecoder, AggWatch::<carbon_sharky_decoder::instructions::SharkyInstruction>::new("sharky"))
        .instruction(carbon_solayer_restaking_program_decoder::SolayerRestakingProgramDecoder, AggWatch::<carbon_solayer_restaking_program_decoder::instructions::SolayerRestakingProgramInstruction>::new("solayer_restaking_program"))
        .instruction(carbon_stabble_stable_swap_decoder::StableSwapDecoder, AggWatch::<carbon_stabble_stable_swap_decoder::instructions::StableSwapInstruction>::new("stabble_stable_swap"))
        .instruction(carbon_stabble_weighted_swap_decoder::WeightedSwapDecoder, AggWatch::<carbon_stabble_weighted_swap_decoder::instructions::WeightedSwapInstruction>::new("stabble_weighted_swap"))
        .instruction(carbon_stake_program_decoder::StakeProgramDecoder, AggWatch::<carbon_stake_program_decoder::instructions::StakeProgramInstruction>::new("stake_program"))
        .instruction(carbon_swig_decoder::SwigDecoder, AggWatch::<carbon_swig_decoder::instructions::SwigInstruction>::new("swig"))
        .instruction(carbon_system_program_decoder::SystemProgramDecoder, AggWatch::<carbon_system_program_decoder::instructions::SystemProgramInstruction>::new("system_program"))
        .instruction(carbon_token_2022_decoder::Token2022Decoder, AggWatch::<carbon_token_2022_decoder::instructions::Token2022Instruction>::new("token_2022"))
        .instruction(carbon_token_program_decoder::TokenProgramDecoder, AggWatch::<carbon_token_program_decoder::instructions::TokenProgramInstruction>::new("token_program"))
        .instruction(carbon_vertigo_decoder::VertigoDecoder, AggWatch::<carbon_vertigo_decoder::instructions::VertigoInstruction>::new("vertigo"))
        .instruction(carbon_virtuals_decoder::VirtualsDecoder, AggWatch::<carbon_virtuals_decoder::instructions::VirtualsInstruction>::new("virtuals"))
        .instruction(carbon_wavebreak_decoder::WavebreakDecoder, AggWatch::<carbon_wavebreak_decoder::instructions::WavebreakInstruction>::new("wavebreak"))
        .instruction(carbon_zeta_decoder::ZetaDecoder, AggWatch::<carbon_zeta_decoder::instructions::ZetaInstruction>::new("zeta"))
        .build()?
        .run()
        .await?;

    Ok(())
}
