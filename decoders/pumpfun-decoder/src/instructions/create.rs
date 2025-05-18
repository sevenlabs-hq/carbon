use alloc::string::String;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x181ec828051c0777")]
pub struct Create {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creator: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub associated_bonding_curve: solana_pubkey::Pubkey,
    pub global: solana_pubkey::Pubkey,
    pub mpl_token_metadata: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Create {
    type ArrangedAccounts = CreateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, mint_authority, bonding_curve, associated_bonding_curve, global, mpl_token_metadata, metadata, user, system_program, token_program, associated_token_program, rent, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateInstructionAccounts {
            mint: mint.pubkey,
            mint_authority: mint_authority.pubkey,
            bonding_curve: bonding_curve.pubkey,
            associated_bonding_curve: associated_bonding_curve.pubkey,
            global: global.pubkey,
            mpl_token_metadata: mpl_token_metadata.pubkey,
            metadata: metadata.pubkey,
            user: user.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            rent: rent.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
