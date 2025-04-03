use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x01")]
pub struct Swap {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

#[derive(Debug, PartialEq)]
pub struct SwapInstructionAccounts {
    pub swap: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub user_transfer_authority: solana_pubkey::Pubkey,
    pub source: solana_pubkey::Pubkey,
    pub swap_source: solana_pubkey::Pubkey,
    pub swap_destination: solana_pubkey::Pubkey,
    pub destination: solana_pubkey::Pubkey,
    pub pool_mint: solana_pubkey::Pubkey,
    pub pool_fee: solana_pubkey::Pubkey,
    pub source_mint: solana_pubkey::Pubkey,
    pub destination_mint: solana_pubkey::Pubkey,
    pub source_token_program: solana_pubkey::Pubkey,
    pub destination_token_program: solana_pubkey::Pubkey,
    pub pool_token_program: solana_pubkey::Pubkey,
    pub swap_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Swap {
    type ArrangedAccounts = SwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [swap, authority, user_transfer_authority, source, swap_source, swap_destination, destination, pool_mint, pool_fee, source_mint, destination_mint, source_token_program, destination_token_program, pool_token_program, swap_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SwapInstructionAccounts {
            swap: swap.pubkey,
            authority: authority.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            source: source.pubkey,
            swap_source: swap_source.pubkey,
            swap_destination: swap_destination.pubkey,
            destination: destination.pubkey,
            pool_mint: pool_mint.pubkey,
            pool_fee: pool_fee.pubkey,
            source_mint: source_mint.pubkey,
            destination_mint: destination_mint.pubkey,
            source_token_program: source_token_program.pubkey,
            destination_token_program: destination_token_program.pubkey,
            pool_token_program: pool_token_program.pubkey,
            swap_program: swap_program.pubkey,
        })
    }
}
