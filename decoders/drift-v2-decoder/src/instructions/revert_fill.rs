use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xeceeb045ef0ab5c1")]
pub struct RevertFill {}

pub struct RevertFillInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub filler: solana_pubkey::Pubkey,
    pub filler_stats: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RevertFill {
    type ArrangedAccounts = RevertFillInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, authority, filler, filler_stats, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RevertFillInstructionAccounts {
            state: state.pubkey,
            authority: authority.pubkey,
            filler: filler.pubkey,
            filler_stats: filler_stats.pubkey,
        })
    }
}
