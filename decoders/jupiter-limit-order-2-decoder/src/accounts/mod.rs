 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::JupiterLimitOrder2Decoder; 
pub mod order; 
pub mod fee; 

pub enum JupiterLimitOrder2Account { 
        Order(order::Order), 
        Fee(fee::Fee), 
}


impl<'a> AccountDecoder<'a> for JupiterLimitOrder2Decoder { 
    type AccountType = JupiterLimitOrder2Account;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = order::Order::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: JupiterLimitOrder2Account::Order(decoded_account), 
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
