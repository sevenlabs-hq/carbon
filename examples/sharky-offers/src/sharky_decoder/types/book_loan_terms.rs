
use super::*;

use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum BookLoanTerms {
    Fixed
                {
                    terms: LoanTermsSpec,
                }
    ,
    LenderChooses,
}


