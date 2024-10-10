 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::PumpfunDecoder; 
pub mod global; 
pub mod last_withdraw; 
pub mod bonding_curve; 

pub enum PumpfunAccount { 
        Global(global::Global), 
        LastWithdraw(last_withdraw::LastWithdraw), 
        BondingCurve(bonding_curve::BondingCurve), 
}


impl<'a> AccountDecoder<'a> for PumpfunDecoder { 
    type AccountType = PumpfunAccount;
     fn decode_account( &self, account: &solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = global::Global::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: PumpfunAccount::Global(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = last_withdraw::LastWithdraw::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: PumpfunAccount::LastWithdraw(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = bonding_curve::BondingCurve::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: PumpfunAccount::BondingCurve(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}
