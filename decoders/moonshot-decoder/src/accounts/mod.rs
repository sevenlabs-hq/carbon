use {
    super::MoonshotDecoder,
    crate::PROGRAM_ID,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod config_account;
pub mod curve_account;

pub enum MoonshotAccount {
    ConfigAccount(config_account::ConfigAccount),
    CurveAccount(curve_account::CurveAccount),
}

impl AccountDecoder<'_> for MoonshotDecoder {
    type AccountType = MoonshotAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            config_account::ConfigAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MoonshotAccount::ConfigAccount(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            curve_account::CurveAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MoonshotAccount::CurveAccount(decoded_account),
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

    use super::*;

    #[test]
    fn test_decode_config_account() {
        // Arrange
        let expected_cfg_account = config_account::ConfigAccount {
            migration_authority: pubkey!("CGsqR7CTqTwbmAUTPnfg9Bj9GLJgkrUD9rhjh3vHEYvh"),
            backend_authority: pubkey!("Cb8Fnhp95f9dLxB3sYkNCbN3Mjxuc3v2uQZ7uVeqvNGB"),
            config_authority: pubkey!("AnHuyURVXM9nzWYGJZCxBFT5MJGr9fGjTg2kKFZBHgUk"),
            helio_fee: pubkey!("5K5RtTWzzLp4P8Npi84ocf7F1vBsAu29N1irG4iiUnzt"),
            dex_fee: pubkey!("3udvfL24waJcLhskRAsStNMoNUvtyXdxrWQz4hgi953N"),
            fee_bps: 100,
            dex_fee_share: 60,
            migration_fee: 2,
            marketcap_threshold: 345000000000,
            marketcap_currency: crate::types::Currency::Sol,
            min_supported_decimal_places: 6,
            max_supported_decimal_places: 9,
            min_supported_token_supply: 10000000,
            max_supported_token_supply: 1000000000,
            bump: 251,
            coef_b: 25,
            ..config_account::ConfigAccount::default()
        };

        // Act
        let decoder = MoonshotDecoder;
        let account =
            carbon_test_utils::read_account("../../tests/fixtures/moonshot/config_account.json")
                .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            MoonshotAccount::ConfigAccount(cfg_account) => {
                assert_eq!(
                    expected_cfg_account.migration_authority,
                    cfg_account.migration_authority
                );
                assert_eq!(
                    expected_cfg_account.backend_authority,
                    cfg_account.backend_authority
                );
                assert_eq!(
                    expected_cfg_account.config_authority,
                    cfg_account.config_authority
                );
                assert_eq!(expected_cfg_account.helio_fee, cfg_account.helio_fee);
                assert_eq!(expected_cfg_account.dex_fee, cfg_account.dex_fee);
                assert_eq!(expected_cfg_account.fee_bps, cfg_account.fee_bps);
                assert_eq!(
                    expected_cfg_account.dex_fee_share,
                    cfg_account.dex_fee_share
                );
                assert_eq!(
                    expected_cfg_account.migration_fee,
                    cfg_account.migration_fee
                );
                assert_eq!(
                    expected_cfg_account.marketcap_threshold,
                    cfg_account.marketcap_threshold
                );
                assert_eq!(
                    expected_cfg_account.marketcap_currency,
                    cfg_account.marketcap_currency
                );
                assert_eq!(
                    expected_cfg_account.min_supported_decimal_places,
                    cfg_account.min_supported_decimal_places
                );
                assert_eq!(
                    expected_cfg_account.max_supported_decimal_places,
                    cfg_account.max_supported_decimal_places
                );
                assert_eq!(
                    expected_cfg_account.min_supported_token_supply,
                    cfg_account.min_supported_token_supply
                );
                assert_eq!(
                    expected_cfg_account.max_supported_token_supply,
                    cfg_account.max_supported_token_supply
                );
                assert_eq!(expected_cfg_account.bump, cfg_account.bump);
                assert_eq!(expected_cfg_account.coef_b, cfg_account.coef_b);
            }
            _ => panic!("Expected ConfigAccount"),
        }
    }

    #[test]
    fn test_decode_curve_account() {
        // Arrange
        let expected_curve_account = curve_account::CurveAccount {
            total_supply: 1000000000000000000,
            curve_amount: 979053197346106125,
            mint: pubkey!("3cBFsM1wosTJi9yun6kcHhYHyJcut1MNQY28zjC4moon"),
            decimals: 9,
            collateral_currency: crate::types::Currency::Sol,
            curve_type: crate::types::CurveType::ConstantProductV1,
            marketcap_threshold: 345000000000,
            marketcap_currency: crate::types::Currency::Sol,
            migration_fee: 2,
            coef_b: 25,
            bump: 255,
            migration_target: crate::types::MigrationTarget::Raydium,
            ..curve_account::CurveAccount::default()
        };

        // Act
        let decoder = MoonshotDecoder;
        let account =
            carbon_test_utils::read_account("../../tests/fixtures/moonshot/curve_account.json")
                .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            MoonshotAccount::CurveAccount(curve_account) => {
                assert_eq!(expected_curve_account, curve_account);
            }
            _ => panic!("Expected CurveAccount"),
        }
    }
}
