
use super::*;

use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct LoanOffer {
    pub lender_wallet: solana_sdk::pubkey::Pubkey,
    pub terms_spec: LoanTermsSpec,
    pub offer_time: i64,
}
