use {
    async_trait::async_trait,
    carbon_core::{
        error::CarbonResult, instruction::InstructionProcessorInputType,
        metrics::MetricsCollection, processor::Processor,
    },
    carbon_pumpfun_decoder::{
        instructions::PumpfunInstruction, PumpfunDecoder, PROGRAM_ID as PUMPFUN_PROGRAM_ID,
    },
    helius::types::{
        Cluster, RpcTransactionsConfig, TransactionCommitment, TransactionDetails,
        TransactionSubscribeFilter, TransactionSubscribeOptions, UiEnhancedTransactionEncoding,
    },
    solana_native_token::LAMPORTS_PER_SOL,
    std::{collections::HashSet, sync::Arc},
    tokio::sync::RwLock,
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let helius_websocket = carbon_helius_atlas_ws_datasource::HeliusWebsocket::new(
        std::env::var("API_KEY").expect("API_KEY must be set"),
        carbon_helius_atlas_ws_datasource::Filters {
            accounts: vec![],
            transactions: Some(RpcTransactionsConfig {
                filter: TransactionSubscribeFilter {
                    account_include: Some(vec![PUMPFUN_PROGRAM_ID.to_string().clone()]),
                    account_exclude: None,
                    account_required: None,
                    vote: None,
                    failed: None,
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
        Arc::new(RwLock::new(HashSet::new())),
        Cluster::MainnetBeta,
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(helius_websocket)
        .instruction(PumpfunDecoder, PumpfunInstructionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct PumpfunInstructionProcessor;

#[async_trait]
impl Processor for PumpfunInstructionProcessor {
    type InputType = InstructionProcessorInputType<PumpfunInstruction>;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let pumpfun_instruction: PumpfunInstruction = data.1.data;

        match pumpfun_instruction {
            PumpfunInstruction::CreateEvent(create_event) => {
                log::info!("New token created: {create_event:#?}");
            }
            PumpfunInstruction::TradeEvent(trade_event) => {
                if trade_event.sol_amount > 10 * LAMPORTS_PER_SOL {
                    log::info!("Big trade occured: {trade_event:#?}");
                }
            }
            PumpfunInstruction::CompleteEvent(complete_event) => {
                log::info!("Bonded: {complete_event:#?}");
            }
            _ => {}
        };

        Ok(())
    }
}
