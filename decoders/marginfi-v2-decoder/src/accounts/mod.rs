 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::MarginfiV2Decoder; 
pub mod marginfi_account; 
pub mod marginfi_group; 
pub mod bank; 

pub enum MarginfiV2Account { 
        MarginfiAccount(marginfi_account::MarginfiAccount), 
        MarginfiGroup(marginfi_group::MarginfiGroup), 
        Bank(bank::Bank), 
}


impl<'a> AccountDecoder<'a> for MarginfiV2Decoder { 
    type AccountType = MarginfiV2Account;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = marginfi_account::MarginfiAccount::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MarginfiV2Account::MarginfiAccount(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = marginfi_group::MarginfiGroup::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MarginfiV2Account::MarginfiGroup(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = bank::Bank::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: MarginfiV2Account::Bank(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}