use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;
use carbon_macros::try_decode_instructions;

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

impl<'a> InstructionDecoder<'a> for VotingProgramDecoder {
    type InstructionType = VotingProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        try_decode_instructions!(instruction,
            VotingProgramInstruction::CastVote => cast_vote::CastVote,
            VotingProgramInstruction::CreateVote => create_vote::CreateVote,
            VotingProgramInstruction::CastVoteEvent => cast_vote_event::CastVoteEvent,
            VotingProgramInstruction::CreateVoteEvent => create_vote_event::CreateVoteEvent,
        )
    }
}
