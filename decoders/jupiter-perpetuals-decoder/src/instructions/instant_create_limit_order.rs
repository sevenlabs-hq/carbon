use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc225c37b287f7e9c")]
pub struct InstantCreateLimitOrder {
    pub params: InstantCreateLimitOrderParams,
}

pub struct InstantCreateLimitOrderInstructionAccounts {
    pub keeper: solana_pubkey::Pubkey,
    pub api_keeper: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub funding_account: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_request: solana_pubkey::Pubkey,
    pub position_request_ata: solana_pubkey::Pubkey,
    pub custody: solana_pubkey::Pubkey,
    pub custody_doves_price_account: solana_pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_pubkey::Pubkey,
    pub collateral_custody: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub referral: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InstantCreateLimitOrder {
    type ArrangedAccounts = InstantCreateLimitOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, api_keeper, owner, funding_account, perpetuals, pool, position, position_request, position_request_ata, custody, custody_doves_price_account, custody_pythnet_price_account, collateral_custody, input_mint, referral, token_program, associated_token_program, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InstantCreateLimitOrderInstructionAccounts {
            keeper: keeper.pubkey,
            api_keeper: api_keeper.pubkey,
            owner: owner.pubkey,
            funding_account: funding_account.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            position_request: position_request.pubkey,
            position_request_ata: position_request_ata.pubkey,
            custody: custody.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            custody_pythnet_price_account: custody_pythnet_price_account.pubkey,
            collateral_custody: collateral_custody.pubkey,
            input_mint: input_mint.pubkey,
            referral: referral.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
