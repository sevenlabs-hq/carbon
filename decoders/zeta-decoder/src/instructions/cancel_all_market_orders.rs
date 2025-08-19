use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8bbee6f94da0ce04")]
pub struct CancelAllMarketOrders {
    pub asset: Asset,
}

pub struct CancelAllMarketOrdersInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub cancel_accounts: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelAllMarketOrders {
    type ArrangedAccounts = CancelAllMarketOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, cancel_accounts, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CancelAllMarketOrdersInstructionAccounts {
            authority: authority.pubkey,
            cancel_accounts: cancel_accounts.pubkey,
        })
    }
}
