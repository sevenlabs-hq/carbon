use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x90e47225a5f26f65")]
pub struct InstantUpdateTpsl {
    pub params: InstantUpdateTpslParams,
}

pub struct InstantUpdateTpslInstructionAccounts {
    pub keeper: solana_pubkey::Pubkey,
    pub api_keeper: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_request: solana_pubkey::Pubkey,
    pub custody: solana_pubkey::Pubkey,
    pub custody_doves_price_account: solana_pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InstantUpdateTpsl {
    type ArrangedAccounts = InstantUpdateTpslInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, api_keeper, owner, perpetuals, pool, position, position_request, custody, custody_doves_price_account, custody_pythnet_price_account, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InstantUpdateTpslInstructionAccounts {
            keeper: keeper.pubkey,
            api_keeper: api_keeper.pubkey,
            owner: owner.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            position_request: position_request.pubkey,
            custody: custody.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            custody_pythnet_price_account: custody_pythnet_price_account.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
