use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9d356b68b8bd64dc")]
pub struct MigrateToCrossMarginAccount {}

pub struct MigrateToCrossMarginAccountInstructionAccounts {
    pub cross_margin_account: solana_pubkey::Pubkey,
    pub pricing: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MigrateToCrossMarginAccount {
    type ArrangedAccounts = MigrateToCrossMarginAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [cross_margin_account, pricing, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(MigrateToCrossMarginAccountInstructionAccounts {
            cross_margin_account: cross_margin_account.pubkey,
            pricing: pricing.pubkey,
            authority: authority.pubkey,
        })
    }
}
