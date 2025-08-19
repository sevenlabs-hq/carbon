use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
#[carbon(discriminator = "0x2C")]
pub struct ToggleMintPause {
    pub pause: bool,
}

pub struct ToggleMintPauseInstructionAccounts {
    pub mint_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ToggleMintPause {
    type ArrangedAccounts = ToggleMintPauseInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ToggleMintPauseInstructionAccounts {
            mint_account: mint.pubkey,
            authority: authority.pubkey,
        })
    }
}
