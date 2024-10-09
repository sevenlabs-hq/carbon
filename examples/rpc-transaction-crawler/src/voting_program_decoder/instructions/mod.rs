use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;

use super::VotingProgramDecoder;
pub mod cast_vote;
pub mod cast_vote_event;
pub mod create_vote;
pub mod create_vote_event;

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
pub enum VotingProgramInstruction {
    CastVote(cast_vote::CastVote),
    CreateVote(create_vote::CreateVote),
    CastVoteEvent(cast_vote_event::CastVoteEvent),
    CreateVoteEvent(create_vote_event::CreateVoteEvent),
}

impl InstructionDecoder for VotingProgramDecoder {
    type InstructionType = VotingProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) =
            cast_vote::CastVote::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: VotingProgramInstruction::CastVote(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            create_vote::CreateVote::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: VotingProgramInstruction::CreateVote(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            cast_vote_event::CastVoteEvent::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: VotingProgramInstruction::CastVoteEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) =
            create_vote_event::CreateVoteEvent::deserialize(instruction.data.as_slice())
        {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                accounts: instruction.accounts,
                data: VotingProgramInstruction::CreateVoteEvent(decoded_instruction),
            });
        }

        None
    }
}
