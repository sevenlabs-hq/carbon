use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7a7f315944e4559d")]
pub struct CleanZetaMarkets {}

pub struct CleanZetaMarketsInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CleanZetaMarkets {
    type ArrangedAccounts = CleanZetaMarketsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, zeta_group, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CleanZetaMarketsInstructionAccounts {
            state: state.pubkey,
            zeta_group: zeta_group.pubkey,
        })
    }
}
