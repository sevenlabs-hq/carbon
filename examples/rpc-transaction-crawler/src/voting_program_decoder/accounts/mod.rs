 
use carbon_core::account::AccountDecoder; 
use carbon_core::deserialize::CarbonDeserialize;
 

use super::VotingProgramDecoder; 
pub mod vote; 
pub mod vote_receipt; 

pub enum VotingProgramAccount { 
        Vote(vote::Vote), 
        VoteReceipt(vote_receipt::VoteReceipt), 
}


impl AccountDecoder for VotingProgramDecoder { 
    type AccountType = VotingProgramAccount;
     fn decode_account( &self, account: solana_sdk::account::Account, ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> { 
         
            if let Some(decoded_account) = vote::Vote::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: VotingProgramAccount::Vote(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
            if let Some(decoded_account) = vote_receipt::VoteReceipt::deserialize(account.data.as_slice()) { 
            return Some(carbon_core::account::DecodedAccount { 
                lamports: account.lamports, 
                data: VotingProgramAccount::VoteReceipt(decoded_account), 
                owner: account.owner, 
                executable: account.executable, 
                rent_epoch: account.rent_epoch, 
            }); 
        } 
         
    None 
    } 
}