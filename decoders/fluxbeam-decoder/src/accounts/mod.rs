use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

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
