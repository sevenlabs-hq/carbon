 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::KaminoFarmsDecoder; 
pub mod farm_state; 
pub mod global_config; 
pub mod user_state; 
pub mod oracle_prices; 

pub enum KaminoFarmsAccount { 
        FarmState(farm_state::FarmState), 
        GlobalConfig(global_config::GlobalConfig), 
        UserState(user_state::UserState), 
        OraclePrices(oracle_prices::OraclePrices), 
}


impl<'a> AccountDecoder<'a> for KaminoFarmsDecoder { 
    type AccountType = KaminoFarmsAccount;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = farm_state::FarmState::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: KaminoFarmsAccount::FarmState(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = global_config::GlobalConfig::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: KaminoFarmsAccount::GlobalConfig(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = user_state::UserState::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: KaminoFarmsAccount::UserState(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = oracle_prices::OraclePrices::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: KaminoFarmsAccount::OraclePrices(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}