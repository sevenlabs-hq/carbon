use {
    async_trait::async_trait,
    carbon_core::{
        error::CarbonResult,
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstructions},
        metrics::MetricsCollection,
        processor::Processor,
    },
    carbon_jupiter_swap_decoder::{
        instructions::JupiterSwapInstruction, JupiterSwapDecoder,
        PROGRAM_ID as JUPITER_SWAP_PROGRAM_ID,
    },
    carbon_log_metrics::LogMetrics,
    carbon_yellowstone_grpc_datasource::YellowstoneGrpcGeyserClient,
    std::{
        collections::{HashMap, HashSet},
        env,
        sync::Arc,
    },
    tokio::sync::RwLock,
    yellowstone_grpc_proto::geyser::{CommitmentLevel, SubscribeRequestFilterTransactions},
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();
    dotenv::dotenv().ok();

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

    let yellowstone_grpc = YellowstoneGrpcGeyserClient::new(
        env::var("GEYSER_URL").unwrap_or_default(),
        env::var("X_TOKEN").ok(),
        Some(CommitmentLevel::Confirmed),
        HashMap::default(),
        transaction_filters,
        Arc::new(RwLock::new(HashSet::new())),
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(yellowstone_grpc)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(3)
        .instruction(JupiterSwapDecoder, JupiterSwapInstructionProcessor)
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct JupiterSwapInstructionProcessor;

#[async_trait]
impl Processor for JupiterSwapInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<JupiterSwapInstruction>,
        NestedInstructions,
    );
    async fn process(
        &mut self,
        (metadata, instruction, nested_instructions): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = metadata.transaction_metadata.signature;

        match instruction.data {
            JupiterSwapInstruction::Claim(claim) => {
                log::info!("claim: signature: {signature}, claim: {claim:?}");
            }
            JupiterSwapInstruction::ClaimToken(claim_token) => {
                log::info!("claim_token: signature: {signature}, claim_token: {claim_token:?}");
            }
            JupiterSwapInstruction::CreateOpenOrders(create_open_orders) => {
                log::info!("create_open_orders: signature: {signature}, create_open_orders: {create_open_orders:?}");
            }
            JupiterSwapInstruction::CreateProgramOpenOrders(create_program_open_orders) => {
                log::info!("create_program_open_orders: signature: {signature}, create_program_open_orders: {create_program_open_orders:?}");
            }
            JupiterSwapInstruction::CreateTokenLedger(create_token_ledger) => {
                log::info!("create_token_ledger: signature: {signature}, create_token_ledger: {create_token_ledger:?}");
            }
            JupiterSwapInstruction::ExactOutRoute(exact_out_route) => {
                assert!(
                    !nested_instructions.is_empty(),
                    "nested instructions empty: {} ",
                    signature
                );
                log::info!(
                    "exact_out_route: signature: {signature}, exact_out_route: {exact_out_route:?}"
                );
            }
            JupiterSwapInstruction::Route(route) => {
                assert!(
                    !nested_instructions.is_empty(),
                    "nested instructions empty: {} ",
                    signature
                );
                log::info!("route: signature: {signature}, route: {route:?}");
            }
            JupiterSwapInstruction::RouteWithTokenLedger(route_with_token_ledger) => {
                assert!(
                    !nested_instructions.is_empty(),
                    "nested instructions empty: {} ",
                    signature
                );
                log::info!("route_with_token_ledger: signature: {signature}, route_with_token_ledger: {route_with_token_ledger:?}");
            }
            JupiterSwapInstruction::SetTokenLedger(set_token_ledger) => {
                log::info!("set_token_ledger: signature: {signature}, set_token_ledger: {set_token_ledger:?}");
            }
            JupiterSwapInstruction::SharedAccountsExactOutRoute(
                shared_accounts_exact_out_route,
            ) => {
                assert!(
                    !nested_instructions.is_empty(),
                    "nested instructions empty: {} ",
                    signature
                );
                log::info!("shared_accounts_exact_out_route: signature: {signature}, shared_accounts_exact_out_route: {shared_accounts_exact_out_route:?}");
            }
            JupiterSwapInstruction::SharedAccountsRoute(shared_accounts_route) => {
                assert!(
                    !nested_instructions.is_empty(),
                    "nested instructions empty: {} ",
                    signature
                );
                log::info!("shared_accounts_route: signature: {signature}, shared_accounts_route: {shared_accounts_route:?}");
            }
            JupiterSwapInstruction::SharedAccountsRouteWithTokenLedger(
                shared_accounts_route_with_token_ledger,
            ) => {
                assert!(
                    !nested_instructions.is_empty(),
                    "nested instructions empty: {} ",
                    signature
                );
                log::info!("shared_accounts_route_with_token_ledger: signature: {signature}, shared_accounts_route_with_token_ledger: {shared_accounts_route_with_token_ledger:?}");
            }
            JupiterSwapInstruction::FeeEvent(fee_event) => {
                log::info!("fee_event: signature: {signature}, fee_event: {fee_event:?}");
            }
            JupiterSwapInstruction::SwapEvent(swap_event) => {
                log::info!("swap_event: signature: {signature}, swap_event: {swap_event:?}");
            }
        };

        Ok(())
    }
}
