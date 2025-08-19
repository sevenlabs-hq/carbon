use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x414b3f4ceb5b5b88")]
pub struct Swap2 {
    pub params: Swap2Params,
}

pub struct Swap2InstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub funding_account: solana_pubkey::Pubkey,
    pub receiving_account: solana_pubkey::Pubkey,
    pub transfer_authority: solana_pubkey::Pubkey,
    pub perpetuals: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub receiving_custody: solana_pubkey::Pubkey,
    pub receiving_custody_doves_price_account: solana_pubkey::Pubkey,
    pub receiving_custody_pythnet_price_account: solana_pubkey::Pubkey,
    pub receiving_custody_token_account: solana_pubkey::Pubkey,
    pub dispensing_custody: solana_pubkey::Pubkey,
    pub dispensing_custody_doves_price_account: solana_pubkey::Pubkey,
    pub dispensing_custody_pythnet_price_account: solana_pubkey::Pubkey,
    pub dispensing_custody_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Swap2 {
    type ArrangedAccounts = Swap2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, funding_account, receiving_account, transfer_authority, perpetuals, pool, receiving_custody, receiving_custody_doves_price_account, receiving_custody_pythnet_price_account, receiving_custody_token_account, dispensing_custody, dispensing_custody_doves_price_account, dispensing_custody_pythnet_price_account, dispensing_custody_token_account, token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(Swap2InstructionAccounts {
            owner: owner.pubkey,
            funding_account: funding_account.pubkey,
            receiving_account: receiving_account.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            receiving_custody: receiving_custody.pubkey,
            receiving_custody_doves_price_account: receiving_custody_doves_price_account.pubkey,
            receiving_custody_pythnet_price_account: receiving_custody_pythnet_price_account.pubkey,
            receiving_custody_token_account: receiving_custody_token_account.pubkey,
            dispensing_custody: dispensing_custody.pubkey,
            dispensing_custody_doves_price_account: dispensing_custody_doves_price_account.pubkey,
            dispensing_custody_pythnet_price_account: dispensing_custody_pythnet_price_account
                .pubkey,
            dispensing_custody_token_account: dispensing_custody_token_account.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
