use crate::PROGRAM_ID;

use super::Token2022Decoder;
pub mod amount_to_ui_amount;
pub mod apply_confidential_pending_balance;
pub mod approve;
pub mod approve_checked;
pub mod approve_confidential_transfer_account;
pub mod burn;
pub mod burn_checked;
pub mod close_account;
pub mod confidential_deposit;
pub mod confidential_transfer;
pub mod confidential_transfer_with_fee;
pub mod confidential_withdraw;
pub mod configure_confidential_transfer_account;
pub mod create_native_mint;
pub mod disable_confidential_credits;
pub mod disable_cpi_guard;
pub mod disable_harvest_to_mint;
pub mod disable_memo_transfers;
pub mod disable_non_confidential_credits;
pub mod emit_token_metadata;
pub mod empty_confidential_transfer_account;
pub mod enable_confidential_credits;
pub mod enable_cpi_guard;
pub mod enable_harvest_to_mint;
pub mod enable_memo_transfers;
pub mod enable_non_confidential_credits;
pub mod freeze_account;
pub mod get_account_data_size;
pub mod harvest_withheld_tokens_to_mint;
pub mod harvest_withheld_tokens_to_mint_for_confidential_transfer_fee;
pub mod initialize_account;
pub mod initialize_account2;
pub mod initialize_account3;
pub mod initialize_confidential_transfer_fee;
pub mod initialize_confidential_transfer_mint;
pub mod initialize_default_account_state;
pub mod initialize_group_member_pointer;
pub mod initialize_group_pointer;
pub mod initialize_immutable_owner;
pub mod initialize_interest_bearing_mint;
pub mod initialize_metadata_pointer;
pub mod initialize_mint;
pub mod initialize_mint2;
pub mod initialize_mint_close_authority;
pub mod initialize_mint_pausable;
pub mod initialize_multisig;
pub mod initialize_multisig2;
pub mod initialize_non_transferable_mint;
pub mod initialize_permanent_delegate;
pub mod initialize_scaled_ui_amount;
pub mod initialize_token_group;
pub mod initialize_token_group_member;
pub mod initialize_token_metadata;
pub mod initialize_transfer_fee_config;
pub mod initialize_transfer_hook;
pub mod mint_to;
pub mod mint_to_checked;
pub mod reallocate;
pub mod remove_token_metadata_key;
pub mod revoke;
pub mod set_authority;
pub mod set_transfer_fee;
pub mod sync_native;
pub mod thaw_account;
pub mod toggle_mint_pause;
pub mod transfer;
pub mod transfer_checked;
pub mod transfer_checked_with_fee;
pub mod ui_amount_to_amount;
pub mod update_confidential_transfer_mint;
pub mod update_default_account_state;
pub mod update_group_member_pointer;
pub mod update_group_pointer;
pub mod update_metadata_pointer;
pub mod update_multiplier_scaled_ui_amount;
pub mod update_rate_interest_bearing_mint;
pub mod update_token_group_max_size;
pub mod update_token_group_update_authority;
pub mod update_token_metadata_field;
pub mod update_token_metadata_update_authority;
pub mod update_transfer_hook;
pub mod withdraw_excess_lamports;
pub mod withdraw_withheld_tokens_from_accounts;
pub mod withdraw_withheld_tokens_from_accounts_for_confidential_transfer_fee;
pub mod withdraw_withheld_tokens_from_mint;
pub mod withdraw_withheld_tokens_from_mint_for_confidential_transfer_fee;

