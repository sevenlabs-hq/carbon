use {
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType, processor::Processor,
    },
    carbon_jupiter_swap_decoder::{
        instructions::{CpiEvent, JupiterSwapInstruction},
        JupiterSwapDecoder, PROGRAM_ID as JUPITER_SWAP_PROGRAM_ID,
    },
    carbon_log_metrics::LogMetrics,
    carbon_yellowstone_grpc_datasource::{
        YellowstoneGrpcClientConfig, YellowstoneGrpcGeyserClient,
    },
    std::{
        collections::{HashMap, HashSet},
        env,
        sync::Arc,
        time::Duration,
    },
    tokio::sync::RwLock,
    yellowstone_grpc_proto::geyser::{CommitmentLevel, SubscribeRequestFilterTransactions},
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // NOTE: Workaround, that solving issue https://github.com/rustls/rustls/issues/1877
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Can't set crypto provider to aws_lc_rs");

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec![JUPITER_SWAP_PROGRAM_ID.to_string().clone()],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert(
        "jupiter_swap_transaction_filter".to_string(),
        transaction_filter,
    );

    let geyser_config = YellowstoneGrpcClientConfig::new(
        None,
        Some(Duration::from_secs(15)),
        Some(Duration::from_secs(15)),
        None,
        None,
        None,
    );

    let yellowstone_grpc = YellowstoneGrpcGeyserClient::new(
        env::var("GEYSER_URL").unwrap_or_default(),
        env::var("X_TOKEN").ok(),
        Some(CommitmentLevel::Confirmed),
        HashMap::default(),
        transaction_filters,
        Default::default(),
        Arc::new(RwLock::new(HashSet::new())),
        geyser_config,
        None,
        None,
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(yellowstone_grpc)
        .metrics(Arc::new(LogMetrics::new()))
        .instruction(JupiterSwapDecoder, JupiterSwapInstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct JupiterSwapInstructionProcessor;

impl Processor<InstructionProcessorInputType<'_, JupiterSwapInstruction>>
    for JupiterSwapInstructionProcessor
{
    async fn process(
        &mut self,
        input: &InstructionProcessorInputType<'_, JupiterSwapInstruction>,
    ) -> CarbonResult<()> {
        let signature = input.metadata.transaction_metadata.signature;

        match input.decoded_instruction {
            JupiterSwapInstruction::Claim { data, .. } => {
                log::info!("claim: signature: {signature}, claim: {data:?}");
            }
            JupiterSwapInstruction::ClaimToken { data, .. } => {
                log::info!("claim_token: signature: {signature}, claim_token: {data:?}");
            }
            JupiterSwapInstruction::CreateTokenLedger { data, .. } => {
                log::info!(
                    "create_token_ledger: signature: {signature}, create_token_ledger: {data:?}"
                );
            }
            JupiterSwapInstruction::ExactOutRoute { data, .. } => {
                log::info!("exact_out_route: signature: {signature}, exact_out_route: {data:?}");
            }
            JupiterSwapInstruction::Route { data, .. } => {
                log::info!("route: signature: {signature}, route: {data:?}");
            }
            JupiterSwapInstruction::RouteWithTokenLedger { data, .. } => {
                log::info!("route_with_token_ledger: signature: {signature}, route_with_token_ledger: {data:?}");
            }
            JupiterSwapInstruction::SetTokenLedger { data, .. } => {
                log::info!("set_token_ledger: signature: {signature}, set_token_ledger: {data:?}");
            }
            JupiterSwapInstruction::SharedAccountsExactOutRoute { data, .. } => {
                log::info!("shared_accounts_exact_out_route: signature: {signature}, shared_accounts_exact_out_route: {data:?}");
            }
            JupiterSwapInstruction::ExactOutRouteV2 { data, .. } => {
                log::info!(
                    "exact_out_route_v2: signature: {signature}, exact_out_route_v2: {data:?}"
                );
            }
            JupiterSwapInstruction::RouteV2 { data, .. } => {
                log::info!("route_v2: signature: {signature}, route_v2: {data:?}");
            }
            JupiterSwapInstruction::SharedAccountsExactOutRouteV2 { data, .. } => {
                log::info!("shared_accounts_exact_out_route_v2: signature: {signature}, shared_accounts_exact_out_route_v2: {data:?}");
            }
            JupiterSwapInstruction::SharedAccountsRouteV2 { data, .. } => {
                log::info!("shared_accounts_route_v2: signature: {signature}, shared_accounts_route_v2: {data:?}");
            }
            JupiterSwapInstruction::SharedAccountsRoute { data, .. } => {
                log::info!("shared_accounts_route: signature: {signature}, shared_accounts_route: {data:?}");
            }
            JupiterSwapInstruction::SharedAccountsRouteWithTokenLedger { data, .. } => {
                log::info!("shared_accounts_route_with_token_ledger: signature: {signature}, shared_accounts_route_with_token_ledger: {data:?}");
            }
            JupiterSwapInstruction::CloseToken { data, .. } => {
                log::info!("close_token: signature: {signature}, close_token: {data:?}");
            }
            JupiterSwapInstruction::CreateTokenAccount { data, .. } => {
                log::info!(
                    "create_token_account: signature: {signature}, create_token_account: {data:?}"
                );
            }
            JupiterSwapInstruction::CpiEvent { data, .. } => match data {
                CpiEvent::FeeEvent(fee_event) => {
                    log::info!("fee_event: signature: {signature}, fee_event: {fee_event:?}");
                }
                CpiEvent::SwapEvent(swap_event) => {
                    log::info!("swap_event: signature: {signature}, swap_event: {swap_event:?}");
                }
                CpiEvent::SwapsEvent(swaps_event) => {
                    log::info!("swaps_event: signature: {signature}, swaps_event: {swaps_event:?}");
                }
                CpiEvent::CandidateSwapResults(candidate_swap_results) => {
                    log::info!("candidate_swap_results: signature: {signature}, candidate_swap_results: {candidate_swap_results:?}");
                }
                CpiEvent::BestSwapOutAmountViolation(best_swap_out_amount_violation) => {
                    log::info!("best_swap_out_amount_violation: signature: {signature}, best_swap_out_amount_violation: {best_swap_out_amount_violation:?}");
                }
            },
        };

        Ok(())
    }
}
