use {
    super::OrcaWhirlpoolDecoder,
    crate::PROGRAM_ID,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod fee_tier;
pub mod position;
pub mod position_bundle;
pub mod tick_array;
pub mod token_badge;
pub mod whirlpool;
pub mod whirlpools_config;
pub mod whirlpools_config_extension;

pub enum OrcaWhirlpoolAccount {
    WhirlpoolsConfigExtension(whirlpools_config_extension::WhirlpoolsConfigExtension),
    WhirlpoolsConfig(whirlpools_config::WhirlpoolsConfig),
    FeeTier(fee_tier::FeeTier),
    PositionBundle(position_bundle::PositionBundle),
    Position(position::Position),
    TickArray(Box<tick_array::TickArray>),
    TokenBadge(token_badge::TokenBadge),
    Whirlpool(Box<whirlpool::Whirlpool>),
}

impl AccountDecoder<'_> for OrcaWhirlpoolDecoder {
    type AccountType = OrcaWhirlpoolAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            whirlpools_config_extension::WhirlpoolsConfigExtension::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OrcaWhirlpoolAccount::WhirlpoolsConfigExtension(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            whirlpools_config::WhirlpoolsConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OrcaWhirlpoolAccount::WhirlpoolsConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = fee_tier::FeeTier::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OrcaWhirlpoolAccount::FeeTier(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            position_bundle::PositionBundle::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OrcaWhirlpoolAccount::PositionBundle(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = position::Position::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OrcaWhirlpoolAccount::Position(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = tick_array::TickArray::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OrcaWhirlpoolAccount::TickArray(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = token_badge::TokenBadge::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OrcaWhirlpoolAccount::TokenBadge(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = whirlpool::Whirlpool::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OrcaWhirlpoolAccount::Whirlpool(Box::new(decoded_account)),
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
    use {
        super::*,
        crate::types::{PositionRewardInfo, Tick, WhirlpoolRewardInfo},
        solana_pubkey::pubkey,
    };

    #[test]
    fn test_decode_whirlpools_config_extension_account() {
        // Arrange
        let expected_account = whirlpools_config_extension::WhirlpoolsConfigExtension {
            whirlpools_config: pubkey!("ET8gW61Ru2Fgi4Nyvm4uQHeah7v8QMhnXnrgXxN2Y2s9"),
            config_extension_authority: pubkey!("6Pg1AsSovedLHBgtAzm4i3A7AoZDTo28qkV9U46Zj8iA"),
            token_badge_authority: pubkey!("6Pg1AsSovedLHBgtAzm4i3A7AoZDTo28qkV9U46Zj8iA"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let account = carbon_test_utils::read_account(
            "tests/fixtures/whirlpools_config_extension_account.json",
        )
        .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            OrcaWhirlpoolAccount::WhirlpoolsConfigExtension(account) => {
                assert_eq!(
                    expected_account.config_extension_authority,
                    account.config_extension_authority
                );
                assert_eq!(
                    expected_account.token_badge_authority,
                    account.token_badge_authority
                );
                assert_eq!(
                    expected_account.whirlpools_config,
                    account.whirlpools_config
                );
            }
            _ => panic!("Expected WhirlpoolsConfigExtension"),
        }
    }

    #[test]
    fn test_decode_whirlpools_config_account() {
        // Arrange
        let expected_account = whirlpools_config::WhirlpoolsConfig {
            fee_authority: pubkey!("6Pg1AsSovedLHBgtAzm4i3A7AoZDTo28qkV9U46Zj8iA"),
            collect_protocol_fees_authority: pubkey!(
                "6Pg1AsSovedLHBgtAzm4i3A7AoZDTo28qkV9U46Zj8iA"
            ),
            reward_emissions_super_authority: pubkey!(
                "6Pg1AsSovedLHBgtAzm4i3A7AoZDTo28qkV9U46Zj8iA"
            ),
            default_protocol_fee_rate: 300,
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let account =
            carbon_test_utils::read_account("tests/fixtures/whirlpools_config_account.json")
                .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            OrcaWhirlpoolAccount::WhirlpoolsConfig(account) => {
                assert_eq!(
                    expected_account.collect_protocol_fees_authority,
                    account.collect_protocol_fees_authority
                );
                assert_eq!(
                    expected_account.default_protocol_fee_rate,
                    account.default_protocol_fee_rate
                );
                assert_eq!(expected_account.fee_authority, account.fee_authority);
                assert_eq!(
                    expected_account.reward_emissions_super_authority,
                    account.reward_emissions_super_authority
                );
            }
            _ => panic!("Expected WhirlpoolsConfig"),
        }
    }

    #[test]
    fn test_decode_fee_tier_account() {
        // Arrange
        let expected_account = fee_tier::FeeTier {
            whirlpools_config: pubkey!("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ"),
            tick_spacing: 32896,
            default_fee_rate: 10000,
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/fee_tier_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            OrcaWhirlpoolAccount::FeeTier(account) => {
                assert_eq!(expected_account.default_fee_rate, account.default_fee_rate);
                assert_eq!(expected_account.tick_spacing, account.tick_spacing);
                assert_eq!(
                    expected_account.whirlpools_config,
                    account.whirlpools_config
                );
            }
            _ => panic!("Expected FeeTier"),
        }
    }

    #[test]
    fn test_decode_position_bundle_account() {
        // Arrange
        let expected_account = position_bundle::PositionBundle {
            position_bundle_mint: pubkey!("EMY7ftt18nnjQSLd8v9QvJ91hZbeRFQRjXbkb5pEJWpF"),
            position_bitmap: [0u8; 32],
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let account =
            carbon_test_utils::read_account("tests/fixtures/position_bundle_account.json")
                .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            OrcaWhirlpoolAccount::PositionBundle(account) => {
                assert_eq!(expected_account.position_bitmap, account.position_bitmap);
                assert_eq!(
                    expected_account.position_bundle_mint,
                    account.position_bundle_mint
                );
            }
            _ => panic!("Expected PositionBundle"),
        }
    }

    #[test]
    fn test_decode_position_account() {
        // Arrange
        let expected_account = position::Position {
            whirlpool: pubkey!("C9U2Ksk6KKWvLEeo5yUQ7Xu46X7NzeBJtd9PBfuXaUSM"),
            position_mint: pubkey!("4t8CquCQLazH6AZHtCdqfNowTvGztBRFdQ5ewimu8bpB"),
            liquidity: 27713585964,
            tick_lower_index: -21264,
            tick_upper_index: -19424,
            fee_growth_checkpoint_a: 340282366920938463461883143742666180270,
            fee_owed_a: 0,
            fee_growth_checkpoint_b: 340282366920938463463072690694709896731,
            fee_owed_b: 0,
            reward_infos: [
                PositionRewardInfo {
                    growth_inside_checkpoint: 0,
                    amount_owed: 0,
                },
                PositionRewardInfo {
                    growth_inside_checkpoint: 0,
                    amount_owed: 0,
                },
                PositionRewardInfo {
                    growth_inside_checkpoint: 0,
                    amount_owed: 0,
                },
            ],
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/position_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            OrcaWhirlpoolAccount::Position(account) => {
                assert_eq!(
                    expected_account.fee_growth_checkpoint_a,
                    account.fee_growth_checkpoint_a
                );
                assert_eq!(
                    expected_account.fee_growth_checkpoint_b,
                    account.fee_growth_checkpoint_b
                );
                assert_eq!(expected_account.fee_owed_a, account.fee_owed_a);
                assert_eq!(expected_account.fee_owed_b, account.fee_owed_b);
                assert_eq!(expected_account.liquidity, account.liquidity);
                assert_eq!(expected_account.position_mint, account.position_mint);
                assert_eq!(expected_account.reward_infos, account.reward_infos);
                assert_eq!(expected_account.tick_lower_index, account.tick_lower_index);
                assert_eq!(expected_account.tick_upper_index, account.tick_upper_index);
                assert_eq!(expected_account.whirlpool, account.whirlpool);
            }
            _ => panic!("Expected Position"),
        }
    }

    #[test]
    fn test_decode_tick_array_account() {
        // Arrange
        let expected_account = tick_array::TickArray {
            start_tick_index: -2894848,
            ticks: {
                let mut ticks = [Tick {
                    initialized: false,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside_a: 0,
                    fee_growth_outside_b: 0,
                    reward_growths_outside: [0, 0, 0],
                }; 88];

                ticks[75] = Tick {
                    initialized: true,
                    liquidity_net: 141421356,
                    liquidity_gross: 141421356,
                    fee_growth_outside_a: 0,
                    fee_growth_outside_b: 0,
                    reward_growths_outside: [0, 0, 0],
                };

                ticks
            },
            whirlpool: pubkey!("CGGNcohZdLdeDBdhmQRGmUH1Viv1p4d1ds2aPLoiVWaR"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/tick_array_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            OrcaWhirlpoolAccount::TickArray(account) => {
                assert_eq!(expected_account.start_tick_index, account.start_tick_index);
                assert_eq!(expected_account.whirlpool, account.whirlpool);
                assert_eq!(expected_account.ticks, account.ticks);
            }
            _ => panic!("Expected TickArray"),
        }
    }

    #[test]
    fn test_decode_token_badge_account() {
        // Arrange
        let expected_account = token_badge::TokenBadge {
            whirlpools_config: pubkey!("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ"),
            token_mint: pubkey!("mzeroXDoBpRVhnEXBra27qzAMdxgpWVY3DzQW7xMVJp"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/token_badge_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            OrcaWhirlpoolAccount::TokenBadge(account) => {
                assert_eq!(expected_account.token_mint, account.token_mint);
                assert_eq!(
                    expected_account.whirlpools_config,
                    account.whirlpools_config
                );
            }
            _ => panic!("Expected TokenBadge"),
        }
    }

    #[test]
    fn test_decode_whirlpool_account() {
        // Arrange
        let expected_account = whirlpool::Whirlpool {
            whirlpools_config: pubkey!("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ"),
            whirlpool_bump: [246],
            tick_spacing: 32896,
            tick_spacing_seed: [128, 128],
            fee_rate: 10000,
            protocol_fee_rate: 1300,
            liquidity: 141421356,
            sqrt_price: 1304381782533278221200000,
            tick_current_index: 223338,
            protocol_fee_owed_a: 0,
            protocol_fee_owed_b: 0,
            token_mint_a: pubkey!("So11111111111111111111111111111111111111112"),
            token_vault_a: pubkey!("6cS7s6TZuQofs1TPYSpZ51qGPQKnrxk6xWhW131MysXh"),
            fee_growth_global_a: 0,
            token_mint_b: pubkey!("DzkKGBw4njGApBJ4Bd2csCpm4rsdgEmkgivR7XZvENPv"),
            token_vault_b: pubkey!("25tSGzR5NcRpuoWKzWFTWfMaB1ijiaHP6HkqjxZNH72m"),
            fee_growth_global_b: 0,
            reward_last_updated_timestamp: 1745354322,
            reward_infos: [
                WhirlpoolRewardInfo {
                    mint: pubkey!("11111111111111111111111111111111"),
                    vault: pubkey!("11111111111111111111111111111111"),
                    authority: pubkey!("DjDsi34mSB66p2nhBL6YvhbcLtZbkGfNybFeLDjJqxJW"),
                    emissions_per_second_x64: 0,
                    growth_global_x64: 0,
                },
                WhirlpoolRewardInfo {
                    mint: pubkey!("11111111111111111111111111111111"),
                    vault: pubkey!("11111111111111111111111111111111"),
                    authority: pubkey!("DjDsi34mSB66p2nhBL6YvhbcLtZbkGfNybFeLDjJqxJW"),
                    emissions_per_second_x64: 0,
                    growth_global_x64: 0,
                },
                WhirlpoolRewardInfo {
                    mint: pubkey!("11111111111111111111111111111111"),
                    vault: pubkey!("11111111111111111111111111111111"),
                    authority: pubkey!("DjDsi34mSB66p2nhBL6YvhbcLtZbkGfNybFeLDjJqxJW"),
                    emissions_per_second_x64: 0,
                    growth_global_x64: 0,
                },
            ],
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/whirlpool_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            OrcaWhirlpoolAccount::Whirlpool(account) => {
                assert_eq!(
                    expected_account.fee_growth_global_a,
                    account.fee_growth_global_a
                );
                assert_eq!(
                    expected_account.fee_growth_global_b,
                    account.fee_growth_global_b
                );
                assert_eq!(expected_account.fee_rate, account.fee_rate);
                assert_eq!(expected_account.liquidity, account.liquidity);
                assert_eq!(
                    expected_account.protocol_fee_owed_a,
                    account.protocol_fee_owed_a
                );
                assert_eq!(
                    expected_account.protocol_fee_owed_b,
                    account.protocol_fee_owed_b
                );
                assert_eq!(
                    expected_account.protocol_fee_rate,
                    account.protocol_fee_rate
                );
                assert_eq!(expected_account.reward_infos, account.reward_infos);
                assert_eq!(
                    expected_account.reward_last_updated_timestamp,
                    account.reward_last_updated_timestamp
                );
                assert_eq!(expected_account.sqrt_price, account.sqrt_price);
                assert_eq!(
                    expected_account.tick_current_index,
                    account.tick_current_index
                );
                assert_eq!(expected_account.tick_spacing, account.tick_spacing);
                assert_eq!(
                    expected_account.tick_spacing_seed,
                    account.tick_spacing_seed
                );
                assert_eq!(expected_account.token_mint_a, account.token_mint_a);
                assert_eq!(expected_account.token_mint_b, account.token_mint_b);
                assert_eq!(expected_account.token_vault_a, account.token_vault_a);
                assert_eq!(expected_account.token_vault_b, account.token_vault_b);
                assert_eq!(expected_account.whirlpool_bump, account.whirlpool_bump);
                assert_eq!(
                    expected_account.whirlpools_config,
                    account.whirlpools_config
                );
            }
            _ => panic!("Expected Whirlpool"),
        }
    }
}
