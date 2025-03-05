use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use crate::PROGRAM_ID;

use super::FluxbeamDecoder;
pub mod swap_v1;

pub enum FluxbeamAccount {
    SwapV1(swap_v1::SwapV1),
}

impl<'a> AccountDecoder<'a> for FluxbeamDecoder {
    type AccountType = FluxbeamAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) = swap_v1::SwapV1::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: FluxbeamAccount::SwapV1(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use solana_sdk::pubkey;

    use crate::types::{CurveType, Fees, SwapCurve};

    use super::*;

    #[test]
    fn test_decode_swap_account() {
        // Arrange
        let expected_swap_account = swap_v1::SwapV1 {
            _padding: 0,
            is_initialized: true,
            bump_seed: 254,
            token_program_id: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            token_a: pubkey!("jM5cFHP9iPj9en1fJFJLfRpLt68Y81UdWfXHv9an3HK"),
            token_b: pubkey!("8a4WD4hbfuPyiistrVU8qcpwMcJmf3RBuw1s1tvVYJ1Q"),
            pool_mint: pubkey!("7XeJQykinTiK1EveXb9y4zodFtdtu1YwkygBmWbz1pC3"),
            token_a_mint: pubkey!("So11111111111111111111111111111111111111112"),
            token_b_mint: pubkey!("3YkBR2w1ttpWKzdP5XQtzXqsGFS9i1mGg9pDrqn4e9j6"),
            pool_fee_account: pubkey!("396TeW1MeyQvFGgxjaxJxRFkuiir4Ye4imuxVDcqfE88"),
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
        };

        // Act
        let decoder = FluxbeamDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/swap_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            FluxbeamAccount::SwapV1(swap_account) => {
                assert_eq!(
                    expected_swap_account.is_initialized,
                    swap_account.is_initialized
                );
                assert_eq!(expected_swap_account.bump_seed, swap_account.bump_seed);
                assert_eq!(
                    expected_swap_account.token_program_id,
                    swap_account.token_program_id
                );
                assert_eq!(expected_swap_account.token_a, swap_account.token_a);
                assert_eq!(expected_swap_account.token_b, swap_account.token_b);
                assert_eq!(expected_swap_account.pool_mint, swap_account.pool_mint);
                assert_eq!(
                    expected_swap_account.token_a_mint,
                    swap_account.token_a_mint
                );
                assert_eq!(
                    expected_swap_account.token_b_mint,
                    swap_account.token_b_mint
                );
                assert_eq!(
                    expected_swap_account.pool_fee_account,
                    swap_account.pool_fee_account
                );
                assert_eq!(expected_swap_account.fees, swap_account.fees);
                assert_eq!(expected_swap_account.swap_curve, swap_account.swap_curve);
            }
            _ => panic!("Expected SwapV1"),
        }
    }
}
