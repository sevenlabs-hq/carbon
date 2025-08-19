use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4ac6c356c163014f")]
pub struct CreateDecreasePositionMarketRequest {
    pub params: CreateDecreasePositionMarketRequestParams,
}

pub struct CreateDecreasePositionMarketRequestInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub receiving_account: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_request: solana_pubkey::Pubkey,
    pub position_request_ata: solana_pubkey::Pubkey,
    pub custody: solana_pubkey::Pubkey,
    pub collateral_custody: solana_pubkey::Pubkey,
    pub desired_mint: solana_pubkey::Pubkey,
    pub referral: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateDecreasePositionMarketRequest {
    type ArrangedAccounts = CreateDecreasePositionMarketRequestInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, receiving_account, perpetuals, pool, position, position_request, position_request_ata, custody, collateral_custody, desired_mint, referral, token_program, associated_token_program, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateDecreasePositionMarketRequestInstructionAccounts {
            owner: owner.pubkey,
            receiving_account: receiving_account.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            position_request: position_request.pubkey,
            position_request_ata: position_request_ata.pubkey,
            custody: custody.pubkey,
            collateral_custody: collateral_custody.pubkey,
            desired_mint: desired_mint.pubkey,
            referral: referral.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
