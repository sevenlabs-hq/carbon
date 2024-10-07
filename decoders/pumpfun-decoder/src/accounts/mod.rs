 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::PumpDecoder; 
pub mod global; 
pub mod last_withdraw; 
pub mod bonding_curve; 

pub enum PumpAccount { 
        Global(global::Global), 
        LastWithdraw(last_withdraw::LastWithdraw), 
        BondingCurve(bonding_curve::BondingCurve), 
}


impl AccountDecoder for PumpDecoder { 
    type AccountType = PumpAccount;
     fn decode_account( &self, account: solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = global::Global::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: PumpAccount::Global(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = last_withdraw::LastWithdraw::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: PumpAccount::LastWithdraw(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = bonding_curve::BondingCurve::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: PumpAccount::BondingCurve(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}