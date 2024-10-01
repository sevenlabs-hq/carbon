use crate::types::*;
use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x19670f3f58024dbf")]
pub struct RedeemMicroshare {}

pub struct RedeemMicroshareInstructionAccounts {
    pub watch: solana_sdk::pubkey::Pubkey,
    pub redeem_token_account: solana_sdk::pubkey::Pubkey,
    pub microshare_account: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub owner_token_account: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub core_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for RedeemMicroshare {
    type ArrangedAccounts = RedeemMicroshareInstructionAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let watch = accounts.get(0)?;
        let redeem_token_account = accounts.get(1)?;
        let microshare_account = accounts.get(2)?;
        let authority = accounts.get(3)?;
        let owner = accounts.get(4)?;
        let owner_token_account = accounts.get(5)?;
        let token_mint = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let core_program = accounts.get(8)?;
        let token_program = accounts.get(9)?;

        Some(RedeemMicroshareInstructionAccounts {
            watch: *watch,
            redeem_token_account: *redeem_token_account,
            microshare_account: *microshare_account,
            authority: *authority,
            owner: *owner,
            owner_token_account: *owner_token_account,
            token_mint: *token_mint,
            system_program: *system_program,
            core_program: *core_program,
            token_program: *token_program,
        })
    }
}
