use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x76e00a3ec4e6b859")]
pub struct UpdateLendingMarketOwner {}

pub struct UpdateLendingMarketOwnerInstructionAccounts {
    pub lending_market_owner_cached: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateLendingMarketOwner {
    type ArrangedAccounts = UpdateLendingMarketOwnerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lending_market_owner_cached, lending_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateLendingMarketOwnerInstructionAccounts {
            lending_market_owner_cached: lending_market_owner_cached.pubkey,
            lending_market: lending_market.pubkey,
        })
    }
}
