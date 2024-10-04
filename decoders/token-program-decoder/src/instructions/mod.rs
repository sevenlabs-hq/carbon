use carbon_core::{
    deserialize::CarbonDeserialize,
    instruction::{DecodedInstruction, InstructionDecoder},
};

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
    carbon_proc_macros::InstructionType,
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

impl InstructionDecoder for TokenProgramDecoder {
    type InstructionType = TokenProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) =
            amount_to_ui_amount::AmountToUiAmount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::AmountToUiAmount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            approve_checked::ApproveChecked::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::ApproveChecked(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            approve::Approve::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::Approve(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            burn_checked::BurnChecked::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::BurnChecked(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = burn::Burn::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::Burn(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            close_account::CloseAccount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::CloseAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            freeze_account::FreezeAccount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::FreezeAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            get_account_data_size::GetAccountDataSize::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::GetAccountDataSize(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_account::InitializeAccount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::InitializeAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_account2::InitializeAccount2::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::InitializeAccount2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_account3::InitializeAccount3::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::InitializeAccount3(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_immutable_owner::InitializeImmutableOwner::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::InitializeImmutableOwner(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_mint::InitializeMint::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::InitializeMint(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_mint2::InitializeMint2::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::InitializeMint2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_multisig::InitializeMultisig::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::InitializeMultisig(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_multisig2::InitializeMultisig2::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::InitializeMultisig2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            mint_to_checked::MintToChecked::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::MintToChecked(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = mint_to::MintTo::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::MintTo(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = revoke::Revoke::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::Revoke(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            set_authority::SetAuthority::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::SetAuthority(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            sync_native::SyncNative::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::SyncNative(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            thaw_account::ThawAccount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::ThawAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            transfer_checked::TransferChecked::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::TransferChecked(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            transfer::Transfer::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: TokenProgramInstruction::Transfer(decoded_instruction),
            });
        }

        None
    }
}
