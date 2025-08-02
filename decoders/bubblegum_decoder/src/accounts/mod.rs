use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::BubblegumDecoder;
pub mod tree_config;
pub mod voucher;

pub enum BubblegumAccount {
    TreeConfig(tree_config::TreeConfig),
    Voucher(voucher::Voucher),
}

impl AccountDecoder<'_> for BubblegumDecoder {
    type AccountType = BubblegumAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) = tree_config::TreeConfig::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: BubblegumAccount::TreeConfig(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = voucher::Voucher::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: BubblegumAccount::Voucher(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
