use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use crate::PROGRAM_ID;

use super::PumpfunDecoder;
pub mod bonding_curve;
pub mod global;
pub mod last_withdraw;

pub enum PumpAccount {
    BondingCurve(bonding_curve::BondingCurve),
    Global(global::Global),
    LastWithdraw(last_withdraw::LastWithdraw),
}

impl<'a> AccountDecoder<'a> for PumpfunDecoder {
    type AccountType = PumpAccount;
    fn decode_account(
        &self,
        account: &solana_sdk::account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            bonding_curve::BondingCurve::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpAccount::BondingCurve(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = global::Global::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpAccount::Global(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            last_withdraw::LastWithdraw::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpAccount::LastWithdraw(decoded_account),
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
    use super::*;

    #[test]
    fn test_decode_bonding_curve_account() {
        // Arrange
        let expected_bonding_curve = bonding_curve::BondingCurve {
            virtual_token_reserves: 1072906494066221,
            virtual_sol_reserves: 30002615555,
            real_token_reserves: 793006494066221,
            real_sol_reserves: 2615555,
            token_total_supply: 1000000000000000,
            complete: false,
        };

        // Act
        let decoder = PumpfunDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/bonding_curve_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            PumpAccount::BondingCurve(bonding_curve) => {
                assert_eq!(
                    expected_bonding_curve.virtual_token_reserves,
                    bonding_curve.virtual_token_reserves
                );
                assert_eq!(
                    expected_bonding_curve.virtual_sol_reserves,
                    bonding_curve.virtual_sol_reserves
                );
                assert_eq!(
                    expected_bonding_curve.real_token_reserves,
                    bonding_curve.real_token_reserves
                );
                assert_eq!(
                    expected_bonding_curve.real_sol_reserves,
                    bonding_curve.real_sol_reserves
                );
                assert_eq!(
                    expected_bonding_curve.token_total_supply,
                    bonding_curve.token_total_supply
                );
                assert_eq!(expected_bonding_curve.complete, bonding_curve.complete);
            }
            _ => panic!("Expected BondingCurve"),
        }
    }
}
