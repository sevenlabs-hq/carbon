use {super::KaminoLimitOrderDecoder, crate::PROGRAM_ID};
pub mod close_order_and_claim_tip;
pub mod create_order;
pub mod flash_take_order_end;
pub mod flash_take_order_start;
pub mod initialize_global_config;
pub mod initialize_vault;
pub mod log_user_swap_balances;
pub mod order_display_event;
pub mod take_order;
pub mod update_global_config;
pub mod update_global_config_admin;
pub mod user_swap_balances_event;
pub mod withdraw_host_tip;

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
pub enum KaminoLimitOrderInstruction {
    InitializeGlobalConfig(initialize_global_config::InitializeGlobalConfig),
    InitializeVault(initialize_vault::InitializeVault),
    CreateOrder(create_order::CreateOrder),
    CloseOrderAndClaimTip(close_order_and_claim_tip::CloseOrderAndClaimTip),
    TakeOrder(take_order::TakeOrder),
    FlashTakeOrderStart(flash_take_order_start::FlashTakeOrderStart),
    FlashTakeOrderEnd(flash_take_order_end::FlashTakeOrderEnd),
    UpdateGlobalConfig(update_global_config::UpdateGlobalConfig),
    UpdateGlobalConfigAdmin(update_global_config_admin::UpdateGlobalConfigAdmin),
    WithdrawHostTip(withdraw_host_tip::WithdrawHostTip),
    LogUserSwapBalances(log_user_swap_balances::LogUserSwapBalances),
    OrderDisplayEvent(order_display_event::OrderDisplayEvent),
    UserSwapBalancesEvent(user_swap_balances_event::UserSwapBalancesEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for KaminoLimitOrderDecoder {
    type InstructionType = KaminoLimitOrderInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }
        carbon_core::try_decode_instructions!(instruction,
            KaminoLimitOrderInstruction::InitializeGlobalConfig => initialize_global_config::InitializeGlobalConfig,
            KaminoLimitOrderInstruction::InitializeVault => initialize_vault::InitializeVault,
            KaminoLimitOrderInstruction::CreateOrder => create_order::CreateOrder,
            KaminoLimitOrderInstruction::CloseOrderAndClaimTip => close_order_and_claim_tip::CloseOrderAndClaimTip,
            KaminoLimitOrderInstruction::TakeOrder => take_order::TakeOrder,
            KaminoLimitOrderInstruction::FlashTakeOrderStart => flash_take_order_start::FlashTakeOrderStart,
            KaminoLimitOrderInstruction::FlashTakeOrderEnd => flash_take_order_end::FlashTakeOrderEnd,
            KaminoLimitOrderInstruction::UpdateGlobalConfig => update_global_config::UpdateGlobalConfig,
            KaminoLimitOrderInstruction::UpdateGlobalConfigAdmin => update_global_config_admin::UpdateGlobalConfigAdmin,
            KaminoLimitOrderInstruction::WithdrawHostTip => withdraw_host_tip::WithdrawHostTip,
            KaminoLimitOrderInstruction::LogUserSwapBalances => log_user_swap_balances::LogUserSwapBalances,
            KaminoLimitOrderInstruction::OrderDisplayEvent => order_display_event::OrderDisplayEvent,
            KaminoLimitOrderInstruction::UserSwapBalancesEvent => user_swap_balances_event::UserSwapBalancesEvent,
        )
    }
}
