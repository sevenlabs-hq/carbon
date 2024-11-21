

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x2477fb8122f00793")]
pub struct RequestElevationGroup{
    pub elevation_group: u8,
}

pub struct RequestElevationGroupInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub obligation: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RequestElevationGroup {
    type ArrangedAccounts = RequestElevationGroupInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let obligation = accounts.get(1)?;
        let lending_market = accounts.get(2)?;

        Some(RequestElevationGroupInstructionAccounts {
            owner: owner.pubkey,
            obligation: obligation.pubkey,
            lending_market: lending_market.pubkey,
        })
    }
}