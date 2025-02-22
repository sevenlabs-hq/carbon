use crate::PROGRAM_ID;

use super::RaydiumCpmmDecoder;
pub mod collect_fund_fee;
pub mod collect_protocol_fee;
pub mod create_amm_config;
pub mod deposit;
pub mod initialize;
pub mod lp_change_event;
pub mod swap_base_input;
pub mod swap_base_output;
pub mod swap_event;
pub mod update_amm_config;
pub mod update_pool_status;
pub mod withdraw;

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
pub enum RaydiumCpmmInstruction {
    CreateAmmConfig(create_amm_config::CreateAmmConfig),
    UpdateAmmConfig(update_amm_config::UpdateAmmConfig),
    UpdatePoolStatus(update_pool_status::UpdatePoolStatus),
    CollectProtocolFee(collect_protocol_fee::CollectProtocolFee),
    CollectFundFee(collect_fund_fee::CollectFundFee),
    Initialize(initialize::Initialize),
    Deposit(deposit::Deposit),
    Withdraw(withdraw::Withdraw),
    SwapBaseInput(swap_base_input::SwapBaseInput),
    SwapBaseOutput(swap_base_output::SwapBaseOutput),
    LpChangeEvent(lp_change_event::LpChangeEvent),
    SwapEvent(swap_event::SwapEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for RaydiumCpmmDecoder {
    type InstructionType = RaydiumCpmmInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            RaydiumCpmmInstruction::CreateAmmConfig => create_amm_config::CreateAmmConfig,
            RaydiumCpmmInstruction::UpdateAmmConfig => update_amm_config::UpdateAmmConfig,
            RaydiumCpmmInstruction::UpdatePoolStatus => update_pool_status::UpdatePoolStatus,
            RaydiumCpmmInstruction::CollectProtocolFee => collect_protocol_fee::CollectProtocolFee,
            RaydiumCpmmInstruction::CollectFundFee => collect_fund_fee::CollectFundFee,
            RaydiumCpmmInstruction::Initialize => initialize::Initialize,
            RaydiumCpmmInstruction::Deposit => deposit::Deposit,
            RaydiumCpmmInstruction::Withdraw => withdraw::Withdraw,
            RaydiumCpmmInstruction::SwapBaseInput => swap_base_input::SwapBaseInput,
            RaydiumCpmmInstruction::SwapBaseOutput => swap_base_output::SwapBaseOutput,
            RaydiumCpmmInstruction::LpChangeEvent => lp_change_event::LpChangeEvent,
            RaydiumCpmmInstruction::SwapEvent => swap_event::SwapEvent,
        )
    }
}
