

use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum LoanTerms {
    Time
                {
                    start: i64,
                    duration: u64,
                    total_owed_lamports: u64,
                }
    ,
}


