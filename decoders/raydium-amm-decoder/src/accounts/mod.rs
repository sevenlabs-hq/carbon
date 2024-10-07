 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::RaydiumAmmDecoder; 
pub mod target_orders; 
pub mod fees; 
pub mod amm_info; 

pub enum RaydiumAmmAccount { 
        TargetOrders(target_orders::TargetOrders), 
        Fees(fees::Fees), 
        AmmInfo(amm_info::AmmInfo), 
}


impl AccountDecoder for RaydiumAmmDecoder { 
    type AccountType = RaydiumAmmAccount;
     fn decode_account( &self, account: solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = target_orders::TargetOrders::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: RaydiumAmmAccount::TargetOrders(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = fees::Fees::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: RaydiumAmmAccount::Fees(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = amm_info::AmmInfo::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: RaydiumAmmAccount::AmmInfo(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}