



use super::SolayerRestakingProgramDecoder;
pub mod initialize;
pub mod restake;
pub mod unrestake;
pub mod batch_thaw_lst_accounts;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum SolayerRestakingProgramInstruction {
    Initialize(initialize::Initialize),
    Restake(restake::Restake),
    Unrestake(unrestake::Unrestake),
    BatchThawLstAccounts(batch_thaw_lst_accounts::BatchThawLstAccounts),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for SolayerRestakingProgramDecoder {
    type InstructionType = SolayerRestakingProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            SolayerRestakingProgramInstruction::Initialize => initialize::Initialize,
            SolayerRestakingProgramInstruction::Restake => restake::Restake,
            SolayerRestakingProgramInstruction::Unrestake => unrestake::Unrestake,
            SolayerRestakingProgramInstruction::BatchThawLstAccounts => batch_thaw_lst_accounts::BatchThawLstAccounts,
        )
    }
}