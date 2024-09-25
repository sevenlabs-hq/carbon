 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use crate::JupiterDecoder; 
pub mod token_ledger; 

pub enum JupiterAccount { 
        TokenLedger(token_ledger::TokenLedger), 
}


impl AccountDecoder for JupiterDecoder { 
    type AccountType = JupiterAccount;
     fn decode_accounts( &self, account: solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = token_ledger::TokenLedger::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: JupiterAccount::TokenLedger(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}