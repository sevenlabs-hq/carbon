use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x032ca4b87b0df5b3")]
pub struct TokenMint {
    pub mint_params: TokenMintParams,
}

#[derive(Debug)]
pub struct TokenMintInstructionAccounts {
    pub sender: solana_sdk::pubkey::Pubkey,
    pub backend_authority: solana_sdk::pubkey::Pubkey,
    pub curve_account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub mint_metadata: solana_sdk::pubkey::Pubkey,
    pub curve_token_account: solana_sdk::pubkey::Pubkey,
    pub config_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub mpl_token_metadata: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TokenMint {
    type ArrangedAccounts = TokenMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [sender, backend_authority, curve_account, mint, mint_metadata, curve_token_account, config_account, token_program, associated_token_program, mpl_token_metadata, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(TokenMintInstructionAccounts {
            sender: sender.pubkey,
            backend_authority: backend_authority.pubkey,
            curve_account: curve_account.pubkey,
            mint: mint.pubkey,
            mint_metadata: mint_metadata.pubkey,
            curve_token_account: curve_token_account.pubkey,
            config_account: config_account.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            mpl_token_metadata: mpl_token_metadata.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
