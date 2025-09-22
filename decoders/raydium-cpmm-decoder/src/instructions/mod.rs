use crate::PROGRAM_ID;

use super::RaydiumCpmmDecoder;
pub mod close_permission_pda;
pub mod collect_creator_fee;
pub mod collect_fund_fee;
pub mod collect_protocol_fee;
pub mod create_amm_config;
pub mod create_permission_pda;
pub mod deposit;
pub mod initialize;
pub mod initialize_with_permission;
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
    ClosePermissionPda(close_permission_pda::ClosePermissionPda),
    CollectCreatorFee(collect_creator_fee::CollectCreatorFee),
    CollectFundFee(collect_fund_fee::CollectFundFee),
    CollectProtocolFee(collect_protocol_fee::CollectProtocolFee),
    CreateAmmConfig(create_amm_config::CreateAmmConfig),
    CreatePermissionPda(create_permission_pda::CreatePermissionPda),
    Deposit(deposit::Deposit),
    Initialize(initialize::Initialize),
    InitializeWithPermission(initialize_with_permission::InitializeWithPermission),
    SwapBaseInput(swap_base_input::SwapBaseInput),
    SwapBaseOutput(swap_base_output::SwapBaseOutput),
    UpdateAmmConfig(update_amm_config::UpdateAmmConfig),
    UpdatePoolStatus(update_pool_status::UpdatePoolStatus),
    Withdraw(withdraw::Withdraw),
    LpChangeEvent(lp_change_event::LpChangeEvent),
    SwapEvent(swap_event::SwapEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for RaydiumCpmmDecoder {
    type InstructionType = RaydiumCpmmInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            RaydiumCpmmInstruction::ClosePermissionPda => close_permission_pda::ClosePermissionPda,
            RaydiumCpmmInstruction::CollectCreatorFee => collect_creator_fee::CollectCreatorFee,
            RaydiumCpmmInstruction::CollectFundFee => collect_fund_fee::CollectFundFee,
            RaydiumCpmmInstruction::CollectProtocolFee => collect_protocol_fee::CollectProtocolFee,
            RaydiumCpmmInstruction::CreateAmmConfig => create_amm_config::CreateAmmConfig,
            RaydiumCpmmInstruction::CreatePermissionPda => create_permission_pda::CreatePermissionPda,
            RaydiumCpmmInstruction::Deposit => deposit::Deposit,
            RaydiumCpmmInstruction::Initialize => initialize::Initialize,
            RaydiumCpmmInstruction::InitializeWithPermission => initialize_with_permission::InitializeWithPermission,
            RaydiumCpmmInstruction::SwapBaseInput => swap_base_input::SwapBaseInput,
            RaydiumCpmmInstruction::SwapBaseOutput => swap_base_output::SwapBaseOutput,
            RaydiumCpmmInstruction::UpdateAmmConfig => update_amm_config::UpdateAmmConfig,
            RaydiumCpmmInstruction::UpdatePoolStatus => update_pool_status::UpdatePoolStatus,
            RaydiumCpmmInstruction::Withdraw => withdraw::Withdraw,
            RaydiumCpmmInstruction::LpChangeEvent => lp_change_event::LpChangeEvent,
            RaydiumCpmmInstruction::SwapEvent => swap_event::SwapEvent,
        )
    }
}
