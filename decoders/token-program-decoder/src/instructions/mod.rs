use crate::TokenProgramDecoder;

pub mod amount_to_ui_amount;
pub mod approve;
pub mod approve_checked;
pub mod burn;
pub mod burn_checked;
pub mod close_account;
pub mod freeze_account;
pub mod get_account_data_size;
pub mod initialize_account;
pub mod initialize_account2;
pub mod initialize_account3;
pub mod initialize_immutable_owner;
pub mod initialize_mint;
pub mod initialize_mint2;
pub mod initialize_multisig;
pub mod initialize_multisig2;
pub mod mint_to;
pub mod mint_to_checked;
pub mod revoke;
pub mod set_authority;
pub mod sync_native;
pub mod thaw_account;
pub mod transfer;
pub mod transfer_checked;
pub mod ui_amount_to_amount;

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
pub enum TokenProgramInstruction {
    AmountToUiAmount(amount_to_ui_amount::AmountToUiAmount),
    ApproveChecked(approve_checked::ApproveChecked),
    Approve(approve::Approve),
    BurnChecked(burn_checked::BurnChecked),
    Burn(burn::Burn),
    CloseAccount(close_account::CloseAccount),
    FreezeAccount(freeze_account::FreezeAccount),
    GetAccountDataSize(get_account_data_size::GetAccountDataSize),
    InitializeAccount(initialize_account::InitializeAccount),
    InitializeAccount2(initialize_account2::InitializeAccount2),
    InitializeAccount3(initialize_account3::InitializeAccount3),
    InitializeImmutableOwner(initialize_immutable_owner::InitializeImmutableOwner),
    InitializeMint(initialize_mint::InitializeMint),
    InitializeMint2(initialize_mint2::InitializeMint2),
    InitializeMultisig(initialize_multisig::InitializeMultisig),
    InitializeMultisig2(initialize_multisig2::InitializeMultisig2),
    MintToChecked(mint_to_checked::MintToChecked),
    MintTo(mint_to::MintTo),
    Revoke(revoke::Revoke),
    SetAuthority(set_authority::SetAuthority),
    SyncNative(sync_native::SyncNative),
    ThawAccount(thaw_account::ThawAccount),
    TransferChecked(transfer_checked::TransferChecked),
    Transfer(transfer::Transfer),
    UiAmountToAmount(ui_amount_to_amount::UiAmountToAmount),
}

impl carbon_core::instruction::InstructionDecoder<'_> for TokenProgramDecoder {
    type InstructionType = TokenProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&spl_token_interface::id()) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            TokenProgramInstruction::AmountToUiAmount => amount_to_ui_amount::AmountToUiAmount,
            TokenProgramInstruction::ApproveChecked => approve_checked::ApproveChecked,
            TokenProgramInstruction::Approve => approve::Approve,
            TokenProgramInstruction::BurnChecked => burn_checked::BurnChecked,
            TokenProgramInstruction::Burn => burn::Burn,
            TokenProgramInstruction::CloseAccount => close_account::CloseAccount,
            TokenProgramInstruction::FreezeAccount => freeze_account::FreezeAccount,
            TokenProgramInstruction::GetAccountDataSize => get_account_data_size::GetAccountDataSize,
            TokenProgramInstruction::InitializeAccount => initialize_account::InitializeAccount,
            TokenProgramInstruction::InitializeAccount2 => initialize_account2::InitializeAccount2,
            TokenProgramInstruction::InitializeAccount3 => initialize_account3::InitializeAccount3,
            TokenProgramInstruction::InitializeImmutableOwner => initialize_immutable_owner::InitializeImmutableOwner,
            TokenProgramInstruction::InitializeMint => initialize_mint::InitializeMint,
            TokenProgramInstruction::InitializeMint2 => initialize_mint2::InitializeMint2,
            TokenProgramInstruction::InitializeMultisig => initialize_multisig::InitializeMultisig,
            TokenProgramInstruction::InitializeMultisig2 => initialize_multisig2::InitializeMultisig2,
            TokenProgramInstruction::MintToChecked => mint_to_checked::MintToChecked,
            TokenProgramInstruction::MintTo => mint_to::MintTo,
            TokenProgramInstruction::Revoke => revoke::Revoke,
            TokenProgramInstruction::SetAuthority => set_authority::SetAuthority,
            TokenProgramInstruction::SyncNative => sync_native::SyncNative,
            TokenProgramInstruction::ThawAccount => thaw_account::ThawAccount,
            TokenProgramInstruction::TransferChecked => transfer_checked::TransferChecked,
            TokenProgramInstruction::Transfer => transfer::Transfer,
            TokenProgramInstruction::UiAmountToAmount => ui_amount_to_amount::UiAmountToAmount,
        )
    }
}
