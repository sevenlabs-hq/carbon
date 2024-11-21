

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf54b5b00ec611303")]
pub struct SocializeLoss{
    pub liquidity_amount: u64,
}

pub struct SocializeLossInstructionAccounts {
    pub risk_council: solana_sdk::pubkey::Pubkey,
    pub obligation: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SocializeLoss {
    type ArrangedAccounts = SocializeLossInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let risk_council = accounts.get(0)?;
        let obligation = accounts.get(1)?;
        let lending_market = accounts.get(2)?;
        let reserve = accounts.get(3)?;
        let instruction_sysvar_account = accounts.get(4)?;

        Some(SocializeLossInstructionAccounts {
            risk_council: risk_council.pubkey,
            obligation: obligation.pubkey,
            lending_market: lending_market.pubkey,
            reserve: reserve.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}