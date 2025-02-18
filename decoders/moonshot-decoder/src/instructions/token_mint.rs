use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x032ca4b87b0df5b3")]
pub struct TokenMint {
    pub mint_params: TokenMintParams,
}

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
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let sender = accounts.get(0)?;
        let backend_authority = accounts.get(1)?;
        let curve_account = accounts.get(2)?;
        let mint = accounts.get(3)?;
        let mint_metadata = accounts.get(4)?;
        let curve_token_account = accounts.get(5)?;
        let config_account = accounts.get(6)?;
        let token_program = accounts.get(7)?;
        let associated_token_program = accounts.get(8)?;
        let mpl_token_metadata = accounts.get(9)?;
        let system_program = accounts.get(10)?;

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