#[derive(
    carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone,
)]
pub enum Token2022Instruction {
    InitializeMint(initialize_mint::InitializeMint),
    InitializeAccount(initialize_account::InitializeAccount),
    InitializeMultisig(initialize_multisig::InitializeMultisig),
    Transfer(transfer::Transfer),
    Approve(approve::Approve),
    Revoke(revoke::Revoke),
    SetAuthority(set_authority::SetAuthority),
    MintTo(mint_to::MintTo),
    Burn(burn::Burn),
    CloseAccount(close_account::CloseAccount),
    FreezeAccount(freeze_account::FreezeAccount),
    ThawAccount(thaw_account::ThawAccount),
    TransferChecked(transfer_checked::TransferChecked),
    ApproveChecked(approve_checked::ApproveChecked),
    MintToChecked(mint_to_checked::MintToChecked),
    BurnChecked(burn_checked::BurnChecked),
    InitializeAccount2(initialize_account2::InitializeAccount2),
    SyncNative(sync_native::SyncNative),
    InitializeAccount3(initialize_account3::InitializeAccount3),
    InitializeMultisig2(initialize_multisig2::InitializeMultisig2),
    InitializeMint2(initialize_mint2::InitializeMint2),
    GetAccountDataSize(get_account_data_size::GetAccountDataSize),
    InitializeImmutableOwner(initialize_immutable_owner::InitializeImmutableOwner),
    AmountToUiAmount(amount_to_ui_amount::AmountToUiAmount),
    UiAmountToAmount(ui_amount_to_amount::UiAmountToAmount),
    InitializeMintCloseAuthority(initialize_mint_close_authority::InitializeMintCloseAuthority),
    InitializeTransferFeeConfig(initialize_transfer_fee_config::InitializeTransferFeeConfig),
    TransferCheckedWithFee(transfer_checked_with_fee::TransferCheckedWithFee),
    WithdrawWithheldTokensFromMint(withdraw_withheld_tokens_from_mint::WithdrawWithheldTokensFromMint),
    WithdrawWithheldTokensFromAccounts(withdraw_withheld_tokens_from_accounts::WithdrawWithheldTokensFromAccounts),
    HarvestWithheldTokensToMint(harvest_withheld_tokens_to_mint::HarvestWithheldTokensToMint),
    SetTransferFee(set_transfer_fee::SetTransferFee),
    InitializeConfidentialTransferMint(initialize_confidential_transfer_mint::InitializeConfidentialTransferMint),
    UpdateConfidentialTransferMint(update_confidential_transfer_mint::UpdateConfidentialTransferMint),
    ConfigureConfidentialTransferAccount(configure_confidential_transfer_account::ConfigureConfidentialTransferAccount),
    ApproveConfidentialTransferAccount(approve_confidential_transfer_account::ApproveConfidentialTransferAccount),
    EmptyConfidentialTransferAccount(empty_confidential_transfer_account::EmptyConfidentialTransferAccount),
    ConfidentialDeposit(confidential_deposit::ConfidentialDeposit),
    ConfidentialWithdraw(confidential_withdraw::ConfidentialWithdraw),
    ConfidentialTransfer(confidential_transfer::ConfidentialTransfer),
    ApplyConfidentialPendingBalance(apply_confidential_pending_balance::ApplyConfidentialPendingBalance),
    EnableConfidentialCredits(enable_confidential_credits::EnableConfidentialCredits),
    DisableConfidentialCredits(disable_confidential_credits::DisableConfidentialCredits),
    EnableNonConfidentialCredits(enable_non_confidential_credits::EnableNonConfidentialCredits),
    DisableNonConfidentialCredits(disable_non_confidential_credits::DisableNonConfidentialCredits),
    ConfidentialTransferWithFee(confidential_transfer_with_fee::ConfidentialTransferWithFee),
    InitializeDefaultAccountState(initialize_default_account_state::InitializeDefaultAccountState),
    UpdateDefaultAccountState(update_default_account_state::UpdateDefaultAccountState),
    Reallocate(reallocate::Reallocate),
    EnableMemoTransfers(enable_memo_transfers::EnableMemoTransfers),
    DisableMemoTransfers(disable_memo_transfers::DisableMemoTransfers),
    CreateNativeMint(create_native_mint::CreateNativeMint),
    InitializeNonTransferableMint(initialize_non_transferable_mint::InitializeNonTransferableMint),
    InitializeInterestBearingMint(initialize_interest_bearing_mint::InitializeInterestBearingMint),
    UpdateRateInterestBearingMint(update_rate_interest_bearing_mint::UpdateRateInterestBearingMint),
    EnableCpiGuard(enable_cpi_guard::EnableCpiGuard),
    DisableCpiGuard(disable_cpi_guard::DisableCpiGuard),
    InitializePermanentDelegate(initialize_permanent_delegate::InitializePermanentDelegate),
    InitializeTransferHook(initialize_transfer_hook::InitializeTransferHook),
    UpdateTransferHook(update_transfer_hook::UpdateTransferHook),
    InitializeConfidentialTransferFee(initialize_confidential_transfer_fee::InitializeConfidentialTransferFee),
    WithdrawWithheldTokensFromMintForConfidentialTransferFee(withdraw_withheld_tokens_from_mint_for_confidential_transfer_fee::WithdrawWithheldTokensFromMintForConfidentialTransferFee),
    WithdrawWithheldTokensFromAccountsForConfidentialTransferFee(withdraw_withheld_tokens_from_accounts_for_confidential_transfer_fee::WithdrawWithheldTokensFromAccountsForConfidentialTransferFee),
    HarvestWithheldTokensToMintForConfidentialTransferFee(harvest_withheld_tokens_to_mint_for_confidential_transfer_fee::HarvestWithheldTokensToMintForConfidentialTransferFee),
    EnableHarvestToMint(enable_harvest_to_mint::EnableHarvestToMint),
    DisableHarvestToMint(disable_harvest_to_mint::DisableHarvestToMint),
    WithdrawExcessLamports(withdraw_excess_lamports::WithdrawExcessLamports),
    InitializeMetadataPointer(initialize_metadata_pointer::InitializeMetadataPointer),
    UpdateMetadataPointer(update_metadata_pointer::UpdateMetadataPointer),
    InitializeGroupPointer(initialize_group_pointer::InitializeGroupPointer),
    UpdateGroupPointer(update_group_pointer::UpdateGroupPointer),
    InitializeGroupMemberPointer(initialize_group_member_pointer::InitializeGroupMemberPointer),
    UpdateGroupMemberPointer(update_group_member_pointer::UpdateGroupMemberPointer),
    InitializeTokenMetadata(initialize_token_metadata::InitializeTokenMetadata),
    UpdateTokenMetadataField(update_token_metadata_field::UpdateTokenMetadataField),
    RemoveTokenMetadataKey(remove_token_metadata_key::RemoveTokenMetadataKey),
    UpdateTokenMetadataUpdateAuthority(update_token_metadata_update_authority::UpdateTokenMetadataUpdateAuthority),
    EmitTokenMetadata(emit_token_metadata::EmitTokenMetadata),
    InitializeTokenGroup(initialize_token_group::InitializeTokenGroup),
    UpdateTokenGroupMaxSize(update_token_group_max_size::UpdateTokenGroupMaxSize),
    UpdateTokenGroupUpdateAuthority(update_token_group_update_authority::UpdateTokenGroupUpdateAuthority),
    InitializeTokenGroupMember(initialize_token_group_member::InitializeTokenGroupMember),
    InitializeScaledUiAmount(initialize_scaled_ui_amount::InitializedScaledUiAmount),
    UpdateMultiplierScaledUiAmount(update_multiplier_scaled_ui_amount::UpdateMultiplierScaledUiAmount),
    InitializeMintPausable(initialize_mint_pausable::InitializeMintPausable),
    ToggleMintPause(toggle_mint_pause::ToggleMintPause)
}

