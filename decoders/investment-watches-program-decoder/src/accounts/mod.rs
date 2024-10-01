 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use crate::InvestmentWatchesProgramDecoder; 
pub mod micro_share; 
pub mod watch; 

pub enum InvestmentWatchesProgramAccount { 
        MicroShare(micro_share::MicroShare), 
        Watch(watch::Watch), 
}


impl AccountDecoder for InvestmentWatchesProgramDecoder { 
    type AccountType = InvestmentWatchesProgramAccount;
     fn decode_account( &self, account: solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = micro_share::MicroShare::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: InvestmentWatchesProgramAccount::MicroShare(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = watch::Watch::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: InvestmentWatchesProgramAccount::Watch(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}