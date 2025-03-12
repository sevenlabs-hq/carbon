



use super::MarginfiV2Decoder;
pub mod marginfi_group_initialize;
pub mod marginfi_group_configure;
pub mod lending_pool_add_bank;
pub mod lending_pool_add_bank_with_seed;
pub mod lending_pool_configure_bank;
pub mod lending_pool_setup_emissions;
pub mod lending_pool_update_emissions_parameters;
pub mod lending_pool_handle_bankruptcy;
pub mod marginfi_account_initialize;
pub mod lending_account_deposit;
pub mod lending_account_repay;
pub mod lending_account_withdraw;
pub mod lending_account_borrow;
pub mod lending_account_close_balance;
pub mod lending_account_withdraw_emissions;
pub mod lending_account_settle_emissions;
pub mod lending_account_liquidate;
pub mod lending_account_start_flashloan;
pub mod lending_account_end_flashloan;
pub mod lending_pool_accrue_bank_interest;
pub mod lending_pool_collect_bank_fees;
pub mod set_account_flag;
pub mod unset_account_flag;
pub mod set_new_account_authority;
pub mod marginfi_group_create_event;
pub mod marginfi_group_configure_event;
pub mod lending_pool_bank_create_event;
pub mod lending_pool_bank_configure_event;
pub mod lending_pool_bank_accrue_interest_event;
pub mod lending_pool_bank_collect_fees_event;
pub mod lending_pool_bank_handle_bankruptcy_event;
pub mod marginfi_account_create_event;
pub mod lending_account_deposit_event;
pub mod lending_account_repay_event;
pub mod lending_account_borrow_event;
pub mod lending_account_withdraw_event;
pub mod lending_account_liquidate_event;
pub mod marginfi_account_transfer_account_authority_event;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum MarginfiV2Instruction {
    MarginfiGroupInitialize(marginfi_group_initialize::MarginfiGroupInitialize),
    MarginfiGroupConfigure(marginfi_group_configure::MarginfiGroupConfigure),
    LendingPoolAddBank(lending_pool_add_bank::LendingPoolAddBank),
    LendingPoolAddBankWithSeed(lending_pool_add_bank_with_seed::LendingPoolAddBankWithSeed),
    LendingPoolConfigureBank(lending_pool_configure_bank::LendingPoolConfigureBank),
    LendingPoolSetupEmissions(lending_pool_setup_emissions::LendingPoolSetupEmissions),
    LendingPoolUpdateEmissionsParameters(lending_pool_update_emissions_parameters::LendingPoolUpdateEmissionsParameters),
    LendingPoolHandleBankruptcy(lending_pool_handle_bankruptcy::LendingPoolHandleBankruptcy),
    MarginfiAccountInitialize(marginfi_account_initialize::MarginfiAccountInitialize),
    LendingAccountDeposit(lending_account_deposit::LendingAccountDeposit),
    LendingAccountRepay(lending_account_repay::LendingAccountRepay),
    LendingAccountWithdraw(lending_account_withdraw::LendingAccountWithdraw),
    LendingAccountBorrow(lending_account_borrow::LendingAccountBorrow),
    LendingAccountCloseBalance(lending_account_close_balance::LendingAccountCloseBalance),
    LendingAccountWithdrawEmissions(lending_account_withdraw_emissions::LendingAccountWithdrawEmissions),
    LendingAccountSettleEmissions(lending_account_settle_emissions::LendingAccountSettleEmissions),
    LendingAccountLiquidate(lending_account_liquidate::LendingAccountLiquidate),
    LendingAccountStartFlashloan(lending_account_start_flashloan::LendingAccountStartFlashloan),
    LendingAccountEndFlashloan(lending_account_end_flashloan::LendingAccountEndFlashloan),
    LendingPoolAccrueBankInterest(lending_pool_accrue_bank_interest::LendingPoolAccrueBankInterest),
    LendingPoolCollectBankFees(lending_pool_collect_bank_fees::LendingPoolCollectBankFees),
    SetAccountFlag(set_account_flag::SetAccountFlag),
    UnsetAccountFlag(unset_account_flag::UnsetAccountFlag),
    SetNewAccountAuthority(set_new_account_authority::SetNewAccountAuthority),
    MarginfiGroupCreateEvent(marginfi_group_create_event::MarginfiGroupCreateEvent),
    MarginfiGroupConfigureEvent(marginfi_group_configure_event::MarginfiGroupConfigureEvent),
    LendingPoolBankCreateEvent(lending_pool_bank_create_event::LendingPoolBankCreateEvent),
    LendingPoolBankConfigureEvent(lending_pool_bank_configure_event::LendingPoolBankConfigureEvent),
    LendingPoolBankAccrueInterestEvent(lending_pool_bank_accrue_interest_event::LendingPoolBankAccrueInterestEvent),
    LendingPoolBankCollectFeesEvent(lending_pool_bank_collect_fees_event::LendingPoolBankCollectFeesEvent),
    LendingPoolBankHandleBankruptcyEvent(lending_pool_bank_handle_bankruptcy_event::LendingPoolBankHandleBankruptcyEvent),
    MarginfiAccountCreateEvent(marginfi_account_create_event::MarginfiAccountCreateEvent),
    LendingAccountDepositEvent(lending_account_deposit_event::LendingAccountDepositEvent),
    LendingAccountRepayEvent(lending_account_repay_event::LendingAccountRepayEvent),
    LendingAccountBorrowEvent(lending_account_borrow_event::LendingAccountBorrowEvent),
    LendingAccountWithdrawEvent(lending_account_withdraw_event::LendingAccountWithdrawEvent),
    LendingAccountLiquidateEvent(lending_account_liquidate_event::LendingAccountLiquidateEvent),
    MarginfiAccountTransferAccountAuthorityEvent(marginfi_account_transfer_account_authority_event::MarginfiAccountTransferAccountAuthorityEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for MarginfiV2Decoder {
    type InstructionType = MarginfiV2Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            MarginfiV2Instruction::MarginfiGroupInitialize => marginfi_group_initialize::MarginfiGroupInitialize,
            MarginfiV2Instruction::MarginfiGroupConfigure => marginfi_group_configure::MarginfiGroupConfigure,
            MarginfiV2Instruction::LendingPoolAddBank => lending_pool_add_bank::LendingPoolAddBank,
            MarginfiV2Instruction::LendingPoolAddBankWithSeed => lending_pool_add_bank_with_seed::LendingPoolAddBankWithSeed,
            MarginfiV2Instruction::LendingPoolConfigureBank => lending_pool_configure_bank::LendingPoolConfigureBank,
            MarginfiV2Instruction::LendingPoolSetupEmissions => lending_pool_setup_emissions::LendingPoolSetupEmissions,
            MarginfiV2Instruction::LendingPoolUpdateEmissionsParameters => lending_pool_update_emissions_parameters::LendingPoolUpdateEmissionsParameters,
            MarginfiV2Instruction::LendingPoolHandleBankruptcy => lending_pool_handle_bankruptcy::LendingPoolHandleBankruptcy,
            MarginfiV2Instruction::MarginfiAccountInitialize => marginfi_account_initialize::MarginfiAccountInitialize,
            MarginfiV2Instruction::LendingAccountDeposit => lending_account_deposit::LendingAccountDeposit,
            MarginfiV2Instruction::LendingAccountRepay => lending_account_repay::LendingAccountRepay,
            MarginfiV2Instruction::LendingAccountWithdraw => lending_account_withdraw::LendingAccountWithdraw,
            MarginfiV2Instruction::LendingAccountBorrow => lending_account_borrow::LendingAccountBorrow,
            MarginfiV2Instruction::LendingAccountCloseBalance => lending_account_close_balance::LendingAccountCloseBalance,
            MarginfiV2Instruction::LendingAccountWithdrawEmissions => lending_account_withdraw_emissions::LendingAccountWithdrawEmissions,
            MarginfiV2Instruction::LendingAccountSettleEmissions => lending_account_settle_emissions::LendingAccountSettleEmissions,
            MarginfiV2Instruction::LendingAccountLiquidate => lending_account_liquidate::LendingAccountLiquidate,
            MarginfiV2Instruction::LendingAccountStartFlashloan => lending_account_start_flashloan::LendingAccountStartFlashloan,
            MarginfiV2Instruction::LendingAccountEndFlashloan => lending_account_end_flashloan::LendingAccountEndFlashloan,
            MarginfiV2Instruction::LendingPoolAccrueBankInterest => lending_pool_accrue_bank_interest::LendingPoolAccrueBankInterest,
            MarginfiV2Instruction::LendingPoolCollectBankFees => lending_pool_collect_bank_fees::LendingPoolCollectBankFees,
            MarginfiV2Instruction::SetAccountFlag => set_account_flag::SetAccountFlag,
            MarginfiV2Instruction::UnsetAccountFlag => unset_account_flag::UnsetAccountFlag,
            MarginfiV2Instruction::SetNewAccountAuthority => set_new_account_authority::SetNewAccountAuthority,
            MarginfiV2Instruction::MarginfiGroupCreateEvent => marginfi_group_create_event::MarginfiGroupCreateEvent,
            MarginfiV2Instruction::MarginfiGroupConfigureEvent => marginfi_group_configure_event::MarginfiGroupConfigureEvent,
            MarginfiV2Instruction::LendingPoolBankCreateEvent => lending_pool_bank_create_event::LendingPoolBankCreateEvent,
            MarginfiV2Instruction::LendingPoolBankConfigureEvent => lending_pool_bank_configure_event::LendingPoolBankConfigureEvent,
            MarginfiV2Instruction::LendingPoolBankAccrueInterestEvent => lending_pool_bank_accrue_interest_event::LendingPoolBankAccrueInterestEvent,
            MarginfiV2Instruction::LendingPoolBankCollectFeesEvent => lending_pool_bank_collect_fees_event::LendingPoolBankCollectFeesEvent,
            MarginfiV2Instruction::LendingPoolBankHandleBankruptcyEvent => lending_pool_bank_handle_bankruptcy_event::LendingPoolBankHandleBankruptcyEvent,
            MarginfiV2Instruction::MarginfiAccountCreateEvent => marginfi_account_create_event::MarginfiAccountCreateEvent,
            MarginfiV2Instruction::LendingAccountDepositEvent => lending_account_deposit_event::LendingAccountDepositEvent,
            MarginfiV2Instruction::LendingAccountRepayEvent => lending_account_repay_event::LendingAccountRepayEvent,
            MarginfiV2Instruction::LendingAccountBorrowEvent => lending_account_borrow_event::LendingAccountBorrowEvent,
            MarginfiV2Instruction::LendingAccountWithdrawEvent => lending_account_withdraw_event::LendingAccountWithdrawEvent,
            MarginfiV2Instruction::LendingAccountLiquidateEvent => lending_account_liquidate_event::LendingAccountLiquidateEvent,
            MarginfiV2Instruction::MarginfiAccountTransferAccountAuthorityEvent => marginfi_account_transfer_account_authority_event::MarginfiAccountTransferAccountAuthorityEvent,
        )
    }
}