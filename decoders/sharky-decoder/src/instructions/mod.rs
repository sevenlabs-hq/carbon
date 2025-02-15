use super::SharkyDecoder;
pub mod close_nft_list;
pub mod close_order_book;
pub mod create_nft_list;
pub mod create_order_book;
pub mod create_program_version;
pub mod extend_loan_v3;
pub mod extend_loan_v3_compressed;
pub mod foreclose_loan_v3;
pub mod foreclose_loan_v3_compressed;
pub mod offer_loan;
pub mod repay_loan_v3;
pub mod repay_loan_v3_compressed;
pub mod rescind_loan;
pub mod take_loan_v3;
pub mod take_loan_v3_compressed;
pub mod update_nft_list;
pub mod update_order_book;
pub mod update_program_version;

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
pub enum SharkyInstruction {
    CreateOrderBook(create_order_book::CreateOrderBook),
    UpdateOrderBook(update_order_book::UpdateOrderBook),
    CloseOrderBook(close_order_book::CloseOrderBook),
    OfferLoan(offer_loan::OfferLoan),
    RescindLoan(rescind_loan::RescindLoan),
    TakeLoanV3(take_loan_v3::TakeLoanV3),
    TakeLoanV3Compressed(take_loan_v3_compressed::TakeLoanV3Compressed),
    ForecloseLoanV3(foreclose_loan_v3::ForecloseLoanV3),
    ForecloseLoanV3Compressed(foreclose_loan_v3_compressed::ForecloseLoanV3Compressed),
    RepayLoanV3Compressed(repay_loan_v3_compressed::RepayLoanV3Compressed),
    RepayLoanV3(repay_loan_v3::RepayLoanV3),
    ExtendLoanV3(extend_loan_v3::ExtendLoanV3),
    ExtendLoanV3Compressed(extend_loan_v3_compressed::ExtendLoanV3Compressed),
    CreateNftList(create_nft_list::CreateNftList),
    UpdateNftList(update_nft_list::UpdateNftList),
    CloseNftList(close_nft_list::CloseNftList),
    CreateProgramVersion(create_program_version::CreateProgramVersion),
    UpdateProgramVersion(update_program_version::UpdateProgramVersion),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for SharkyDecoder {
    type InstructionType = SharkyInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            SharkyInstruction::CreateOrderBook => create_order_book::CreateOrderBook,
            SharkyInstruction::UpdateOrderBook => update_order_book::UpdateOrderBook,
            SharkyInstruction::CloseOrderBook => close_order_book::CloseOrderBook,
            SharkyInstruction::OfferLoan => offer_loan::OfferLoan,
            SharkyInstruction::RescindLoan => rescind_loan::RescindLoan,
            SharkyInstruction::TakeLoanV3 => take_loan_v3::TakeLoanV3,
            SharkyInstruction::TakeLoanV3Compressed => take_loan_v3_compressed::TakeLoanV3Compressed,
            SharkyInstruction::ForecloseLoanV3 => foreclose_loan_v3::ForecloseLoanV3,
            SharkyInstruction::ForecloseLoanV3Compressed => foreclose_loan_v3_compressed::ForecloseLoanV3Compressed,
            SharkyInstruction::RepayLoanV3Compressed => repay_loan_v3_compressed::RepayLoanV3Compressed,
            SharkyInstruction::RepayLoanV3 => repay_loan_v3::RepayLoanV3,
            SharkyInstruction::ExtendLoanV3 => extend_loan_v3::ExtendLoanV3,
            SharkyInstruction::ExtendLoanV3Compressed => extend_loan_v3_compressed::ExtendLoanV3Compressed,
            SharkyInstruction::CreateNftList => create_nft_list::CreateNftList,
            SharkyInstruction::UpdateNftList => update_nft_list::UpdateNftList,
            SharkyInstruction::CloseNftList => close_nft_list::CloseNftList,
            SharkyInstruction::CreateProgramVersion => create_program_version::CreateProgramVersion,
            SharkyInstruction::UpdateProgramVersion => update_program_version::UpdateProgramVersion,
        )
    }
}
