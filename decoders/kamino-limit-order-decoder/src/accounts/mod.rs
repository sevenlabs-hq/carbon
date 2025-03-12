 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::KaminoLimitOrderDecoder; 
pub mod order; 
pub mod global_config; 

pub enum KaminoLimitOrderAccount { 
        Order(order::Order), 
        GlobalConfig(global_config::GlobalConfig), 
}


impl<'a> AccountDecoder<'a> for KaminoLimitOrderDecoder { 
    type AccountType = KaminoLimitOrderAccount;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = order::Order::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: KaminoLimitOrderAccount::Order(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = global_config::GlobalConfig::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: KaminoLimitOrderAccount::GlobalConfig(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}