impl carbon_core::instruction::InstructionDecoder<'_> for Token2022Decoder {
    type InstructionType = Token2022Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            Token2022Instruction::InitializeMint => initialize_mint::InitializeMint,
            Token2022Instruction::InitializeAccount => initialize_account::InitializeAccount,
            Token2022Instruction::InitializeMultisig => initialize_multisig::InitializeMultisig,
            Token2022Instruction::Transfer => transfer::Transfer,
            Token2022Instruction::Approve => approve::Approve,
            Token2022Instruction::Revoke => revoke::Revoke,
            Token2022Instruction::SetAuthority => set_authority::SetAuthority,
            Token2022Instruction::MintTo => mint_to::MintTo,
            Token2022Instruction::Burn => burn::Burn,
            Token2022Instruction::CloseAccount => close_account::CloseAccount,
            Token2022Instruction::FreezeAccount => freeze_account::FreezeAccount,
            Token2022Instruction::ThawAccount => thaw_account::ThawAccount,
            Token2022Instruction::TransferChecked => transfer_checked::TransferChecked,
            Token2022Instruction::ApproveChecked => approve_checked::ApproveChecked,
            Token2022Instruction::MintToChecked => mint_to_checked::MintToChecked,
            Token2022Instruction::BurnChecked => burn_checked::BurnChecked,
            Token2022Instruction::InitializeAccount2 => initialize_account2::InitializeAccount2,
            Token2022Instruction::SyncNative => sync_native::SyncNative,
            Token2022Instruction::InitializeAccount3 => initialize_account3::InitializeAccount3,
            Token2022Instruction::InitializeMultisig2 => initialize_multisig2::InitializeMultisig2,
            Token2022Instruction::InitializeMint2 => initialize_mint2::InitializeMint2,
            Token2022Instruction::GetAccountDataSize => get_account_data_size::GetAccountDataSize,
            Token2022Instruction::InitializeImmutableOwner => initialize_immutable_owner::InitializeImmutableOwner,
            Token2022Instruction::AmountToUiAmount => amount_to_ui_amount::AmountToUiAmount,
            Token2022Instruction::UiAmountToAmount => ui_amount_to_amount::UiAmountToAmount,
            Token2022Instruction::InitializeMintCloseAuthority => initialize_mint_close_authority::InitializeMintCloseAuthority,
            Token2022Instruction::InitializeTransferFeeConfig => initialize_transfer_fee_config::InitializeTransferFeeConfig,
            Token2022Instruction::TransferCheckedWithFee => transfer_checked_with_fee::TransferCheckedWithFee,
            Token2022Instruction::WithdrawWithheldTokensFromMint => withdraw_withheld_tokens_from_mint::WithdrawWithheldTokensFromMint,
            Token2022Instruction::WithdrawWithheldTokensFromAccounts => withdraw_withheld_tokens_from_accounts::WithdrawWithheldTokensFromAccounts,
            Token2022Instruction::HarvestWithheldTokensToMint => harvest_withheld_tokens_to_mint::HarvestWithheldTokensToMint,
            Token2022Instruction::SetTransferFee => set_transfer_fee::SetTransferFee,
            Token2022Instruction::InitializeConfidentialTransferMint => initialize_confidential_transfer_mint::InitializeConfidentialTransferMint,
            Token2022Instruction::UpdateConfidentialTransferMint => update_confidential_transfer_mint::UpdateConfidentialTransferMint,
            Token2022Instruction::ConfigureConfidentialTransferAccount => configure_confidential_transfer_account::ConfigureConfidentialTransferAccount,
            Token2022Instruction::ApproveConfidentialTransferAccount => approve_confidential_transfer_account::ApproveConfidentialTransferAccount,
            Token2022Instruction::EmptyConfidentialTransferAccount => empty_confidential_transfer_account::EmptyConfidentialTransferAccount,
            Token2022Instruction::ConfidentialDeposit => confidential_deposit::ConfidentialDeposit,
            Token2022Instruction::ConfidentialWithdraw => confidential_withdraw::ConfidentialWithdraw,
            Token2022Instruction::ConfidentialTransfer => confidential_transfer::ConfidentialTransfer,
            Token2022Instruction::ApplyConfidentialPendingBalance => apply_confidential_pending_balance::ApplyConfidentialPendingBalance,
            Token2022Instruction::EnableConfidentialCredits => enable_confidential_credits::EnableConfidentialCredits,
            Token2022Instruction::DisableConfidentialCredits => disable_confidential_credits::DisableConfidentialCredits,
            Token2022Instruction::EnableNonConfidentialCredits => enable_non_confidential_credits::EnableNonConfidentialCredits,
            Token2022Instruction::DisableNonConfidentialCredits => disable_non_confidential_credits::DisableNonConfidentialCredits,
            Token2022Instruction::ConfidentialTransferWithFee => confidential_transfer_with_fee::ConfidentialTransferWithFee,
            Token2022Instruction::InitializeDefaultAccountState => initialize_default_account_state::InitializeDefaultAccountState,
            Token2022Instruction::UpdateDefaultAccountState => update_default_account_state::UpdateDefaultAccountState,
            Token2022Instruction::Reallocate => reallocate::Reallocate,
            Token2022Instruction::EnableMemoTransfers => enable_memo_transfers::EnableMemoTransfers,
            Token2022Instruction::DisableMemoTransfers => disable_memo_transfers::DisableMemoTransfers,
            Token2022Instruction::CreateNativeMint => create_native_mint::CreateNativeMint,
            Token2022Instruction::InitializeNonTransferableMint => initialize_non_transferable_mint::InitializeNonTransferableMint,
            Token2022Instruction::InitializeInterestBearingMint => initialize_interest_bearing_mint::InitializeInterestBearingMint,
            Token2022Instruction::UpdateRateInterestBearingMint => update_rate_interest_bearing_mint::UpdateRateInterestBearingMint,
            Token2022Instruction::EnableCpiGuard => enable_cpi_guard::EnableCpiGuard,
            Token2022Instruction::DisableCpiGuard => disable_cpi_guard::DisableCpiGuard,
            Token2022Instruction::InitializePermanentDelegate => initialize_permanent_delegate::InitializePermanentDelegate,
            Token2022Instruction::InitializeTransferHook => initialize_transfer_hook::InitializeTransferHook,
            Token2022Instruction::UpdateTransferHook => update_transfer_hook::UpdateTransferHook,
            Token2022Instruction::InitializeConfidentialTransferFee => initialize_confidential_transfer_fee::InitializeConfidentialTransferFee,
            Token2022Instruction::WithdrawWithheldTokensFromMintForConfidentialTransferFee => withdraw_withheld_tokens_from_mint_for_confidential_transfer_fee::WithdrawWithheldTokensFromMintForConfidentialTransferFee,
            Token2022Instruction::WithdrawWithheldTokensFromAccountsForConfidentialTransferFee => withdraw_withheld_tokens_from_accounts_for_confidential_transfer_fee::WithdrawWithheldTokensFromAccountsForConfidentialTransferFee,
            Token2022Instruction::HarvestWithheldTokensToMintForConfidentialTransferFee => harvest_withheld_tokens_to_mint_for_confidential_transfer_fee::HarvestWithheldTokensToMintForConfidentialTransferFee,
            Token2022Instruction::EnableHarvestToMint => enable_harvest_to_mint::EnableHarvestToMint,
            Token2022Instruction::DisableHarvestToMint => disable_harvest_to_mint::DisableHarvestToMint,
            Token2022Instruction::WithdrawExcessLamports => withdraw_excess_lamports::WithdrawExcessLamports,
            Token2022Instruction::InitializeMetadataPointer => initialize_metadata_pointer::InitializeMetadataPointer,
            Token2022Instruction::UpdateMetadataPointer => update_metadata_pointer::UpdateMetadataPointer,
            Token2022Instruction::InitializeGroupPointer => initialize_group_pointer::InitializeGroupPointer,
            Token2022Instruction::UpdateGroupPointer => update_group_pointer::UpdateGroupPointer,
            Token2022Instruction::InitializeGroupMemberPointer => initialize_group_member_pointer::InitializeGroupMemberPointer,
            Token2022Instruction::UpdateGroupMemberPointer => update_group_member_pointer::UpdateGroupMemberPointer,
            Token2022Instruction::InitializeTokenMetadata => initialize_token_metadata::InitializeTokenMetadata,
            Token2022Instruction::UpdateTokenMetadataField => update_token_metadata_field::UpdateTokenMetadataField,
            Token2022Instruction::RemoveTokenMetadataKey => remove_token_metadata_key::RemoveTokenMetadataKey,
            Token2022Instruction::UpdateTokenMetadataUpdateAuthority => update_token_metadata_update_authority::UpdateTokenMetadataUpdateAuthority,
            Token2022Instruction::EmitTokenMetadata => emit_token_metadata::EmitTokenMetadata,
            Token2022Instruction::InitializeTokenGroup => initialize_token_group::InitializeTokenGroup,
            Token2022Instruction::UpdateTokenGroupMaxSize => update_token_group_max_size::UpdateTokenGroupMaxSize,
            Token2022Instruction::UpdateTokenGroupUpdateAuthority => update_token_group_update_authority::UpdateTokenGroupUpdateAuthority,
            Token2022Instruction::InitializeTokenGroupMember => initialize_token_group_member::InitializeTokenGroupMember,
            Token2022Instruction::InitializeScaledUiAmount => initialize_scaled_ui_amount::InitializedScaledUiAmount,
            Token2022Instruction::UpdateMultiplierScaledUiAmount => update_multiplier_scaled_ui_amount::UpdateMultiplierScaledUiAmount,
            Token2022Instruction::InitializeMintPausable => initialize_mint_pausable::InitializeMintPausable,
            Token2022Instruction::ToggleMintPause => toggle_mint_pause::ToggleMintPause
        )
    }
}
