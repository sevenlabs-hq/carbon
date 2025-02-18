use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdb52dbec3c73c540")]
pub struct SetMarketExpired {}

pub struct SetMarketExpiredInstructionAccounts {
    pub close_market_admin: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetMarketExpired {
    type ArrangedAccounts = SetMarketExpiredInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let close_market_admin = accounts.get(0)?;
        let market = accounts.get(1)?;

        Some(SetMarketExpiredInstructionAccounts {
            close_market_admin: close_market_admin.pubkey,
            market: market.pubkey,
        })
    }
}
