use {
    async_trait::async_trait,
    carbon_core::{
        error::CarbonResult,
        instruction::{DecodedInstruction, InstructionMetadata, NestedInstructions},
        metrics::MetricsCollection,
        processor::Processor,
    },
    carbon_log_metrics::LogMetrics,
    carbon_meteora_dlmm_decoder::{
        instructions::MeteoraDlmmInstruction, MeteoraDlmmDecoder, PROGRAM_ID as METEORA_PROGRAM_ID,
    },
    carbon_rpc_transaction_crawler_datasource::{
        ConnectionConfig, Filters, RetryConfig, RpcTransactionCrawler,
    },
    solana_commitment_config::CommitmentConfig,
    std::{env, sync::Arc, time::Duration},
};

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let filters = Filters::new(None, None, None);
    let connection_config = ConnectionConfig::new(
        100,                     // Batch limit
        Duration::from_secs(5),  // Polling interval
        5,                       // Max Concurrent Requests
        RetryConfig::no_retry(), // Retry config
        None,                    // Max Signature Channel Size
        None,                    // Max Transaction Channel Size
        true,                    // Blocking send
    );

    let transaction_crawler = RpcTransactionCrawler::new(
        env::var("RPC_URL").unwrap_or_default(), // RPC URL
        METEORA_PROGRAM_ID,                      // The test account
        connection_config,                       // Connection config
        filters,                                 // Filters
        Some(CommitmentConfig::finalized()),     // Commitment config
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(transaction_crawler)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(3)
        .instruction(MeteoraDlmmDecoder, MeteoraInstructionProcessor)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct MeteoraInstructionProcessor;

#[async_trait]
impl Processor for MeteoraInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<MeteoraDlmmInstruction>,
        NestedInstructions,
        solana_instruction::Instruction,
    );

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let (_instruction_metadata, decoded_instruction, _nested_instructions, _) = data;

        // Process all Meteora Events and add each to DB
        match decoded_instruction.data {
            MeteoraDlmmInstruction::AddLiquidity(_add_liquidity) => {}
            MeteoraDlmmInstruction::RemoveLiquidity(_remove_liquidity) => {}
            MeteoraDlmmInstruction::Swap(_swap) => {}
            MeteoraDlmmInstruction::ClaimReward(_claim_reward) => {}
            MeteoraDlmmInstruction::FundReward(_fund_reward) => {}
            MeteoraDlmmInstruction::InitializeReward(_initialize_reward) => {}
            MeteoraDlmmInstruction::UpdateRewardFunder(_update_reward_funder) => {}
            MeteoraDlmmInstruction::UpdateRewardDuration(_update_reward_duration) => {}
            MeteoraDlmmInstruction::ClaimFee(_claim_fee) => {}
            MeteoraDlmmInstruction::ClosePosition(_position_close) => {}
            MeteoraDlmmInstruction::LbPairCreateEvent(_lb_pair_create) => {}
            MeteoraDlmmInstruction::PositionCreateEvent(_position_create) => {}
            MeteoraDlmmInstruction::FeeParameterUpdateEvent(_fee_parameter_update) => {}
            MeteoraDlmmInstruction::IncreaseObservationEvent(_increase_observation) => {}
            MeteoraDlmmInstruction::WithdrawIneligibleReward(_withdraw_ineligible_reward) => {}
            MeteoraDlmmInstruction::UpdatePositionOperator(_update_position_operator) => {}
            MeteoraDlmmInstruction::UpdatePositionLockReleasePointEvent(
                _update_position_lock_release_point,
            ) => {}
            MeteoraDlmmInstruction::InitializeLbPair(_initialize_lb_pair) => {}
            MeteoraDlmmInstruction::GoToABin(_go_to_abin) => {}
            _ => {}
        };

        Ok(())
    }
}
