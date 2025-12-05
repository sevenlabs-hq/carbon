use {
    super::JupiterLimitOrder2Decoder,
    crate::PROGRAM_ID,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod fee;
pub mod order;

pub enum JupiterLimitOrder2Account {
    Order(Box<order::Order>),
    Fee(fee::Fee),
}

impl AccountDecoder<'_> for JupiterLimitOrder2Decoder {
    type AccountType = JupiterLimitOrder2Account;
    fn decode_account(
        &self,
        account: &'_ solana_account::Account,
        _metadata: Option<&carbon_core::account::AccountMetadata>,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) = order::Order::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: JupiterLimitOrder2Account::Order(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = fee::Fee::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: JupiterLimitOrder2Account::Fee(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
