use carbon_core::{
    deserialize::CarbonDeserialize,
    instruction::{DecodedInstruction, InstructionDecoder},
};

use crate::SystemProgramDecoder;

pub mod advance_nonce_account;
pub mod allocate;
pub mod allocate_with_seed;
pub mod assign;
pub mod assign_with_seed;
pub mod authorize_nonce_account;
pub mod create_account;
pub mod create_account_with_seed;
pub mod initialize_nonce_account;
pub mod transfer;
pub mod transfer_with_seed;
pub mod upgrade_nonce_account;
pub mod withdraw_nonce_account;

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
pub enum SystemProgramInstruction {
    CreateAccount(create_account::CreateAccount),
    Assign(assign::Assign),
    Transfer(transfer::Transfer),
    CreateAccountWithSeed(create_account_with_seed::CreateAccountWithSeed),
    AdvanceNonceAccount(advance_nonce_account::AdvanceNonceAccount),
    WithdrawNonceAccount(withdraw_nonce_account::WithdrawNonceAccount),
    InitializeNonceAccount(initialize_nonce_account::InitializeNonceAccount),
    AuthorizeNonceAccount(authorize_nonce_account::AuthorizeNonceAccount),
    Allocate(allocate::Allocate),
    AllocateWithSeed(allocate_with_seed::AllocateWithSeed),
    AssignWithSeed(assign_with_seed::AssignWithSeed),
    TransferWithSeed(transfer_with_seed::TransferWithSeed),
    UpgradeNonceAccount(upgrade_nonce_account::UpgradeNonceAccount),
}

impl InstructionDecoder for SystemProgramDecoder {
    type InstructionType = SystemProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) =
            create_account::CreateAccount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: SystemProgramInstruction::CreateAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = assign::Assign::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: SystemProgramInstruction::Assign(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            transfer::Transfer::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: SystemProgramInstruction::Transfer(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            create_account_with_seed::CreateAccountWithSeed::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: SystemProgramInstruction::CreateAccountWithSeed(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            advance_nonce_account::AdvanceNonceAccount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: SystemProgramInstruction::AdvanceNonceAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            withdraw_nonce_account::WithdrawNonceAccount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: SystemProgramInstruction::WithdrawNonceAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initialize_nonce_account::InitializeNonceAccount::deserialize(
                instruction.data.as_slice(),
            )
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: SystemProgramInstruction::InitializeNonceAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            authorize_nonce_account::AuthorizeNonceAccount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: SystemProgramInstruction::AuthorizeNonceAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            allocate::Allocate::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: SystemProgramInstruction::Allocate(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            allocate_with_seed::AllocateWithSeed::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: SystemProgramInstruction::AllocateWithSeed(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            assign_with_seed::AssignWithSeed::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: SystemProgramInstruction::AssignWithSeed(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            transfer_with_seed::TransferWithSeed::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: SystemProgramInstruction::TransferWithSeed(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            upgrade_nonce_account::UpgradeNonceAccount::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: SystemProgramInstruction::UpgradeNonceAccount(decoded_instruction),
            });
        }

        None
    }
}
