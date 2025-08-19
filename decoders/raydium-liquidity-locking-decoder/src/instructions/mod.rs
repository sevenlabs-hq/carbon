use {super::RaydiumLiquidityLockingDecoder, crate::PROGRAM_ID};
pub mod collect_clmm_fees_and_rewards;
pub mod collect_cp_fees;
pub mod lock_clmm_position;
pub mod lock_cp_liquidity;
pub mod settle_cp_fee_event;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum RaydiumLiquidityLockingInstruction {
    LockClmmPosition(lock_clmm_position::LockClmmPosition),
    CollectClmmFeesAndRewards(collect_clmm_fees_and_rewards::CollectClmmFeesAndRewards),
    LockCpLiquidity(lock_cp_liquidity::LockCpLiquidity),
    CollectCpFees(collect_cp_fees::CollectCpFees),
    SettleCpFeeEvent(settle_cp_fee_event::SettleCpFeeEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for RaydiumLiquidityLockingDecoder {
    type InstructionType = RaydiumLiquidityLockingInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            RaydiumLiquidityLockingInstruction::LockClmmPosition => lock_clmm_position::LockClmmPosition,
            RaydiumLiquidityLockingInstruction::CollectClmmFeesAndRewards => collect_clmm_fees_and_rewards::CollectClmmFeesAndRewards,
            RaydiumLiquidityLockingInstruction::LockCpLiquidity => lock_cp_liquidity::LockCpLiquidity,
            RaydiumLiquidityLockingInstruction::CollectCpFees => collect_cp_fees::CollectCpFees,
            RaydiumLiquidityLockingInstruction::SettleCpFeeEvent => settle_cp_fee_event::SettleCpFeeEvent,
        )
    }
}
