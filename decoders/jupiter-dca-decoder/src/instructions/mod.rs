use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;

use super::DcaDecoder;
pub mod close_dca;
pub mod closed;
pub mod collected_fee;
pub mod deposit;
pub mod end_and_close;
pub mod filled;
pub mod fulfill_dlmm_fill;
pub mod fulfill_flash_fill;
pub mod initiate_dlmm_fill;
pub mod initiate_flash_fill;
pub mod open_dca;
pub mod open_dca_v2;
pub mod opened;
pub mod transfer;
pub mod withdraw;
pub mod withdraw_fees;

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
pub enum DcaInstruction {
    OpenDca(open_dca::OpenDca),
    OpenDcaV2(open_dca_v2::OpenDcaV2),
    CloseDca(close_dca::CloseDca),
    Withdraw(withdraw::Withdraw),
    Deposit(deposit::Deposit),
    WithdrawFees(withdraw_fees::WithdrawFees),
    InitiateFlashFill(initiate_flash_fill::InitiateFlashFill),
    FulfillFlashFill(fulfill_flash_fill::FulfillFlashFill),
    InitiateDlmmFill(initiate_dlmm_fill::InitiateDlmmFill),
    FulfillDlmmFill(fulfill_dlmm_fill::FulfillDlmmFill),
    Transfer(transfer::Transfer),
    EndAndClose(end_and_close::EndAndClose),
    CollectedFee(collected_fee::CollectedFee),
    Filled(filled::Filled),
    Opened(opened::Opened),
    Closed(closed::Closed),
}

impl InstructionDecoder for DcaDecoder {
    type InstructionType = DcaInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) =
            open_dca::OpenDca::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::OpenDca(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            open_dca_v2::OpenDcaV2::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::OpenDcaV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            close_dca::CloseDca::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::CloseDca(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            withdraw::Withdraw::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::Withdraw(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            deposit::Deposit::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::Deposit(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            withdraw_fees::WithdrawFees::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::WithdrawFees(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initiate_flash_fill::InitiateFlashFill::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::InitiateFlashFill(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            fulfill_flash_fill::FulfillFlashFill::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::FulfillFlashFill(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            initiate_dlmm_fill::InitiateDlmmFill::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::InitiateDlmmFill(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            fulfill_dlmm_fill::FulfillDlmmFill::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::FulfillDlmmFill(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            transfer::Transfer::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::Transfer(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            end_and_close::EndAndClose::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::EndAndClose(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            collected_fee::CollectedFee::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::CollectedFee(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = filled::Filled::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::Filled(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = opened::Opened::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::Opened(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = closed::Closed::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::Closed(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            withdraw::Withdraw::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::Withdraw(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            deposit::Deposit::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: DcaInstruction::Deposit(decoded_instruction),
            });
        }

        None
    }
}
