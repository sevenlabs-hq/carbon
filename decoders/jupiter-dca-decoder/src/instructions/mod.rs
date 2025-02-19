use super::JupiterDcaDecoder;
pub mod close_dca;
pub mod closed_event;
pub mod collected_fee_event;
pub mod deposit;
pub mod deposit_event;
pub mod end_and_close;
pub mod filled_event;
pub mod fulfill_dlmm_fill;
pub mod fulfill_flash_fill;
pub mod initiate_dlmm_fill;
pub mod initiate_flash_fill;
pub mod open_dca;
pub mod open_dca_v2;
pub mod opened_event;
pub mod transfer;
pub mod withdraw;
pub mod withdraw_event;
pub mod withdraw_fees;

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
pub enum JupiterDcaInstruction {
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
    CollectedFeeEvent(collected_fee_event::CollectedFeeEvent),
    FilledEvent(filled_event::FilledEvent),
    OpenedEvent(opened_event::OpenedEvent),
    ClosedEvent(closed_event::ClosedEvent),
    WithdrawEvent(withdraw_event::WithdrawEvent),
    DepositEvent(deposit_event::DepositEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for JupiterDcaDecoder {
    type InstructionType = JupiterDcaInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            JupiterDcaInstruction::OpenDca => open_dca::OpenDca,
            JupiterDcaInstruction::OpenDcaV2 => open_dca_v2::OpenDcaV2,
            JupiterDcaInstruction::CloseDca => close_dca::CloseDca,
            JupiterDcaInstruction::Withdraw => withdraw::Withdraw,
            JupiterDcaInstruction::Deposit => deposit::Deposit,
            JupiterDcaInstruction::WithdrawFees => withdraw_fees::WithdrawFees,
            JupiterDcaInstruction::InitiateFlashFill => initiate_flash_fill::InitiateFlashFill,
            JupiterDcaInstruction::FulfillFlashFill => fulfill_flash_fill::FulfillFlashFill,
            JupiterDcaInstruction::InitiateDlmmFill => initiate_dlmm_fill::InitiateDlmmFill,
            JupiterDcaInstruction::FulfillDlmmFill => fulfill_dlmm_fill::FulfillDlmmFill,
            JupiterDcaInstruction::Transfer => transfer::Transfer,
            JupiterDcaInstruction::EndAndClose => end_and_close::EndAndClose,
            JupiterDcaInstruction::CollectedFeeEvent => collected_fee_event::CollectedFeeEvent,
            JupiterDcaInstruction::FilledEvent => filled_event::FilledEvent,
            JupiterDcaInstruction::OpenedEvent => opened_event::OpenedEvent,
            JupiterDcaInstruction::ClosedEvent => closed_event::ClosedEvent,
            JupiterDcaInstruction::WithdrawEvent => withdraw_event::WithdrawEvent,
            JupiterDcaInstruction::DepositEvent => deposit_event::DepositEvent,
        )
    }
}
