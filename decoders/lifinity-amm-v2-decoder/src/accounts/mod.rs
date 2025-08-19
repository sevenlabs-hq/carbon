use {
    super::LifinityAmmV2Decoder,
    crate::PROGRAM_ID,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod amm;

pub enum LifinityAmmV2Account {
    Amm(amm::Amm),
}

impl AccountDecoder<'_> for LifinityAmmV2Decoder {
    type AccountType = LifinityAmmV2Account;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) = amm::Amm::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: LifinityAmmV2Account::Amm(decoded_account),
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

    use super::{super::types::*, *};

    #[test]
    fn test_decode_amm_account() {
        let expected_amm_account = amm::Amm {
            initializer_key: solana_pubkey::Pubkey::from_str_const(
                "11111111111111111111111111111111",
            ),
            initializer_deposit_token_account: solana_pubkey::Pubkey::from_str_const(
                "11111111111111111111111111111111",
            ),
            initializer_receive_token_account: solana_pubkey::Pubkey::from_str_const(
                "11111111111111111111111111111111",
            ),
            initializer_amount: 0,
            taker_amount: 0,
            is_initialized: true,
            bump_seed: 255,
            freeze_trade: 1,
            freeze_deposit: 1,
            freeze_withdraw: 0,
            base_decimals: 11,
            token_program_id: solana_pubkey::Pubkey::from_str_const(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            ),
            token_a_account: solana_pubkey::Pubkey::from_str_const(
                "9xNYopD8DTGm2M489XavdnMshBYA81RqJeEPoTUB6sNL",
            ),
            token_b_account: solana_pubkey::Pubkey::from_str_const(
                "HzFLZptVPTGd3bj1fNQXeUbpUxVm6t8f9i1BsuLo41zj",
            ),
            pool_mint: solana_pubkey::Pubkey::from_str_const(
                "FGPYaW5HRuMhw9y68UHSecA6mLdc3JbLfdvq11DZePRH",
            ),
            token_a_mint: solana_pubkey::Pubkey::from_str_const(
                "7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr",
            ),
            token_b_mint: solana_pubkey::Pubkey::from_str_const(
                "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
            ),
            fee_account: solana_pubkey::Pubkey::from_str_const(
                "GbbjF9fnx6dRjqcbEbkuRPRd41r3CgSue8av4sad79rX",
            ),
            oracle_main_account: solana_pubkey::Pubkey::from_str_const(
                "5A2LoA3rHP51WDfGVKbrmrDkeTLVBTiV4uvjJBDKHK2X",
            ),
            oracle_sub_account: solana_pubkey::Pubkey::from_str_const(
                "5A2LoA3rHP51WDfGVKbrmrDkeTLVBTiV4uvjJBDKHK2X",
            ),
            oracle_pc_account: solana_pubkey::Pubkey::from_str_const(
                "FVAouryrBKKtgSf4jBqU5UwuwwrT6TUniSwQC7TJXDTK",
            ),
            fees: AmmFees {
                trade_fee_numerator: 0,
                trade_fee_denominator: 1_000_000,
                owner_trade_fee_numerator: 200,
                owner_trade_fee_denominator: 1_000_000,
                owner_withdraw_fee_numerator: 0,
                owner_withdraw_fee_denominator: 0,
                host_fee_numerator: 0,
                host_fee_denominator: 0,
            },
            curve: AmmCurve {
                curve_type: 0,
                curve_parameters: 0,
            },
            config: AmmConfig {
                last_price: 177_071_193,
                last_balanced_price: 1_638_750_694,
                config_denominator: 1_000_000,
                volume_x: 45_609_928_585_195_809,
                volume_y: 61_722_464_173_238,
                volume_x_in_y: 61_761_324_387_894,
                deposit_cap: 1_000_000_000_000,
                regression_target: 16_000_000_000_000,
                oracle_type: 0,
                oracle_status: 0,
                oracle_main_slot_limit: 180,
                oracle_sub_confidence_limit: 100,
                oracle_sub_slot_limit: 25,
                oracle_pc_confidence_limit: 100,
                oracle_pc_slot_limit: 10_000,
                std_spread: 150_000_000_000,
                std_spread_buffer: 10_300,
                spread_coefficient: 12_000_000,
                price_buffer_coin: 0,
                price_buffer_pc: 0,
                rebalance_ratio: 0,
                fee_trade: 0,
                fee_platform: 9_134_557_312_188,
                oracle_main_slot_buffer: 0,
                config_temp4: 0,
                config_temp5: 0,
                config_temp6: 0,
                config_temp7: 0,
                config_temp8: 0,
            },
            amm_p_temp1: solana_pubkey::Pubkey::from_str_const("11111111111111111111111111111111"),
            amm_p_temp2: solana_pubkey::Pubkey::from_str_const("11111111111111111111111111111111"),
            amm_p_temp3: solana_pubkey::Pubkey::from_str_const("11111111111111111111111111111111"),
            amm_p_temp4: solana_pubkey::Pubkey::from_str_const("11111111111111111111111111111111"),
            amm_p_temp5: solana_pubkey::Pubkey::from_str_const("11111111111111111111111111111111"),
        };

        let decoder = LifinityAmmV2Decoder;
        let account = carbon_test_utils::read_account("tests/fixtures/amm_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        match decoded_account.data {
            LifinityAmmV2Account::Amm(amm_account) => {
                assert_eq!(
                    expected_amm_account.initializer_key,
                    amm_account.initializer_key
                );
                assert_eq!(
                    expected_amm_account.initializer_deposit_token_account,
                    amm_account.initializer_deposit_token_account
                );
                assert_eq!(
                    expected_amm_account.initializer_receive_token_account,
                    amm_account.initializer_receive_token_account
                );
                assert_eq!(
                    expected_amm_account.initializer_amount,
                    amm_account.initializer_amount
                );
                assert_eq!(expected_amm_account.taker_amount, amm_account.taker_amount);
                assert_eq!(
                    expected_amm_account.is_initialized,
                    amm_account.is_initialized
                );
                assert_eq!(expected_amm_account.bump_seed, amm_account.bump_seed);
                assert_eq!(expected_amm_account.freeze_trade, amm_account.freeze_trade);
                assert_eq!(
                    expected_amm_account.freeze_deposit,
                    amm_account.freeze_deposit
                );
                assert_eq!(
                    expected_amm_account.freeze_withdraw,
                    amm_account.freeze_withdraw
                );
                assert_eq!(
                    expected_amm_account.base_decimals,
                    amm_account.base_decimals
                );
                assert_eq!(
                    expected_amm_account.token_program_id,
                    amm_account.token_program_id
                );
                assert_eq!(
                    expected_amm_account.token_a_account,
                    amm_account.token_a_account
                );
                assert_eq!(
                    expected_amm_account.token_b_account,
                    amm_account.token_b_account
                );
                assert_eq!(expected_amm_account.pool_mint, amm_account.pool_mint);
                assert_eq!(expected_amm_account.token_a_mint, amm_account.token_a_mint);
                assert_eq!(expected_amm_account.token_b_mint, amm_account.token_b_mint);
                assert_eq!(expected_amm_account.fee_account, amm_account.fee_account);
                assert_eq!(
                    expected_amm_account.oracle_main_account,
                    amm_account.oracle_main_account
                );
                assert_eq!(
                    expected_amm_account.oracle_sub_account,
                    amm_account.oracle_sub_account
                );
                assert_eq!(
                    expected_amm_account.oracle_pc_account,
                    amm_account.oracle_pc_account
                );

                assert_eq!(expected_amm_account.fees, amm_account.fees);
                assert_eq!(expected_amm_account.curve, amm_account.curve);
                assert_eq!(expected_amm_account.config, amm_account.config);
                assert_eq!(expected_amm_account.amm_p_temp1, amm_account.amm_p_temp1);
                assert_eq!(expected_amm_account.amm_p_temp2, amm_account.amm_p_temp2);
                assert_eq!(expected_amm_account.amm_p_temp3, amm_account.amm_p_temp3);
                assert_eq!(expected_amm_account.amm_p_temp4, amm_account.amm_p_temp4);
                assert_eq!(expected_amm_account.amm_p_temp5, amm_account.amm_p_temp5);
            }
        }
    }
}
