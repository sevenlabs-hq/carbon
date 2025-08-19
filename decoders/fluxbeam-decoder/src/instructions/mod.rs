use {super::FluxbeamDecoder, crate::PROGRAM_ID};
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

impl carbon_core::instruction::InstructionDecoder<'_> for FluxbeamDecoder {
    type InstructionType = FluxbeamInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
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
    use {
        super::*,
        crate::types::{CurveType, Fees, SwapCurve},
        carbon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder},
        solana_instruction::AccountMeta,
    };

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
                solana_pubkey::Pubkey::from_str_const(
                    "6bJUX2XqmGp6nZGrnEoZh3mAt8M73G1AZbgUhT4hooAC",
                ),
                true,
            ),
            AccountMeta::new_readonly(
                solana_pubkey::Pubkey::from_str_const(
                    "2CQ6cW8RzowMEcdEiRRgEWzaYjpLWaHv1WoVyWfF8nsY",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "jM5cFHP9iPj9en1fJFJLfRpLt68Y81UdWfXHv9an3HK",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "8a4WD4hbfuPyiistrVU8qcpwMcJmf3RBuw1s1tvVYJ1Q",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "7XeJQykinTiK1EveXb9y4zodFtdtu1YwkygBmWbz1pC3",
                ),
                true,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88",
                ),
                false,
            ),
            AccountMeta::new_readonly(
                solana_pubkey::Pubkey::from_str_const(
                    "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
                ),
                false,
            ),
        ];
        let expected_arranged_accounts = initialize::InitializeInstructionAccounts {
            swap: solana_pubkey::Pubkey::from_str_const(
                "6bJUX2XqmGp6nZGrnEoZh3mAt8M73G1AZbgUhT4hooAC",
            ),
            authority: solana_pubkey::Pubkey::from_str_const(
                "2CQ6cW8RzowMEcdEiRRgEWzaYjpLWaHv1WoVyWfF8nsY",
            ),
            token_a: solana_pubkey::Pubkey::from_str_const(
                "jM5cFHP9iPj9en1fJFJLfRpLt68Y81UdWfXHv9an3HK",
            ),
            token_b: solana_pubkey::Pubkey::from_str_const(
                "8a4WD4hbfuPyiistrVU8qcpwMcJmf3RBuw1s1tvVYJ1Q",
            ),
            pool: solana_pubkey::Pubkey::from_str_const(
                "7XeJQykinTiK1EveXb9y4zodFtdtu1YwkygBmWbz1pC3",
            ),
            fee: solana_pubkey::Pubkey::from_str_const(
                "396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88",
            ),
            destination: solana_pubkey::Pubkey::from_str_const(
                "396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88",
            ),
            token_program: solana_pubkey::Pubkey::from_str_const(
                "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
            ),
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
                solana_pubkey::Pubkey::from_str_const(
                    "6bJUX2XqmGp6nZGrnEoZh3mAt8M73G1AZbgUhT4hooAC",
                ),
                false,
            ),
            AccountMeta::new_readonly(
                solana_pubkey::Pubkey::from_str_const(
                    "2CQ6cW8RzowMEcdEiRRgEWzaYjpLWaHv1WoVyWfF8nsY",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "AB1daTZcHAySAexN1SpacinrwRixNP7nLd31TVnNXMLx",
                ),
                true,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "DX1mX7WN7jQJXzaiiQR6W1G69xHg3kXjDtrEyYXxgZAm",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "jM5cFHP9iPj9en1fJFJLfRpLt68Y81UdWfXHv9an3HK",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "8a4WD4hbfuPyiistrVU8qcpwMcJmf3RBuw1s1tvVYJ1Q",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "Ew1Aj2Mm82KCN9dtMNnhVXZDjUfiiu18CNj9Qx6Vystk",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "7XeJQykinTiK1EveXb9y4zodFtdtu1YwkygBmWbz1pC3",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88",
                ),
                false,
            ),
            AccountMeta::new_readonly(
                solana_pubkey::Pubkey::from_str_const(
                    "So11111111111111111111111111111111111111112",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "3YkBR2w1ttpWKzdP5XQtzXqsGFS9i1mGg9pDrqn4e9j6",
                ),
                false,
            ),
            AccountMeta::new_readonly(
                solana_pubkey::Pubkey::from_str_const(
                    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                ),
                false,
            ),
            AccountMeta::new_readonly(
                solana_pubkey::Pubkey::from_str_const(
                    "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
                ),
                false,
            ),
            AccountMeta::new_readonly(
                solana_pubkey::Pubkey::from_str_const(
                    "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88",
                ),
                false,
            ),
        ];
        let expected_arranged_accounts = swap::SwapInstructionAccounts {
            swap: solana_pubkey::Pubkey::from_str_const(
                "6bJUX2XqmGp6nZGrnEoZh3mAt8M73G1AZbgUhT4hooAC",
            ),
            authority: solana_pubkey::Pubkey::from_str_const(
                "2CQ6cW8RzowMEcdEiRRgEWzaYjpLWaHv1WoVyWfF8nsY",
            ),
            user_transfer_authority: solana_pubkey::Pubkey::from_str_const(
                "AB1daTZcHAySAexN1SpacinrwRixNP7nLd31TVnNXMLx",
            ),
            source: solana_pubkey::Pubkey::from_str_const(
                "DX1mX7WN7jQJXzaiiQR6W1G69xHg3kXjDtrEyYXxgZAm",
            ),
            swap_source: solana_pubkey::Pubkey::from_str_const(
                "jM5cFHP9iPj9en1fJFJLfRpLt68Y81UdWfXHv9an3HK",
            ),
            swap_destination: solana_pubkey::Pubkey::from_str_const(
                "8a4WD4hbfuPyiistrVU8qcpwMcJmf3RBuw1s1tvVYJ1Q",
            ),
            destination: solana_pubkey::Pubkey::from_str_const(
                "Ew1Aj2Mm82KCN9dtMNnhVXZDjUfiiu18CNj9Qx6Vystk",
            ),
            pool_mint: solana_pubkey::Pubkey::from_str_const(
                "7XeJQykinTiK1EveXb9y4zodFtdtu1YwkygBmWbz1pC3",
            ),
            pool_fee: solana_pubkey::Pubkey::from_str_const(
                "396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88",
            ),
            source_mint: solana_pubkey::Pubkey::from_str_const(
                "So11111111111111111111111111111111111111112",
            ),
            destination_mint: solana_pubkey::Pubkey::from_str_const(
                "3YkBR2w1ttpWKzdP5XQtzXqsGFS9i1mGg9pDrqn4e9j6",
            ),
            source_token_program: solana_pubkey::Pubkey::from_str_const(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            ),
            destination_token_program: solana_pubkey::Pubkey::from_str_const(
                "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
            ),
            pool_token_program: solana_pubkey::Pubkey::from_str_const(
                "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
            ),
            swap_program: solana_pubkey::Pubkey::from_str_const(
                "396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88",
            ),
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
                solana_pubkey::Pubkey::from_str_const(
                    "6bJUX2XqmGp6nZGrnEoZh3mAt8M73G1AZbgUhT4hooAC",
                ),
                false,
            ),
            AccountMeta::new_readonly(
                solana_pubkey::Pubkey::from_str_const(
                    "2CQ6cW8RzowMEcdEiRRgEWzaYjpLWaHv1WoVyWfF8nsY",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "3oYDvLGyqdTsZ5mU8gKPxjjkKsZiE6y9qUE2Ed4qR6iG",
                ),
                true,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "7XeJQykinTiK1EveXb9y4zodFtdtu1YwkygBmWbz1pC3",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "jM5cFHP9iPj9en1fJFJLfRpLt68Y81UdWfXHv9an3HK",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "8a4WD4hbfuPyiistrVU8qcpwMcJmf3RBuw1s1tvVYJ1Q",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "D86hML8ecnD7kPLpjKDPzRCToPKyJ6xfQUoJ39wLTxYZ",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "7xUofx3SxTGYgsVbdnYd6qMuxzaBGbHFoiAfD2q5d1aA",
                ),
                false,
            ),
            AccountMeta::new(
                solana_pubkey::Pubkey::from_str_const(
                    "396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88",
                ),
                false,
            ),
            AccountMeta::new_readonly(
                solana_pubkey::Pubkey::from_str_const(
                    "So11111111111111111111111111111111111111112",
                ),
                false,
            ),
            AccountMeta::new_readonly(
                solana_pubkey::Pubkey::from_str_const(
                    "3YkBR2w1ttpWKzdP5XQtzXqsGFS9i1mGg9pDrqn4e9j6",
                ),
                false,
            ),
            AccountMeta::new_readonly(
                solana_pubkey::Pubkey::from_str_const(
                    "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
                ),
                false,
            ),
            AccountMeta::new_readonly(
                solana_pubkey::Pubkey::from_str_const(
                    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                ),
                false,
            ),
            AccountMeta::new_readonly(
                solana_pubkey::Pubkey::from_str_const(
                    "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
                ),
                false,
            ),
        ];
        let expected_arranged_accounts =
            withdraw_all_token_types::WithdrawAllTokenTypesInstructionAccounts {
                swap: solana_pubkey::Pubkey::from_str_const(
                    "6bJUX2XqmGp6nZGrnEoZh3mAt8M73G1AZbgUhT4hooAC",
                ),
                authority: solana_pubkey::Pubkey::from_str_const(
                    "2CQ6cW8RzowMEcdEiRRgEWzaYjpLWaHv1WoVyWfF8nsY",
                ),
                user_transfer_authority: solana_pubkey::Pubkey::from_str_const(
                    "3oYDvLGyqdTsZ5mU8gKPxjjkKsZiE6y9qUE2Ed4qR6iG",
                ),
                pool_mint: solana_pubkey::Pubkey::from_str_const(
                    "7XeJQykinTiK1EveXb9y4zodFtdtu1YwkygBmWbz1pC3",
                ),
                source: solana_pubkey::Pubkey::from_str_const(
                    "396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88",
                ),
                swap_token_a: solana_pubkey::Pubkey::from_str_const(
                    "jM5cFHP9iPj9en1fJFJLfRpLt68Y81UdWfXHv9an3HK",
                ),
                swap_token_b: solana_pubkey::Pubkey::from_str_const(
                    "8a4WD4hbfuPyiistrVU8qcpwMcJmf3RBuw1s1tvVYJ1Q",
                ),
                destination_token_a: solana_pubkey::Pubkey::from_str_const(
                    "D86hML8ecnD7kPLpjKDPzRCToPKyJ6xfQUoJ39wLTxYZ",
                ),
                destination_token_b: solana_pubkey::Pubkey::from_str_const(
                    "7xUofx3SxTGYgsVbdnYd6qMuxzaBGbHFoiAfD2q5d1aA",
                ),
                fee_account: solana_pubkey::Pubkey::from_str_const(
                    "396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88",
                ),
                token_a_mint: solana_pubkey::Pubkey::from_str_const(
                    "So11111111111111111111111111111111111111112",
                ),
                token_b_mint: solana_pubkey::Pubkey::from_str_const(
                    "3YkBR2w1ttpWKzdP5XQtzXqsGFS9i1mGg9pDrqn4e9j6",
                ),
                pool_token_program: solana_pubkey::Pubkey::from_str_const(
                    "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
                ),
                token_a_program: solana_pubkey::Pubkey::from_str_const(
                    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                ),
                token_b_program: solana_pubkey::Pubkey::from_str_const(
                    "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
                ),
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
