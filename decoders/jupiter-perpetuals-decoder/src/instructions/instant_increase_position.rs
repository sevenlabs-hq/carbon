use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa47e44b6dfa640b7")]
pub struct InstantIncreasePosition {
    pub params: InstantIncreasePositionParams,
}

pub struct InstantIncreasePositionInstructionAccounts {
    pub keeper: solana_pubkey::Pubkey,
    pub api_keeper: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub funding_account: solana_pubkey::Pubkey,
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
    pub token_ledger: solana_pubkey::Pubkey,
    pub referral: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InstantIncreasePosition {
    type ArrangedAccounts = InstantIncreasePositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, api_keeper, owner, funding_account, perpetuals, pool, position, custody, custody_doves_price_account, custody_pythnet_price_account, collateral_custody, collateral_custody_doves_price_account, collateral_custody_pythnet_price_account, collateral_custody_token_account, token_ledger, referral, token_program, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InstantIncreasePositionInstructionAccounts {
            keeper: keeper.pubkey,
            api_keeper: api_keeper.pubkey,
            owner: owner.pubkey,
            funding_account: funding_account.pubkey,
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
            token_ledger: token_ledger.pubkey,
            referral: referral.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
