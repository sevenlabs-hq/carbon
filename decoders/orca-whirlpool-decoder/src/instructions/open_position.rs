use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x87802f4d0f98f031")]
pub struct OpenPosition {
    pub bumps: OpenPositionBumps,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

pub struct OpenPositionInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_mint: solana_pubkey::Pubkey,
    pub position_token_account: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OpenPosition {
    type ArrangedAccounts = OpenPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [funder, owner, position, position_mint, position_token_account, whirlpool, token_program, system_program, rent, associated_token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(OpenPositionInstructionAccounts {
            funder: funder.pubkey,
            owner: owner.pubkey,
            position: position.pubkey,
            position_mint: position_mint.pubkey,
            position_token_account: position_token_account.pubkey,
            whirlpool: whirlpool.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            associated_token_program: associated_token_program.pubkey,
        })
    }
}
