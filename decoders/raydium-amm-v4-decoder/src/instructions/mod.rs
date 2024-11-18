use super::RaydiumAmmV4Decoder;
pub mod admin_cancel_orders;
pub mod create_config_account;
pub mod deposit;
pub mod initialize;
pub mod initialize2;
pub mod migrate_to_open_book;
pub mod monitor_step;
pub mod pre_initialize;
pub mod set_params;
pub mod simulate_info;
pub mod swap_base_in;
pub mod swap_base_out;
pub mod update_config_account;
pub mod withdraw;
pub mod withdraw_pnl;
pub mod withdraw_srm;

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
pub enum RaydiumAmmV4Instruction {
    Initialize(initialize::Initialize),
    Initialize2(initialize2::Initialize2),
    MonitorStep(monitor_step::MonitorStep),
    Deposit(deposit::Deposit),
    Withdraw(withdraw::Withdraw),
    MigrateToOpenBook(migrate_to_open_book::MigrateToOpenBook),
    SetParams(set_params::SetParams),
    WithdrawPnl(withdraw_pnl::WithdrawPnl),
    WithdrawSrm(withdraw_srm::WithdrawSrm),
    SwapBaseIn(swap_base_in::SwapBaseIn),
    PreInitialize(pre_initialize::PreInitialize),
    SwapBaseOut(swap_base_out::SwapBaseOut),
    SimulateInfo(simulate_info::SimulateInfo),
    AdminCancelOrders(admin_cancel_orders::AdminCancelOrders),
    CreateConfigAccount(create_config_account::CreateConfigAccount),
    UpdateConfigAccount(update_config_account::UpdateConfigAccount),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for RaydiumAmmV4Decoder {
    type InstructionType = RaydiumAmmV4Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            RaydiumAmmV4Instruction::Initialize => initialize::Initialize,
            RaydiumAmmV4Instruction::Initialize2 => initialize2::Initialize2,
            RaydiumAmmV4Instruction::MonitorStep => monitor_step::MonitorStep,
            RaydiumAmmV4Instruction::Deposit => deposit::Deposit,
            RaydiumAmmV4Instruction::Withdraw => withdraw::Withdraw,
            RaydiumAmmV4Instruction::MigrateToOpenBook => migrate_to_open_book::MigrateToOpenBook,
            RaydiumAmmV4Instruction::SetParams => set_params::SetParams,
            RaydiumAmmV4Instruction::WithdrawPnl => withdraw_pnl::WithdrawPnl,
            RaydiumAmmV4Instruction::WithdrawSrm => withdraw_srm::WithdrawSrm,
            RaydiumAmmV4Instruction::SwapBaseIn => swap_base_in::SwapBaseIn,
            RaydiumAmmV4Instruction::PreInitialize => pre_initialize::PreInitialize,
            RaydiumAmmV4Instruction::SwapBaseOut => swap_base_out::SwapBaseOut,
            RaydiumAmmV4Instruction::SimulateInfo => simulate_info::SimulateInfo,
            RaydiumAmmV4Instruction::AdminCancelOrders => admin_cancel_orders::AdminCancelOrders,
            RaydiumAmmV4Instruction::CreateConfigAccount => create_config_account::CreateConfigAccount,
            RaydiumAmmV4Instruction::UpdateConfigAccount => update_config_account::UpdateConfigAccount,
        )
    }
}
