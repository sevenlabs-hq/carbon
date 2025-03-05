use crate::PROGRAM_ID;

use super::LifinityAmmV2Decoder;
pub mod deposit_all_token_types;
pub mod swap;
pub mod withdraw_all_token_types;

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
pub enum LifinityAmmV2Instruction {
    Swap(swap::Swap),
    DepositAllTokenTypes(deposit_all_token_types::DepositAllTokenTypes),
    WithdrawAllTokenTypes(withdraw_all_token_types::WithdrawAllTokenTypes),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for LifinityAmmV2Decoder {
    type InstructionType = LifinityAmmV2Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            LifinityAmmV2Instruction::Swap => swap::Swap,
            LifinityAmmV2Instruction::DepositAllTokenTypes => deposit_all_token_types::DepositAllTokenTypes,
            LifinityAmmV2Instruction::WithdrawAllTokenTypes => withdraw_all_token_types::WithdrawAllTokenTypes,
        )
    }
}

#[cfg(test)]
mod tests {
    use carbon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
    use solana_sdk::{instruction::AccountMeta, pubkey};

    use super::*;

    #[test]
    fn test_decode_swap() {
        let expected_ix = LifinityAmmV2Instruction::Swap(swap::Swap {
            amount_in: 99140,
            minimum_amount_out: 0,
        });

        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("Hc3DCcYyN7m2Wasf4Zhe8FMQ9qf9PiBiMJ9vqwSqB42h"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EiEAydLqSKFqRPpuwYoVxEJ6h9UZh9tsTaHgs4f8b8Z5"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("CapuXNQoDviLvU1PxFiizLgPNQCxrsag1uMeyk6zLVps"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GQuvMWcBF1M2wgh2sbxkonq7FtBc6UNurtHjREMRAL1x"),
                false,
            ),
            AccountMeta::new(
                pubkey!("91bUbswo6Di8235jAPwim1At4cPZLbG2pkpneyqKg4NQ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("D8F3PPxSuykAgyPPKwQdXDGGoRnUXzxowaheVJw5ATDC"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GUicRosQyLJCYG8hjYcbiGKAVAmT1puQTVmJjFxJmdMK"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2e6NAJy1qaKMq8PaswP2uzimMDvbr71Tbw38G6q9SNZ2"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2EVZT2cFMvbqE9nSVidYVkrSouKfudcKG6R8AKiXoSY9"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("EPBJUVCmzvwkGPGcEuwKmXomfGt78Aozy6pj44x9xxDB"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("EPBJUVCmzvwkGPGcEuwKmXomfGt78Aozy6pj44x9xxDB"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("3ZDBff7jeQaksmGvmkRix36rU159EBDjYiPThvV8QVZM"),
                false,
            ),
        ];
        let expected_arranged_accounts = swap::SwapInstructionAccounts {
            authority: pubkey!("Hc3DCcYyN7m2Wasf4Zhe8FMQ9qf9PiBiMJ9vqwSqB42h"),
            amm: pubkey!("EiEAydLqSKFqRPpuwYoVxEJ6h9UZh9tsTaHgs4f8b8Z5"),
            user_transfer_authority: pubkey!("CapuXNQoDviLvU1PxFiizLgPNQCxrsag1uMeyk6zLVps"),
            source_info: pubkey!("GQuvMWcBF1M2wgh2sbxkonq7FtBc6UNurtHjREMRAL1x"),
            destination_info: pubkey!("91bUbswo6Di8235jAPwim1At4cPZLbG2pkpneyqKg4NQ"),
            swap_source: pubkey!("D8F3PPxSuykAgyPPKwQdXDGGoRnUXzxowaheVJw5ATDC"),
            swap_destination: pubkey!("GUicRosQyLJCYG8hjYcbiGKAVAmT1puQTVmJjFxJmdMK"),
            pool_mint: pubkey!("2e6NAJy1qaKMq8PaswP2uzimMDvbr71Tbw38G6q9SNZ2"),
            fee_account: pubkey!("2EVZT2cFMvbqE9nSVidYVkrSouKfudcKG6R8AKiXoSY9"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            oracle_main_account: pubkey!("EPBJUVCmzvwkGPGcEuwKmXomfGt78Aozy6pj44x9xxDB"),
            oracle_sub_account: pubkey!("EPBJUVCmzvwkGPGcEuwKmXomfGt78Aozy6pj44x9xxDB"),
            oracle_pc_account: pubkey!("3ZDBff7jeQaksmGvmkRix36rU159EBDjYiPThvV8QVZM"),
        };

        let decoder = LifinityAmmV2Decoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/swap_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            swap::Swap::arrange_accounts(&instruction.accounts).expect("aranage accounts");

        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_deposit_all_token_types() {
        let expected_ix = LifinityAmmV2Instruction::DepositAllTokenTypes(
            deposit_all_token_types::DepositAllTokenTypes {
                pool_token_amount: 2457402,
                maximum_token_a_amount: 1000000,
                maximum_token_b_amount: 64575,
            },
        );

        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("4ruXyJT6rgHERQeVmMCc5pJDC5wgLujuMjTQgY562sdh"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("2p4AA6xU2gxVvCVMVMAwpmyF49zBfp9MD4Ef8JeGVeMY"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CbYf9QNrkVgNRCMTDiVdvzMqSzXh8AAgnrKAoTfEACdh"),
                true,
            ),
            AccountMeta::new(
                pubkey!("oVzCGsiinVTxPNmWFzi4QvGJ1iyhzUHj148Jc7SPuyj"),
                false,
            ),
            AccountMeta::new(
                pubkey!("74NpLshFLkuVyFKq2L3yBU2V8mcExSGy5E8tjCpMfVvU"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7GawBqVSriYXQYCTr5XygeRNTeHamRWeHmVFiuf6wLfK"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7oWW46LfAzrzWaZxV8CURBKpWwR8cc3ZpaXMFi6CH8sC"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Dt1EBunetoSDPSEj2EB5m5bc93bt75ZB8Yy9vmaJ2ng"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3hmRWv4Qik4rEzut8kqjxChN3n7Ms1MLKFKLqcoWkbGg"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
        ];

        let expected_arranged_accounts =
            deposit_all_token_types::DepositAllTokenTypesInstructionAccounts {
                amm: pubkey!("4ruXyJT6rgHERQeVmMCc5pJDC5wgLujuMjTQgY562sdh"),
                authority: pubkey!("2p4AA6xU2gxVvCVMVMAwpmyF49zBfp9MD4Ef8JeGVeMY"),
                user_transfer_authority_info: pubkey!(
                    "CbYf9QNrkVgNRCMTDiVdvzMqSzXh8AAgnrKAoTfEACdh"
                ),
                source_a_info: pubkey!("oVzCGsiinVTxPNmWFzi4QvGJ1iyhzUHj148Jc7SPuyj"),
                source_b_info: pubkey!("74NpLshFLkuVyFKq2L3yBU2V8mcExSGy5E8tjCpMfVvU"),
                token_a: pubkey!("7GawBqVSriYXQYCTr5XygeRNTeHamRWeHmVFiuf6wLfK"),
                token_b: pubkey!("7oWW46LfAzrzWaZxV8CURBKpWwR8cc3ZpaXMFi6CH8sC"),
                pool_mint: pubkey!("Dt1EBunetoSDPSEj2EB5m5bc93bt75ZB8Yy9vmaJ2ng"),
                destination: pubkey!("3hmRWv4Qik4rEzut8kqjxChN3n7Ms1MLKFKLqcoWkbGg"),
                token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            };

        let decoder = LifinityAmmV2Decoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/deposit_all_token_types_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            deposit_all_token_types::DepositAllTokenTypes::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_withdraw_all_token_types() {
        let expected_ix = LifinityAmmV2Instruction::WithdrawAllTokenTypes(
            withdraw_all_token_types::WithdrawAllTokenTypes {
                pool_token_amount: 2000655039581,
                minimum_token_a_amount: 1076172871830,
                minimum_token_b_amount: 752578501683,
            },
        );

        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("DrRd8gYMJu9XGxLhwTCPdHNLXCKHsxJtMpbn62YqmwQe"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("7GmDCbu7bYiWJvFaNUyPNiM8PjvvBcmyBcZY1qSsAGi2"),
                false,
            ),
            AccountMeta::new(
                pubkey!("71hhezkHQ2dhmPySsHVCCkLggfWzPFEBdfEjbn4NCXMG"),
                true,
            ),
            AccountMeta::new(
                pubkey!("A9x5SwN9vbg3YDxZySDWaHuVoQQ8YSJDrmjFZ3yMksqW"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EVGW4q1iFjDmtxtHr3NoPi5iVKAxwEjohsusMrinDxr6"),
                false,
            ),
            AccountMeta::new(
                pubkey!("53EkU98Vbv2TQPwGG6t2asCynzFjCX5AnvaabbXafaed"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FGYgFJSxZTGzaLwzUL9YZqK2yUZ8seofCwGq8BPEw4o8"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Gf5ucgWGfJ8NLvVaKJVen9CYsPFRK4eAsZWT8zDjARkA"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8vPGw7d6a4rmejVg2uXMs8kvEnVy7WgN1oNk9mRpdfGP"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
        ];

        let expected_arranged_accounts =
            withdraw_all_token_types::WithdrawAllTokenTypesInstructionAccounts {
                amm: pubkey!("DrRd8gYMJu9XGxLhwTCPdHNLXCKHsxJtMpbn62YqmwQe"),
                authority: pubkey!("7GmDCbu7bYiWJvFaNUyPNiM8PjvvBcmyBcZY1qSsAGi2"),
                user_transfer_authority_info: pubkey!(
                    "71hhezkHQ2dhmPySsHVCCkLggfWzPFEBdfEjbn4NCXMG"
                ),
                source_info: pubkey!("A9x5SwN9vbg3YDxZySDWaHuVoQQ8YSJDrmjFZ3yMksqW"),
                token_a: pubkey!("EVGW4q1iFjDmtxtHr3NoPi5iVKAxwEjohsusMrinDxr6"),
                token_b: pubkey!("53EkU98Vbv2TQPwGG6t2asCynzFjCX5AnvaabbXafaed"),
                pool_mint: pubkey!("FGYgFJSxZTGzaLwzUL9YZqK2yUZ8seofCwGq8BPEw4o8"),
                dest_token_a_info: pubkey!("Gf5ucgWGfJ8NLvVaKJVen9CYsPFRK4eAsZWT8zDjARkA"),
                dest_token_b_info: pubkey!("8vPGw7d6a4rmejVg2uXMs8kvEnVy7WgN1oNk9mRpdfGP"),
                token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            };

        let decoder = LifinityAmmV2Decoder;
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
            .expect("arrange accounts");

        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }
}
