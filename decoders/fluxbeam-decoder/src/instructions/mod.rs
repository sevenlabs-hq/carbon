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

    #[test]
    fn test_decode_swap() {
        // Arrange
        let expected_ix = FluxbeamInstruction::Swap(swap::Swap {
            amount_in: 800000000,
            minimum_amount_out: 0,
        });
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("6bJUX2XqmGp6nZGrnEoZh3mAt8M73G1AZbgUhT4hooAC"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("2CQ6cW8RzowMEcdEiRRgEWzaYjpLWaHv1WoVyWfF8nsY"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AB1daTZcHAySAexN1SpacinrwRixNP7nLd31TVnNXMLx"),
                true,
            ),
            AccountMeta::new(
                pubkey!("DX1mX7WN7jQJXzaiiQR6W1G69xHg3kXjDtrEyYXxgZAm"),
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
                pubkey!("Ew1Aj2Mm82KCN9dtMNnhVXZDjUfiiu18CNj9Qx6Vystk"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7XeJQykinTiK1EveXb9y4zodFtdtu1YwkygBmWbz1pC3"),
                false,
            ),
            AccountMeta::new(
                pubkey!("396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3YkBR2w1ttpWKzdP5XQtzXqsGFS9i1mGg9pDrqn4e9j6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new(
                pubkey!("396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88"),
                false,
            ),
        ];
        let expected_arranged_accounts = swap::SwapInstructionAccounts {
            swap: pubkey!("6bJUX2XqmGp6nZGrnEoZh3mAt8M73G1AZbgUhT4hooAC"),
            authority: pubkey!("2CQ6cW8RzowMEcdEiRRgEWzaYjpLWaHv1WoVyWfF8nsY"),
            user_transfer_authority: pubkey!("AB1daTZcHAySAexN1SpacinrwRixNP7nLd31TVnNXMLx"),
            source: pubkey!("DX1mX7WN7jQJXzaiiQR6W1G69xHg3kXjDtrEyYXxgZAm"),
            swap_source: pubkey!("jM5cFHP9iPj9en1fJFJLfRpLt68Y81UdWfXHv9an3HK"),
            swap_destination: pubkey!("8a4WD4hbfuPyiistrVU8qcpwMcJmf3RBuw1s1tvVYJ1Q"),
            destination: pubkey!("Ew1Aj2Mm82KCN9dtMNnhVXZDjUfiiu18CNj9Qx6Vystk"),
            pool_mint: pubkey!("7XeJQykinTiK1EveXb9y4zodFtdtu1YwkygBmWbz1pC3"),
            pool_fee: pubkey!("396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88"),
            source_mint: pubkey!("So11111111111111111111111111111111111111112"),
            destination_mint: pubkey!("3YkBR2w1ttpWKzdP5XQtzXqsGFS9i1mGg9pDrqn4e9j6"),
            source_token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            destination_token_program: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            pool_token_program: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            swap_program: pubkey!("396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88"),
        };

        // Act
        let decoder = FluxbeamDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/swap_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            swap::Swap::arrange_accounts(&instruction.accounts).expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_withdraw_all_token_types() {
        // Arrange
        let expected_ix = FluxbeamInstruction::WithdrawAllTokenTypes(
            withdraw_all_token_types::WithdrawAllTokenTypes {
                pool_token_amount: 1149106633,
                minimum_token_a_amount: 0,
                minimum_token_b_amount: 0,
            },
        );
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("6bJUX2XqmGp6nZGrnEoZh3mAt8M73G1AZbgUhT4hooAC"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("2CQ6cW8RzowMEcdEiRRgEWzaYjpLWaHv1WoVyWfF8nsY"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3oYDvLGyqdTsZ5mU8gKPxjjkKsZiE6y9qUE2Ed4qR6iG"),
                true,
            ),
            AccountMeta::new(
                pubkey!("7XeJQykinTiK1EveXb9y4zodFtdtu1YwkygBmWbz1pC3"),
                false,
            ),
            AccountMeta::new(
                pubkey!("396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88"),
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
                pubkey!("D86hML8ecnD7kPLpjKDPzRCToPKyJ6xfQUoJ39wLTxYZ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7xUofx3SxTGYgsVbdnYd6qMuxzaBGbHFoiAfD2q5d1aA"),
                false,
            ),
            AccountMeta::new(
                pubkey!("396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("3YkBR2w1ttpWKzdP5XQtzXqsGFS9i1mGg9pDrqn4e9j6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
        ];
        let expected_arranged_accounts =
            withdraw_all_token_types::WithdrawAllTokenTypesInstructionAccounts {
                swap: pubkey!("6bJUX2XqmGp6nZGrnEoZh3mAt8M73G1AZbgUhT4hooAC"),
                authority: pubkey!("2CQ6cW8RzowMEcdEiRRgEWzaYjpLWaHv1WoVyWfF8nsY"),
                user_transfer_authority: pubkey!("3oYDvLGyqdTsZ5mU8gKPxjjkKsZiE6y9qUE2Ed4qR6iG"),
                pool_mint: pubkey!("7XeJQykinTiK1EveXb9y4zodFtdtu1YwkygBmWbz1pC3"),
                source: pubkey!("396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88"),
                swap_token_a: pubkey!("jM5cFHP9iPj9en1fJFJLfRpLt68Y81UdWfXHv9an3HK"),
                swap_token_b: pubkey!("8a4WD4hbfuPyiistrVU8qcpwMcJmf3RBuw1s1tvVYJ1Q"),
                destination_token_a: pubkey!("D86hML8ecnD7kPLpjKDPzRCToPKyJ6xfQUoJ39wLTxYZ"),
                destination_token_b: pubkey!("7xUofx3SxTGYgsVbdnYd6qMuxzaBGbHFoiAfD2q5d1aA"),
                fee_account: pubkey!("396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88"),
                token_a_mint: pubkey!("So11111111111111111111111111111111111111112"),
                token_b_mint: pubkey!("3YkBR2w1ttpWKzdP5XQtzXqsGFS9i1mGg9pDrqn4e9j6"),
                pool_token_program: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                token_a_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                token_b_program: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            };

        // Act
        let decoder = FluxbeamDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/withdraw_all_token_types_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            withdraw_all_token_types::WithdrawAllTokenTypes::arrange_accounts(
                &instruction.accounts,
            )
            .expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }
}
