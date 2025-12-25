use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use crate::PROGRAM_ID;

use super::PumpfunDecoder;
pub mod bonding_curve;
pub mod fee_config;
pub mod global;
pub mod global_volume_accumulator;
pub mod user_volume_accumulator;

pub enum PumpfunAccount {
    BondingCurve(bonding_curve::BondingCurve),
    FeeConfig(fee_config::FeeConfig),
    Global(global::Global),
    GlobalVolumeAccumulator(global_volume_accumulator::GlobalVolumeAccumulator),
    UserVolumeAccumulator(user_volume_accumulator::UserVolumeAccumulator),
}

impl AccountDecoder<'_> for PumpfunDecoder {
    type AccountType = PumpfunAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        {
            let mut data = account.data.clone();

            if data.len() == bonding_curve::BONDING_CURVE_LEN_PRE_MAYHEM_MODE {
                // Append a byte for is_mayhem_mode if it's missing
                data.push(0);
            }

            if let Some(decoded_account) = bonding_curve::BondingCurve::deserialize(data.as_slice())
            {
                return Some(carbon_core::account::DecodedAccount {
                    lamports: account.lamports,
                    data: PumpfunAccount::BondingCurve(decoded_account),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        if let Some(decoded_account) = fee_config::FeeConfig::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpfunAccount::FeeConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = global::Global::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpfunAccount::Global(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            global_volume_accumulator::GlobalVolumeAccumulator::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpfunAccount::GlobalVolumeAccumulator(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            user_volume_accumulator::UserVolumeAccumulator::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpfunAccount::UserVolumeAccumulator(decoded_account),
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
    use crate::types::OptionBool;

    use super::*;

    #[test]
    fn test_decode_bonding_curve_account() {
        let expected_bonding_curve_account = bonding_curve::BondingCurve {
            virtual_token_reserves: 1_072_911_112_000_000,
            virtual_sol_reserves: 30_002_485_430,
            real_token_reserves: 793_011_112_000_000,
            real_sol_reserves: 2_485_430,
            token_total_supply: 1_000_000_000_000_000,
            complete: false,
            creator: solana_pubkey::Pubkey::new_from_array([
                0xae, 0x9e, 0x3d, 0x67, 0x5f, 0xf2, 0x40, 0x8c, 0x28, 0xca, 0x99, 0x5d, 0x9a, 0x70,
                0x3a, 0x0a, 0x56, 0x37, 0x34, 0xc2, 0x4b, 0x58, 0x8c, 0x99, 0x4f, 0x3d, 0x7f, 0xeb,
                0x74, 0x43, 0x96, 0x19,
            ]),
            is_mayhem_mode: OptionBool(false),
        };

        let decoder = PumpfunDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/bonding_curve_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        match decoded_account.data {
            PumpfunAccount::BondingCurve(bonding_curve) => {
                assert_eq!(bonding_curve, expected_bonding_curve_account);
            }
            _ => panic!("expected BondingCurve account"),
        }
    }

    #[test]
    fn test_decode_bonding_curve_account_without_mayhem_mode_byte() {
        let expected_bonding_curve_account = bonding_curve::BondingCurve {
            virtual_token_reserves: 1_072_911_112_000_000,
            virtual_sol_reserves: 30_002_485_430,
            real_token_reserves: 793_011_112_000_000,
            real_sol_reserves: 2_485_430,
            token_total_supply: 1_000_000_000_000_000,
            complete: false,
            creator: solana_pubkey::Pubkey::new_from_array([
                0xae, 0x9e, 0x3d, 0x67, 0x5f, 0xf2, 0x40, 0x8c, 0x28, 0xca, 0x99, 0x5d, 0x9a, 0x70,
                0x3a, 0x0a, 0x56, 0x37, 0x34, 0xc2, 0x4b, 0x58, 0x8c, 0x99, 0x4f, 0x3d, 0x7f, 0xeb,
                0x74, 0x43, 0x96, 0x19,
            ]),
            is_mayhem_mode: OptionBool(false),
        };

        let decoder = PumpfunDecoder;
        let mut account =
            carbon_test_utils::read_account("tests/fixtures/bonding_curve_account.json")
                .expect("read fixture");

        account
            .data
            .truncate(bonding_curve::BONDING_CURVE_LEN_PRE_MAYHEM_MODE);

        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        match decoded_account.data {
            PumpfunAccount::BondingCurve(bonding_curve) => {
                assert_eq!(bonding_curve, expected_bonding_curve_account);
            }
            _ => panic!("expected BondingCurve account"),
        }
    }

    #[test]
    fn test_decode_bonding_curve_account_mayhem_mode_true() {
        let decoder = PumpfunDecoder;
        let mut account =
            carbon_test_utils::read_account("tests/fixtures/bonding_curve_account.json")
                .expect("read fixture");

        account.data[bonding_curve::BONDING_CURVE_LEN_PRE_MAYHEM_MODE] = 1;

        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        match decoded_account.data {
            PumpfunAccount::BondingCurve(bonding_curve) => {
                assert!(bonding_curve.is_mayhem_mode.0);
            }
            _ => panic!("expected BondingCurve account"),
        }
    }
}
