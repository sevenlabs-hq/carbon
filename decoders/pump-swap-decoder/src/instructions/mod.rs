use crate::PROGRAM_ID;

use super::PumpSwapDecoder;
pub mod buy;
pub mod buy_event;
pub mod collect_coin_creator_fee;
pub mod collect_coin_creator_fee_event;
pub mod create_config;
pub mod create_config_event;
pub mod create_pool;
pub mod create_pool_event;
pub mod deposit;
pub mod deposit_event;
pub mod disable;
pub mod disable_event;
pub mod extend_account;
pub mod extend_account_event;
pub mod sell;
pub mod sell_event;
pub mod set_bonding_curve_coin_creator_event;
pub mod set_coin_creator;
pub mod set_metaplex_coin_creator_event;
pub mod update_admin;
pub mod update_admin_event;
pub mod update_fee_config;
pub mod update_fee_config_event;
pub mod withdraw;
pub mod withdraw_event;

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
pub enum PumpSwapInstruction {
    Buy(buy::Buy),
    CollectCoinCreatorFee(collect_coin_creator_fee::CollectCoinCreatorFee),
    CreateConfig(create_config::CreateConfig),
    CreatePool(create_pool::CreatePool),
    Deposit(deposit::Deposit),
    Disable(disable::Disable),
    ExtendAccount(extend_account::ExtendAccount),
    Sell(sell::Sell),
    SetCoinCreator(set_coin_creator::SetCoinCreator),
    UpdateAdmin(update_admin::UpdateAdmin),
    UpdateFeeConfig(update_fee_config::UpdateFeeConfig),
    Withdraw(withdraw::Withdraw),
    BuyEvent(buy_event::BuyEvent),
    CollectCoinCreatorFeeEvent(collect_coin_creator_fee_event::CollectCoinCreatorFeeEvent),
    CreateConfigEvent(create_config_event::CreateConfigEvent),
    CreatePoolEvent(create_pool_event::CreatePoolEvent),
    DepositEvent(deposit_event::DepositEvent),
    DisableEvent(disable_event::DisableEvent),
    ExtendAccountEvent(extend_account_event::ExtendAccountEvent),
    SellEvent(sell_event::SellEvent),
    SetBondingCurveCoinCreatorEvent(
        set_bonding_curve_coin_creator_event::SetBondingCurveCoinCreatorEvent,
    ),
    SetMetaplexCoinCreatorEvent(set_metaplex_coin_creator_event::SetMetaplexCoinCreatorEvent),
    UpdateAdminEvent(update_admin_event::UpdateAdminEvent),
    UpdateFeeConfigEvent(update_fee_config_event::UpdateFeeConfigEvent),
    WithdrawEvent(withdraw_event::WithdrawEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for PumpSwapDecoder {
    type InstructionType = PumpSwapInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }
        carbon_core::try_decode_instructions!(instruction,
            PumpSwapInstruction::Buy => buy::Buy,
            PumpSwapInstruction::CollectCoinCreatorFee => collect_coin_creator_fee::CollectCoinCreatorFee,
            PumpSwapInstruction::CreateConfig => create_config::CreateConfig,
            PumpSwapInstruction::CreatePool => create_pool::CreatePool,
            PumpSwapInstruction::Deposit => deposit::Deposit,
            PumpSwapInstruction::Disable => disable::Disable,
            PumpSwapInstruction::ExtendAccount => extend_account::ExtendAccount,
            PumpSwapInstruction::Sell => sell::Sell,
            PumpSwapInstruction::SetCoinCreator => set_coin_creator::SetCoinCreator,
            PumpSwapInstruction::UpdateAdmin => update_admin::UpdateAdmin,
            PumpSwapInstruction::UpdateFeeConfig => update_fee_config::UpdateFeeConfig,
            PumpSwapInstruction::Withdraw => withdraw::Withdraw,
            PumpSwapInstruction::BuyEvent => buy_event::BuyEvent,
            PumpSwapInstruction::CollectCoinCreatorFeeEvent => collect_coin_creator_fee_event::CollectCoinCreatorFeeEvent,
            PumpSwapInstruction::CreateConfigEvent => create_config_event::CreateConfigEvent,
            PumpSwapInstruction::CreatePoolEvent => create_pool_event::CreatePoolEvent,
            PumpSwapInstruction::DepositEvent => deposit_event::DepositEvent,
            PumpSwapInstruction::DisableEvent => disable_event::DisableEvent,
            PumpSwapInstruction::ExtendAccountEvent => extend_account_event::ExtendAccountEvent,
            PumpSwapInstruction::SellEvent => sell_event::SellEvent,
            PumpSwapInstruction::SetBondingCurveCoinCreatorEvent => set_bonding_curve_coin_creator_event::SetBondingCurveCoinCreatorEvent,
            PumpSwapInstruction::SetMetaplexCoinCreatorEvent => set_metaplex_coin_creator_event::SetMetaplexCoinCreatorEvent,
            PumpSwapInstruction::UpdateAdminEvent => update_admin_event::UpdateAdminEvent,
            PumpSwapInstruction::UpdateFeeConfigEvent => update_fee_config_event::UpdateFeeConfigEvent,
            PumpSwapInstruction::WithdrawEvent => withdraw_event::WithdrawEvent,
        )
    }
}
