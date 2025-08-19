use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x72376a8cc7dd2070")]
pub struct IncreasePositionWithInternalSwap {
    pub params: IncreasePositionWithInternalSwapParams,
}

pub struct IncreasePositionWithInternalSwapInstructionAccounts {
    pub keeper: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub position_request: solana_pubkey::Pubkey,
    pub position_request_ata: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub custody: solana_pubkey::Pubkey,
    pub custody_doves_price_account: solana_pubkey::Pubkey,
    pub custody_pythnet_price_account: solana_pubkey::Pubkey,
    pub collateral_custody: solana_pubkey::Pubkey,
    pub collateral_custody_doves_price_account: solana_pubkey::Pubkey,
    pub collateral_custody_pythnet_price_account: solana_pubkey::Pubkey,
    pub collateral_custody_token_account: solana_pubkey::Pubkey,
    pub receiving_custody: solana_pubkey::Pubkey,
    pub receiving_custody_doves_price_account: solana_pubkey::Pubkey,
    pub receiving_custody_pythnet_price_account: solana_pubkey::Pubkey,
    pub receiving_custody_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IncreasePositionWithInternalSwap {
    type ArrangedAccounts = IncreasePositionWithInternalSwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, perpetuals, pool, position_request, position_request_ata, position, custody, custody_doves_price_account, custody_pythnet_price_account, collateral_custody, collateral_custody_doves_price_account, collateral_custody_pythnet_price_account, collateral_custody_token_account, receiving_custody, receiving_custody_doves_price_account, receiving_custody_pythnet_price_account, receiving_custody_token_account, token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(IncreasePositionWithInternalSwapInstructionAccounts {
            keeper: keeper.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            position_request: position_request.pubkey,
            position_request_ata: position_request_ata.pubkey,
            position: position.pubkey,
            custody: custody.pubkey,
            custody_doves_price_account: custody_doves_price_account.pubkey,
            custody_pythnet_price_account: custody_pythnet_price_account.pubkey,
            collateral_custody: collateral_custody.pubkey,
            collateral_custody_doves_price_account: collateral_custody_doves_price_account.pubkey,
            collateral_custody_pythnet_price_account: collateral_custody_pythnet_price_account
                .pubkey,
            collateral_custody_token_account: collateral_custody_token_account.pubkey,
            receiving_custody: receiving_custody.pubkey,
            receiving_custody_doves_price_account: receiving_custody_doves_price_account.pubkey,
            receiving_custody_pythnet_price_account: receiving_custody_pythnet_price_account.pubkey,
            receiving_custody_token_account: receiving_custody_token_account.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
