use crate::PROGRAM_ID;

use super::FluxbeamDecoder;
pub mod deposit_all_token_types;
pub mod deposit_single_token_type_exact_amount_in;
pub mod initialize;
pub mod swap;
pub mod withdraw_all_token_types;
pub mod withdraw_single_token_type_exact_amount_out;

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
pub enum FluxbeamInstruction {
    Initialize(initialize::Initialize),
    Swap(swap::Swap),
    DepositAllTokenTypes(deposit_all_token_types::DepositAllTokenTypes),
    WithdrawAllTokenTypes(withdraw_all_token_types::WithdrawAllTokenTypes),
    DepositSingleTokenTypeExactAmountIn(
        deposit_single_token_type_exact_amount_in::DepositSingleTokenTypeExactAmountIn,
    ),
    WithdrawSingleTokenTypeExactAmountOut(
        withdraw_single_token_type_exact_amount_out::WithdrawSingleTokenTypeExactAmountOut,
    ),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for FluxbeamDecoder {
    type InstructionType = FluxbeamInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            FluxbeamInstruction::Initialize => initialize::Initialize,
            FluxbeamInstruction::Swap => swap::Swap,
            FluxbeamInstruction::DepositAllTokenTypes => deposit_all_token_types::DepositAllTokenTypes,
            FluxbeamInstruction::WithdrawAllTokenTypes => withdraw_all_token_types::WithdrawAllTokenTypes,
            FluxbeamInstruction::DepositSingleTokenTypeExactAmountIn => deposit_single_token_type_exact_amount_in::DepositSingleTokenTypeExactAmountIn,
            FluxbeamInstruction::WithdrawSingleTokenTypeExactAmountOut => withdraw_single_token_type_exact_amount_out::WithdrawSingleTokenTypeExactAmountOut,
        )
    }
}

#[cfg(test)]
mod tests {
    use carbon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
    use solana_sdk::{instruction::AccountMeta, pubkey};

    use crate::types::{CurveType, Fees, SwapCurve};

    use super::*;

    #[test]
    fn test_decode_initialize() {
        // Arrange
        let expected_ix = FluxbeamInstruction::Initialize(initialize::Initialize {
            fees: Fees {
                trade_fee_numerator: 2,
                trade_fee_denominator: 1000,
                owner_trade_fee_numerator: 90,
                owner_trade_fee_denominator: 100,
                owner_withdraw_fee_numerator: 98,
                owner_withdraw_fee_denominator: 100,
                host_fee_numerator: 0,
                host_fee_denominator: 10000,
            },
            swap_curve: SwapCurve {
                curve_type: CurveType::ConstantProduct,
                calculator: [0u8; 32],
            },
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("6bJUX2XqmGp6nZGrnEoZh3mAt8M73G1AZbgUhT4hooAC"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("2CQ6cW8RzowMEcdEiRRgEWzaYjpLWaHv1WoVyWfF8nsY"),
                false,
            ),
            AccountMeta::new(
                pubkey!("jM5cFHP9iPj9en1fJFJLfRpLt68Y81UdWfXHv9an3HK"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8a4WD4hbfuPyiistrVU8qcpwMcJmf3RBuw1s1tvVYJ1Q"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7XeJQykinTiK1EveXb9y4zodFtdtu1YwkygBmWbz1pC3"),
                true,
            ),
            AccountMeta::new(
                pubkey!("396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88"),
                false,
            ),
            AccountMeta::new(
                pubkey!("396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
        ];
        let expected_arranged_accounts = initialize::InitializeInstructionAccounts {
            swap: pubkey!("6bJUX2XqmGp6nZGrnEoZh3mAt8M73G1AZbgUhT4hooAC"),
            authority: pubkey!("2CQ6cW8RzowMEcdEiRRgEWzaYjpLWaHv1WoVyWfF8nsY"),
            token_a: pubkey!("jM5cFHP9iPj9en1fJFJLfRpLt68Y81UdWfXHv9an3HK"),
            token_b: pubkey!("8a4WD4hbfuPyiistrVU8qcpwMcJmf3RBuw1s1tvVYJ1Q"),
            pool: pubkey!("7XeJQykinTiK1EveXb9y4zodFtdtu1YwkygBmWbz1pC3"),
            fee: pubkey!("396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88"),
            destination: pubkey!("396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88"),
            token_program: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
        };

        // Act
        let decoder = FluxbeamDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/initialize_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            initialize::Initialize::arrange_accounts(&instruction.accounts)
                .expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }
}
