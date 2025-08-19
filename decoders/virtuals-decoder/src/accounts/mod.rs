use {
    super::VirtualsDecoder,
    crate::PROGRAM_ID,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod virtuals_pool;

pub enum VirtualsAccount {
    VirtualsPool(virtuals_pool::VirtualsPool),
}

impl AccountDecoder<'_> for VirtualsDecoder {
    type AccountType = VirtualsAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            virtuals_pool::VirtualsPool::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: VirtualsAccount::VirtualsPool(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
