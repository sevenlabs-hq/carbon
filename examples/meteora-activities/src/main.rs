use async_trait::async_trait;
use carbon_core::{
    error::CarbonResult,
    instruction::{DecodedInstruction, InstructionMetadata, NestedInstruction},
    metrics::Metrics,
    processor::Processor,
};
use carbon_meteora_dlmm_decoder::{instructions::MeteoraDlmmInstruction, MeteoraDlmmDecoder};
use carbon_rpc_transaction_crawler_datasource::{Filters, RpcTransactionCrawler};
use log_metrics::LogMetrics;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey, pubkey::Pubkey};
use std::{env, sync::Arc, time::Duration};

pub const METEORA_PROGRAM_ID: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();

    let filters = Filters::new(None, None, None);

    let transaction_crawler = RpcTransactionCrawler::new(
        env::var("RPC_URL").unwrap_or_default(), // RPC URL
        METEORA_PROGRAM_ID,                      // The test account
        100,                                     // Batch limit
        Duration::from_secs(5),                  // Polling interval
        filters,                                 // Filters
        Some(CommitmentConfig::finalized()),     // Commitment config
        20,                                      // Max Concurrent Requests
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
        Vec<NestedInstruction>,
    );

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Vec<Arc<dyn Metrics>>,
    ) -> CarbonResult<()> {
        let (_instruction_metadata, decoded_instruction, _nested_instructions) = data;

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
            MeteoraDlmmInstruction::PositionClose(_position_close) => {}
            MeteoraDlmmInstruction::LbPairCreate(_lb_pair_create) => {}
            MeteoraDlmmInstruction::PositionCreate(_position_create) => {}
            MeteoraDlmmInstruction::FeeParameterUpdate(_fee_parameter_update) => {}
            MeteoraDlmmInstruction::IncreaseObservation(_increase_observation) => {}
            MeteoraDlmmInstruction::WithdrawIneligibleReward(_withdraw_ineligible_reward) => {}
            MeteoraDlmmInstruction::UpdatePositionOperator(_update_position_operator) => {}
            MeteoraDlmmInstruction::UpdatePositionLockReleasePoint(
                _update_position_lock_release_point,
            ) => {}
            MeteoraDlmmInstruction::InitializeLbPair(_initialize_lb_pair) => {}
            MeteoraDlmmInstruction::GoToABin(_go_to_abin) => {}
            _ => {}
        };

        Ok(())
    }
}