use {
    super::RaydiumClmmDecoder,
    crate::PROGRAM_ID,
    alloc::boxed::Box,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod amm_config;
pub mod observation_state;
pub mod operation_state;
pub mod personal_position_state;
pub mod pool_state;
pub mod protocol_position_state;
pub mod tick_array_bitmap_extension;
pub mod tick_array_state;

#[derive(Debug)]
pub enum RaydiumClmmAccount {
    AmmConfig(amm_config::AmmConfig),
    OperationState(Box<operation_state::OperationState>),
    ObservationState(Box<observation_state::ObservationState>),
    PersonalPositionState(personal_position_state::PersonalPositionState),
    PoolState(Box<pool_state::PoolState>),
    ProtocolPositionState(protocol_position_state::ProtocolPositionState),
    TickArrayState(Box<tick_array_state::TickArrayState>),
    TickArrayBitmapExtension(Box<tick_array_bitmap_extension::TickArrayBitmapExtension>),
}

impl AccountDecoder<'_> for RaydiumClmmDecoder {
    type AccountType = RaydiumClmmAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) = amm_config::AmmConfig::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumClmmAccount::AmmConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            operation_state::OperationState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumClmmAccount::OperationState(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            observation_state::ObservationState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumClmmAccount::ObservationState(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            personal_position_state::PersonalPositionState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumClmmAccount::PersonalPositionState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = pool_state::PoolState::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumClmmAccount::PoolState(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            protocol_position_state::ProtocolPositionState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumClmmAccount::ProtocolPositionState(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            tick_array_state::TickArrayState::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumClmmAccount::TickArrayState(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            tick_array_bitmap_extension::TickArrayBitmapExtension::deserialize(
                account.data.as_slice(),
            )
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: RaydiumClmmAccount::TickArrayBitmapExtension(Box::new(decoded_account)),
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

    use super::{
        amm_config::AmmConfig, observation_state::ObservationState, pool_state::PoolState,
        protocol_position_state::ProtocolPositionState, tick_array_state::TickArrayState, *,
    };
    use crate::types::{Observation, RewardInfo, TickState};
    use solana_pubkey::pubkey;

    #[test]
    fn test_decode_amm_config_account() {
        // Arrange
        let expected_account = AmmConfig {
            bump: 249,
            fund_fee_rate: 40000,
            fund_owner: pubkey!("FundHfY8oo8J9KYGyfXFFuQCHe7Z1VBNmsj84eMcdYs4"),
            index: 4,
            owner: pubkey!("projjosVCPQH49d5em7VYS7fJZzaqKixqKtus7yk416"),
            padding: [0, 0, 0],
            padding_u32: 0,
            protocol_fee_rate: 120000,
            tick_spacing: 1,
            trade_fee_rate: 100,
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/amm_config_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            RaydiumClmmAccount::AmmConfig(account) => {
                assert_eq!(expected_account.bump, account.bump);
                assert_eq!(expected_account.fund_fee_rate, account.fund_fee_rate);
                assert_eq!(expected_account.fund_owner, account.fund_owner);
                assert_eq!(expected_account.index, account.index);
                assert_eq!(expected_account.owner, account.owner);
                assert_eq!(expected_account.padding, account.padding);
                assert_eq!(expected_account.padding_u32, account.padding_u32);
                assert_eq!(
                    expected_account.protocol_fee_rate,
                    account.protocol_fee_rate
                );
                assert_eq!(expected_account.tick_spacing, account.tick_spacing);
                assert_eq!(expected_account.trade_fee_rate, account.trade_fee_rate);
            }
            _ => panic!("Expected AmmConfig"),
        }
    }

    #[test]
    fn test_decode_observation_state_account() {
        // Arrange
        let expected_account = ObservationState {
            initialized: true,
            observation_index: 46,
            observations: [
                Observation {
                    block_timestamp: 1747128365,
                    tick_cumulative: 135450630299,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128389,
                    tick_cumulative: 135450145715,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128405,
                    tick_cumulative: 135449822819,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128420,
                    tick_cumulative: 135449520119,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128438,
                    tick_cumulative: 135449156987,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128454,
                    tick_cumulative: 135448834027,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128469,
                    tick_cumulative: 135448531042,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128496,
                    tick_cumulative: 135447985723,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128525,
                    tick_cumulative: 135447399372,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128552,
                    tick_cumulative: 135446853648,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128567,
                    tick_cumulative: 135446550183,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128586,
                    tick_cumulative: 135446165908,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128604,
                    tick_cumulative: 135445801948,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128629,
                    tick_cumulative: 135445295998,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128647,
                    tick_cumulative: 135444931588,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128692,
                    tick_cumulative: 135444020518,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128714,
                    tick_cumulative: 135443574952,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128733,
                    tick_cumulative: 135443190145,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128749,
                    tick_cumulative: 135442866225,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128764,
                    tick_cumulative: 135442562610,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128779,
                    tick_cumulative: 135442259115,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128794,
                    tick_cumulative: 135441955470,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128812,
                    tick_cumulative: 135441591114,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128832,
                    tick_cumulative: 135441186654,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128848,
                    tick_cumulative: 135440863246,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128865,
                    tick_cumulative: 135440519608,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128909,
                    tick_cumulative: 135439629180,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128925,
                    tick_cumulative: 135439305260,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128945,
                    tick_cumulative: 135438900300,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128974,
                    tick_cumulative: 135438313485,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128989,
                    tick_cumulative: 135438010095,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747129034,
                    tick_cumulative: 135437099970,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747129066,
                    tick_cumulative: 135436452802,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747129082,
                    tick_cumulative: 135436129234,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747129100,
                    tick_cumulative: 135435765328,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747129120,
                    tick_cumulative: 135435360908,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747129138,
                    tick_cumulative: 135434996750,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747129153,
                    tick_cumulative: 135434693210,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747129172,
                    tick_cumulative: 135434308688,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747129200,
                    tick_cumulative: 135433741940,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747129217,
                    tick_cumulative: 135433397792,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747129233,
                    tick_cumulative: 135433073888,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747129249,
                    tick_cumulative: 135432749728,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747129277,
                    tick_cumulative: 135432182308,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747129292,
                    tick_cumulative: 135431878288,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747129328,
                    tick_cumulative: 135431148496,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747129355,
                    tick_cumulative: 135430600720,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127141,
                    tick_cumulative: 135475280103,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127179,
                    tick_cumulative: 135474515163,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127195,
                    tick_cumulative: 135474193067,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127210,
                    tick_cumulative: 135473891222,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127225,
                    tick_cumulative: 135473589437,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127243,
                    tick_cumulative: 135473227331,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127282,
                    tick_cumulative: 135472443041,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127307,
                    tick_cumulative: 135471940316,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127330,
                    tick_cumulative: 135471477901,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127363,
                    tick_cumulative: 135470814205,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127384,
                    tick_cumulative: 135470391769,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127405,
                    tick_cumulative: 135469969165,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127437,
                    tick_cumulative: 135469325261,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127454,
                    tick_cumulative: 135468983340,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127471,
                    tick_cumulative: 135468641487,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127511,
                    tick_cumulative: 135467837167,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127527,
                    tick_cumulative: 135467515423,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127591,
                    tick_cumulative: 135466228255,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127612,
                    tick_cumulative: 135465805798,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127628,
                    tick_cumulative: 135465483862,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127644,
                    tick_cumulative: 135465161798,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127671,
                    tick_cumulative: 135464618288,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127725,
                    tick_cumulative: 135463531214,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127741,
                    tick_cumulative: 135463209054,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127757,
                    tick_cumulative: 135462887150,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127784,
                    tick_cumulative: 135462343856,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127799,
                    tick_cumulative: 135462042011,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127814,
                    tick_cumulative: 135461739941,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127830,
                    tick_cumulative: 135461417861,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127870,
                    tick_cumulative: 135460612781,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127885,
                    tick_cumulative: 135460310771,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127902,
                    tick_cumulative: 135459968408,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127919,
                    tick_cumulative: 135459626198,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127934,
                    tick_cumulative: 135459324188,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127956,
                    tick_cumulative: 135458881306,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747127983,
                    tick_cumulative: 135458337580,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128005,
                    tick_cumulative: 135457894434,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128039,
                    tick_cumulative: 135457209538,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128062,
                    tick_cumulative: 135456746042,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128077,
                    tick_cumulative: 135456443687,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128097,
                    tick_cumulative: 135456040667,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128112,
                    tick_cumulative: 135455738282,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128128,
                    tick_cumulative: 135455415466,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128144,
                    tick_cumulative: 135455092250,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128163,
                    tick_cumulative: 135454707994,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128191,
                    tick_cumulative: 135454142506,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128208,
                    tick_cumulative: 135453799361,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128223,
                    tick_cumulative: 135453496571,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128241,
                    tick_cumulative: 135453133169,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128258,
                    tick_cumulative: 135452789956,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128292,
                    tick_cumulative: 135452103700,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128312,
                    tick_cumulative: 135451700080,
                    padding: [0, 0, 0, 0],
                },
                Observation {
                    block_timestamp: 1747128349,
                    tick_cumulative: 135450953531,
                    padding: [0, 0, 0, 0],
                },
            ],
            padding: [0, 0, 0, 0],
            pool_id: pubkey!("6U4TBh3aJgiJ5EqCDEua4rP75HsqcfHapMKhhyuTqGuo"),
            recent_epoch: 688,
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let account =
            carbon_test_utils::read_account("tests/fixtures/observation_state_account.json")
                .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            RaydiumClmmAccount::ObservationState(account) => {
                assert_eq!(expected_account.initialized, account.initialized);
                assert_eq!(
                    expected_account.observation_index,
                    account.observation_index
                );
                assert_eq!(expected_account.observations, account.observations);
                assert_eq!(expected_account.padding, account.padding);
                assert_eq!(expected_account.pool_id, account.pool_id);
                assert_eq!(expected_account.recent_epoch, account.recent_epoch);
            }
            _ => panic!("Expected ObservationState"),
        }
    }

    #[test]
    fn test_decode_pool_state_account() {
        // Arrange
        let expected_account = PoolState {
            amm_config: pubkey!("9iFER3bpjf1PTTCQCfTRu17EJgvsxo9pVyA9QWwEuX4x"),
            bump: [254],
            fee_growth_global0_x64: 276776890511018,
            fee_growth_global1_x64: 276132430949712850,
            fund_fees_token0: 2474997,
            fund_fees_token1: 2469272531,
            liquidity: 3464101788356,
            mint_decimals0: 9,
            mint_decimals1: 6,
            observation_key: pubkey!("5TtidhhRv3YssrMCQmPiA8am21ZqsLRUgvq4ZHQESpTN"),
            open_time: 0,
            owner: pubkey!("888dRkBFryx1xWqDFf8RYtVyyRrVLeQeZ4XPySicXDv6"),
            padding: [0; 7],
            padding1: [0; 24],
            padding2: [0; 32],
            padding3: 0,
            padding4: 0,
            protocol_fees_token0: 7425061,
            protocol_fees_token1: 7407817781,
            recent_epoch: 786,
            reward_infos: [
                RewardInfo {
                    reward_state: 0,
                    open_time: 0,
                    end_time: 0,
                    last_update_time: 0,
                    emissions_per_second_x64: 0,
                    reward_total_emissioned: 0,
                    reward_claimed: 0,
                    token_mint: pubkey!("11111111111111111111111111111111"),
                    token_vault: pubkey!("11111111111111111111111111111111"),
                    authority: pubkey!("888dRkBFryx1xWqDFf8RYtVyyRrVLeQeZ4XPySicXDv6"),
                    reward_growth_global_x64: 0,
                },
                RewardInfo {
                    reward_state: 0,
                    open_time: 0,
                    end_time: 0,
                    last_update_time: 0,
                    emissions_per_second_x64: 0,
                    reward_total_emissioned: 0,
                    reward_claimed: 0,
                    token_mint: pubkey!("11111111111111111111111111111111"),
                    token_vault: pubkey!("11111111111111111111111111111111"),
                    authority: pubkey!("888dRkBFryx1xWqDFf8RYtVyyRrVLeQeZ4XPySicXDv6"),
                    reward_growth_global_x64: 0,
                },
                RewardInfo {
                    reward_state: 0,
                    open_time: 0,
                    end_time: 0,
                    last_update_time: 0,
                    emissions_per_second_x64: 0,
                    reward_total_emissioned: 0,
                    reward_claimed: 0,
                    token_mint: pubkey!("11111111111111111111111111111111"),
                    token_vault: pubkey!("11111111111111111111111111111111"),
                    authority: pubkey!("888dRkBFryx1xWqDFf8RYtVyyRrVLeQeZ4XPySicXDv6"),
                    reward_growth_global_x64: 0,
                },
            ],
            sqrt_price_x64: 647525941329892376628,
            status: 0,
            swap_in_amount_token0: 618757992853,
            swap_in_amount_token1: 617318155753885,
            swap_out_amount_token0: 580010657141,
            swap_out_amount_token1: 695657958518246,
            tick_array_bitmap: [0; 16],
            tick_current: 71168,
            tick_spacing: 1,
            token_mint0: pubkey!("So11111111111111111111111111111111111111112"),
            token_mint1: pubkey!("4kzLMxfMZVf6UztzjMcAmyLfBYgFcr9FfpwusrqgEjiH"),
            token_vault0: pubkey!("6G5MVC3hqo6oYyucP3F7HK5smyWye84AF8fLExDDMUNL"),
            token_vault1: pubkey!("E1NAfa5JAcNdKEzFiU4SkzX5uVi34pNhLdCJCmjA7nN"),
            total_fees_claimed_token0: 0,
            total_fees_claimed_token1: 0,
            total_fees_token0: 51975748,
            total_fees_token1: 51854725368,
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/pool_state_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            RaydiumClmmAccount::PoolState(account) => {
                assert_eq!(expected_account.amm_config, account.amm_config);
                assert_eq!(expected_account.bump, account.bump);
                assert_eq!(
                    expected_account.fee_growth_global0_x64,
                    account.fee_growth_global0_x64
                );
                assert_eq!(
                    expected_account.fee_growth_global1_x64,
                    account.fee_growth_global1_x64
                );
                assert_eq!(expected_account.fund_fees_token0, account.fund_fees_token0);
                assert_eq!(expected_account.fund_fees_token1, account.fund_fees_token1);
                assert_eq!(expected_account.liquidity, account.liquidity);
                assert_eq!(expected_account.mint_decimals0, account.mint_decimals0);
                assert_eq!(expected_account.mint_decimals1, account.mint_decimals1);
                assert_eq!(expected_account.observation_key, account.observation_key);
                assert_eq!(expected_account.open_time, account.open_time);
                assert_eq!(expected_account.owner, account.owner);
                assert_eq!(expected_account.padding, account.padding);
                assert_eq!(expected_account.padding1, account.padding1);
                assert_eq!(expected_account.padding2, account.padding2);
                assert_eq!(expected_account.padding3, account.padding3);
                assert_eq!(expected_account.padding4, account.padding4);
                assert_eq!(
                    expected_account.protocol_fees_token0,
                    account.protocol_fees_token0
                );
                assert_eq!(
                    expected_account.protocol_fees_token1,
                    account.protocol_fees_token1
                );
                assert_eq!(expected_account.recent_epoch, account.recent_epoch);
                assert_eq!(expected_account.reward_infos, account.reward_infos);
                assert_eq!(expected_account.sqrt_price_x64, account.sqrt_price_x64);
                assert_eq!(expected_account.status, account.status);
                assert_eq!(
                    expected_account.swap_in_amount_token0,
                    account.swap_in_amount_token0
                );
                assert_eq!(
                    expected_account.swap_in_amount_token1,
                    account.swap_in_amount_token1
                );
                assert_eq!(
                    expected_account.swap_out_amount_token0,
                    account.swap_out_amount_token0
                );
                assert_eq!(
                    expected_account.swap_out_amount_token1,
                    account.swap_out_amount_token1
                );
                assert_eq!(
                    expected_account.tick_array_bitmap,
                    account.tick_array_bitmap
                );
                assert_eq!(expected_account.tick_current, account.tick_current);
                assert_eq!(expected_account.tick_spacing, account.tick_spacing);
                assert_eq!(expected_account.token_mint0, account.token_mint0);
                assert_eq!(expected_account.token_mint1, account.token_mint1);
                assert_eq!(expected_account.token_vault0, account.token_vault0);
                assert_eq!(expected_account.token_vault1, account.token_vault1);
                assert_eq!(
                    expected_account.total_fees_claimed_token0,
                    account.total_fees_claimed_token0
                );
                assert_eq!(
                    expected_account.total_fees_claimed_token1,
                    account.total_fees_claimed_token1
                );
                assert_eq!(
                    expected_account.total_fees_token0,
                    account.total_fees_token0
                );
                assert_eq!(
                    expected_account.total_fees_token1,
                    account.total_fees_token1
                );
            }
            _ => panic!("Expected PoolState"),
        }
    }
    #[test]
    fn test_decode_protocol_position_state_account() {
        // Arrange
        let expected_account = ProtocolPositionState {
            bump: 255,
            fee_growth_inside0_last_x64: 0,
            fee_growth_inside1_last_x64: 614911157,
            liquidity: 0,
            padding: [0; 7],
            pool_id: pubkey!("2AP2wU8HnpsxJ2ErSjWeSRzG1HLootMoisthJouWJLka"),
            recent_epoch: 786,
            reward_growth_inside: [0, 0, 0],
            tick_lower_index: -13856,
            tick_upper_index: -13852,
            token_fees_owed0: 0,
            token_fees_owed1: 7391,
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let account =
            carbon_test_utils::read_account("tests/fixtures/protocol_position_state_account.json")
                .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            RaydiumClmmAccount::ProtocolPositionState(account) => {
                assert_eq!(expected_account.bump, account.bump);
                assert_eq!(
                    expected_account.fee_growth_inside0_last_x64,
                    account.fee_growth_inside0_last_x64
                );
                assert_eq!(
                    expected_account.fee_growth_inside1_last_x64,
                    account.fee_growth_inside1_last_x64
                );
                assert_eq!(expected_account.liquidity, account.liquidity);
                assert_eq!(expected_account.padding, account.padding);
                assert_eq!(expected_account.pool_id, account.pool_id);
                assert_eq!(expected_account.recent_epoch, account.recent_epoch);
                assert_eq!(
                    expected_account.reward_growth_inside,
                    account.reward_growth_inside
                );
                assert_eq!(expected_account.tick_lower_index, account.tick_lower_index);
                assert_eq!(expected_account.tick_upper_index, account.tick_upper_index);
                assert_eq!(expected_account.token_fees_owed0, account.token_fees_owed0);
                assert_eq!(expected_account.token_fees_owed1, account.token_fees_owed1);
            }
            _ => panic!("Expected ProtocolPositionState"),
        }
    }

    #[test]
    fn test_decode_tick_array_state_account() {
        // Arrange
        let expected_account = TickArrayState {
            initialized_tick_count: 0,
            padding: [0; 107],
            pool_id: pubkey!("2AP2wU8HnpsxJ2ErSjWeSRzG1HLootMoisthJouWJLka"),
            recent_epoch: 786,
            start_tick_index: -13860,
            ticks: [
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: -13856,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: -13852,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
                TickState {
                    tick: 0,
                    liquidity_net: 0,
                    liquidity_gross: 0,
                    fee_growth_outside0_x64: 0,
                    fee_growth_outside1_x64: 0,
                    reward_growths_outside_x64: [0; 3],
                    padding: [0; 13],
                },
            ],
        };

        // Act
        let decoder = RaydiumClmmDecoder;
        let account =
            carbon_test_utils::read_account("tests/fixtures/tick_array_state_account.json")
                .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            RaydiumClmmAccount::TickArrayState(account) => {
                assert_eq!(
                    expected_account.initialized_tick_count,
                    account.initialized_tick_count
                );
                assert_eq!(expected_account.padding, account.padding);
                assert_eq!(expected_account.pool_id, account.pool_id);
                assert_eq!(expected_account.recent_epoch, account.recent_epoch);
                assert_eq!(expected_account.start_tick_index, account.start_tick_index);
                assert_eq!(expected_account.ticks, account.ticks);
            }
            _ => panic!("Expected TickArrayState"),
        }
    }
}
