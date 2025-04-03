use {
    super::MarinadeFinanceDecoder,
    crate::PROGRAM_ID,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod state;
pub mod ticket_account_data;

pub enum MarinadeFinanceAccount {
    TicketAccountData(ticket_account_data::TicketAccountData),
    State(state::State),
}

impl AccountDecoder<'_> for MarinadeFinanceDecoder {
    type AccountType = MarinadeFinanceAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }
        if let Some(decoded_account) =
            ticket_account_data::TicketAccountData::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MarinadeFinanceAccount::TicketAccountData(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = state::State::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MarinadeFinanceAccount::State(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
