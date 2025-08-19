use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x88f5e53a798d0ccf")]
pub struct InstantUpdateLimitOrder {
    pub params: InstantUpdateLimitOrderParams,
}

pub struct InstantUpdateLimitOrderInstructionAccounts {
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
}

impl carbon_core::deserialize::ArrangeAccounts for InstantUpdateLimitOrder {
    type ArrangedAccounts = InstantUpdateLimitOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, api_keeper, owner, perpetuals, pool, position, position_request, custody, custody_doves_price_account, custody_pythnet_price_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InstantUpdateLimitOrderInstructionAccounts {
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
        })
    }
}
