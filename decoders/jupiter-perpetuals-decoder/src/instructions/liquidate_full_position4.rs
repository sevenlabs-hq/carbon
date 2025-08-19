use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x40b05833a8bc9caf")]
pub struct LiquidateFullPosition4 {
    pub params: LiquidateFullPosition4Params,
}

pub struct LiquidateFullPosition4InstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub custody: solana_pubkey::Pubkey,
    pub custody_doves_price_account: solana_pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_pubkey::Pubkey,
    pub collateral_custody: solana_pubkey::Pubkey,
    pub collateral_custody_doves_price_account: solana_pubkey::Pubkey,
    pub collateral_custody_pythnet_price_account: solana_pubkey::Pubkey,
    pub collateral_custody_token_account: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LiquidateFullPosition4 {
    type ArrangedAccounts = LiquidateFullPosition4InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [signer, perpetuals, pool, position, custody, custody_doves_price_account, custody_pythnet_price_account, collateral_custody, collateral_custody_doves_price_account, collateral_custody_pythnet_price_account, collateral_custody_token_account, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LiquidateFullPosition4InstructionAccounts {
            signer: signer.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            custody: custody.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            custody_pythnet_price_account: custody_pythnet_price_account.pubkey,
            collateral_custody: collateral_custody.pubkey,
            collateral_custody_doves_price_account: collateral_custody_doves_price_account.pubkey,
            collateral_custody_pythnet_price_account: collateral_custody_pythnet_price_account
                .pubkey,
            collateral_custody_token_account: collateral_custody_token_account.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
