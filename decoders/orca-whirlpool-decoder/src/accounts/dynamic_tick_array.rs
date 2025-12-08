use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x11d8f68ee1c7da38")]
pub struct DynamicTickArray {
    pub start_tick_index: i32,
    pub whirlpool: solana_pubkey::Pubkey,
    pub tick_bitmap: u128,
    #[serde(with = "serde_big_array::BigArray")]
    pub ticks: [DynamicTick; 88],
}

#[cfg(test)]
mod tests {
    use carbon_core::account::AccountDecoder;

    use crate::{accounts::WhirlpoolAccount, types::DynamicTick, OrcaWhirlpoolDecoder};

    #[test]
    fn test_decode_dynamic_tick_array_accounts() {
        let decoder = OrcaWhirlpoolDecoder;
        let account =
            carbon_test_utils::read_account("tests/fixtures/dynamic_tick_array_account0.json")
                .expect("read fixture");

        let decoded_account = decoder.decode_account(&account).expect("decode fixture");

        match decoded_account.data {
            WhirlpoolAccount::DynamicTickArray(account) => {
                println!("{account:#?}");
                for i in 0..account.ticks.len() {
                    let tick = account.ticks[i];
                    let is_uninitialized = account.tick_bitmap & (1 << i) == 0;

                    if is_uninitialized {
                        assert_eq!(tick, DynamicTick::Uninitialized);
                    }
                }
            }
            _ => panic!("Expected DynamicTickArray account"),
        }
    }
}
