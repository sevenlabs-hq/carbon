use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2869d9bcdc2d6d6e")]
pub struct ClosePositionRequest {
    pub params: ClosePositionRequestParams,
}

pub struct ClosePositionRequestInstructionAccounts {
    pub keeper: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub owner_ata: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub position_request: solana_pubkey::Pubkey,
    pub position_request_ata: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClosePositionRequest {
    type ArrangedAccounts = ClosePositionRequestInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, owner, owner_ata, pool, position_request, position_request_ata, position, token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ClosePositionRequestInstructionAccounts {
            keeper: keeper.pubkey,
            owner: owner.pubkey,
            owner_ata: owner_ata.pubkey,
            pool: pool.pubkey,
            position_request: position_request.pubkey,
            position_request_ata: position_request_ata.pubkey,
            position: position.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
