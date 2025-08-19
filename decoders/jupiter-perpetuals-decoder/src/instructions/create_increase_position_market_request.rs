use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb855c71869ab9c38")]
pub struct CreateIncreasePositionMarketRequest {
    pub params: CreateIncreasePositionMarketRequestParams,
}

pub struct CreateIncreasePositionMarketRequestInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub funding_account: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_request: solana_pubkey::Pubkey,
    pub position_request_ata: solana_pubkey::Pubkey,
    pub custody: solana_pubkey::Pubkey,
    pub collateral_custody: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub referral: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateIncreasePositionMarketRequest {
    type ArrangedAccounts = CreateIncreasePositionMarketRequestInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, funding_account, perpetuals, pool, position, position_request, position_request_ata, custody, collateral_custody, input_mint, referral, token_program, associated_token_program, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateIncreasePositionMarketRequestInstructionAccounts {
            owner: owner.pubkey,
            funding_account: funding_account.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            position_request: position_request.pubkey,
            position_request_ata: position_request_ata.pubkey,
            custody: custody.pubkey,
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
