use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use crate::PROGRAM_ID;

use super::PumpfunDecoder;
pub mod bonding_curve;
pub mod global;
pub mod last_withdraw;

#[allow(clippy::large_enum_variant)]
pub enum PumpAccount {
    BondingCurve(bonding_curve::BondingCurve),
    Global(global::Global),
    LastWithdraw(last_withdraw::LastWithdraw),
}

impl AccountDecoder<'_> for PumpfunDecoder {
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
    use solana_sdk::pubkey;

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

    #[test]
    fn test_decode_global_account() {
        // Arrange
        let expected_global_account = global::Global {
            initialized: true,
            authority: pubkey!("DCpJReAfonSrgohiQbTmKKbjbqVofspFRHz9yQikzooP"),
            fee_recipient: pubkey!("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
            initial_virtual_token_reserves: 1073000000000000,
            initial_virtual_sol_reserves: 30000000000,
            initial_real_token_reserves: 793100000000000,
            token_total_supply: 1000000000000000,
            fee_basis_points: 100,
            ..Default::default()
        };

        // Act
        let decoder = PumpfunDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/global_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            PumpAccount::Global(global_account) => {
                assert_eq!(
                    expected_global_account.initialized,
                    global_account.initialized
                );
                assert_eq!(expected_global_account.authority, global_account.authority);
                assert_eq!(
                    expected_global_account.fee_recipient,
                    global_account.fee_recipient
                );
                assert_eq!(
                    expected_global_account.initial_virtual_token_reserves,
                    global_account.initial_virtual_token_reserves
                );
                assert_eq!(
                    expected_global_account.initial_virtual_sol_reserves,
                    global_account.initial_virtual_sol_reserves
                );
                assert_eq!(
                    expected_global_account.initial_real_token_reserves,
                    global_account.initial_real_token_reserves
                );
                assert_eq!(
                    expected_global_account.token_total_supply,
                    global_account.token_total_supply
                );
                assert_eq!(
                    expected_global_account.fee_basis_points,
                    global_account.fee_basis_points
                );
            }
            _ => panic!("Expected Global"),
        }
    }

    #[test]
    fn test_decode_last_withdraw_account() {
        // Arrange
        let expected_last_withdraw_account = last_withdraw::LastWithdraw {
            last_withdraw_timestamp: 1741550682,
        };

        // Act
        let decoder = PumpfunDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/last_withdraw_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            PumpAccount::LastWithdraw(last_withdraw) => {
                assert_eq!(
                    expected_last_withdraw_account.last_withdraw_timestamp,
                    last_withdraw.last_withdraw_timestamp
                );
            }
            _ => panic!("Expected LastWithdraw"),
        }
    }
}
