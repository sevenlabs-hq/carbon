use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum SignatureVerificationError {
    InvalidEd25519InstructionProgramId,
    InvalidEd25519InstructionDataLength,
    InvalidSignatureIndex,
    InvalidSignatureOffset,
    InvalidPublicKeyOffset,
    InvalidMessageOffset,
    InvalidMessageDataSize,
    InvalidInstructionIndex,
    MessageOffsetOverflow,
    InvalidMessageHex,
    InvalidMessageData,
    LoadInstructionAtFailed,
}
