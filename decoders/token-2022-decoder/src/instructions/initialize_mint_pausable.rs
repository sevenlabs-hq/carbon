use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
#[carbon(discriminator = "0x2C")]
pub struct InitializeMintPausable {
    pub authority: solana_pubkey::Pubkey,
}

pub struct InitializeMintPausableInstructionAccounts {
    pub mint_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeMintPausable {
    type ArrangedAccounts = InitializeMintPausableInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeMintPausableInstructionAccounts {
            mint_account: mint.pubkey,
        })
    }
}
