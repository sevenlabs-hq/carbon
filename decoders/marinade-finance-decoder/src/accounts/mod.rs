 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::MarinadeFinanceDecoder; 
pub mod ticket_account_data; 
pub mod state; 

pub enum MarinadeFinanceAccount { 
        TicketAccountData(ticket_account_data::TicketAccountData), 
        State(state::State), 
}


impl<'a> AccountDecoder<'a> for MarinadeFinanceDecoder { 
    type AccountType = MarinadeFinanceAccount;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = ticket_account_data::TicketAccountData::deserialize(account.data.as_slice()) { 
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