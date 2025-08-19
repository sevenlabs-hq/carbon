use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe4a24e1c46db7473")]
pub struct AddLiquidity2 {
    pub params: AddLiquidity2Params,
}

pub struct AddLiquidity2InstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub funding_account: solana_pubkey::Pubkey,
    pub lp_token_account: solana_pubkey::Pubkey,
    pub transfer_authority: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub custody: solana_pubkey::Pubkey,
    pub custody_doves_price_account: solana_pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_pubkey::Pubkey,
    pub custody_token_account: solana_pubkey::Pubkey,
    pub lp_token_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddLiquidity2 {
    type ArrangedAccounts = AddLiquidity2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, funding_account, lp_token_account, transfer_authority, perpetuals, pool, custody, custody_doves_price_account, custody_pythnet_price_account, custody_token_account, lp_token_mint, token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AddLiquidity2InstructionAccounts {
            owner: owner.pubkey,
            funding_account: funding_account.pubkey,
            lp_token_account: lp_token_account.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            custody: custody.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            custody_pythnet_price_account: custody_pythnet_price_account.pubkey,
            custody_token_account: custody_token_account.pubkey,
            lp_token_mint: lp_token_mint.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
