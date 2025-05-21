use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use crate::PROGRAM_ID;

use super::PumpfunDecoder;
pub mod bonding_curve;
pub mod global;

#[allow(clippy::large_enum_variant)]
pub enum PumpfunAccount {
    BondingCurve(bonding_curve::BondingCurve),
    Global(global::Global),
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

        if let Some(decoded_account) =
            bonding_curve::BondingCurve::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: PumpfunAccount::BondingCurve(decoded_account),
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

        None
    }
}

#[cfg(test)]
mod tests {
    use solana_pubkey::pubkey;

    use super::*;

    #[test]
    fn test_decode_bonding_curve_account() {
        // Arrange
        let expected_bonding_curve = bonding_curve::BondingCurve {
            virtual_token_reserves: 1072911112000000,
            virtual_sol_reserves: 30002485430,
            real_token_reserves: 793011112000000,
            real_sol_reserves: 2485430,
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
            PumpfunAccount::BondingCurve(bonding_curve) => {
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
            authority: pubkey!("FFWtrEQ4B4PKQoVuHYzZq8FabGkVatYzDpEVHsK5rrhF"),
            withdraw_authority: pubkey!("39azUYFWPz3VHgKCf3VChUwbpURdCHRxjWVowf5jUJjg"),
            fee_recipient: pubkey!("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
            initial_virtual_token_reserves: 1073000000000000,
            initial_virtual_sol_reserves: 30000000000,
            initial_real_token_reserves: 793100000000000,
            token_total_supply: 1000000000000000,
            fee_basis_points: 95,
            pool_migration_fee: 15000001,
            enable_migrate: true,
            fee_recipients: [
                pubkey!("7VtfL8fvgNfhz17qKRMjzQEXgbdpnHHHQRh54R9jP2RJ"),
                pubkey!("7hTckgnGnLQR6sdH7YkqFTAA7VwTfYFaZ6EhEsU3saCX"),
                pubkey!("9rPYyANsfQZw3DnDmKE3YCQF5E8oD89UXoHn9JFEhJUz"),
                pubkey!("AVmoTthdrX6tKt4nDjco2D775W2YK3sDhxPcMmzUAmTY"),
                pubkey!("CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM"),
                pubkey!("FWsW1xNtWscwNmKv6wVsU1iTzRN6wmmk3MjxRP5tT7hz"),
                pubkey!("G5UZAVbAf46s7cKWoyKu8kYTip9DGTpbLZ2qa9Aq69dP"),
            ],
            ..Default::default()
        };

        // Act
        let decoder = PumpfunDecoder;
        let account = carbon_test_utils::read_account("tests/fixtures/global_account.json")
            .expect("read fixture");
        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        // Assert
        match decoded_account.data {
            PumpfunAccount::Global(global_account) => {
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
}
