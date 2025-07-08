use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x365393c67b61da48")]
pub struct TogglePaused {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TogglePausedInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TogglePaused {
    type ArrangedAccounts = TogglePausedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, config, _remaining @ ..] = accounts else {
            return None;
        };

        Some(TogglePausedInstructionAccounts {
            authority: authority.pubkey,
            config: config.pubkey,
        })
    }
}
