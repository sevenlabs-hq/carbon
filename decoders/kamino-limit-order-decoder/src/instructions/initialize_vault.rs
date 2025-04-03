use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x30bfa32c47813fa4")]
pub struct InitializeVault {}

pub struct InitializeVaultInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub pda_authority: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeVault {
    type ArrangedAccounts = InitializeVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, global_config, pda_authority, mint, vault, token_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeVaultInstructionAccounts {
            payer: payer.pubkey,
            global_config: global_config.pubkey,
            pda_authority: pda_authority.pubkey,
            mint: mint.pubkey,
            vault: vault.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
