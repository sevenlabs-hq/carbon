use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf8c69e91e17587c8")]
pub struct Swap {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

#[derive(Debug, PartialEq)]
pub struct SwapInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub amm: solana_pubkey::Pubkey,
    pub user_transfer_authority: solana_pubkey::Pubkey,
    pub source_info: solana_pubkey::Pubkey,
    pub destination_info: solana_pubkey::Pubkey,
    pub swap_source: solana_pubkey::Pubkey,
    pub swap_destination: solana_pubkey::Pubkey,
    pub pool_mint: solana_pubkey::Pubkey,
    pub fee_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub oracle_main_account: solana_pubkey::Pubkey,
    pub oracle_sub_account: solana_pubkey::Pubkey,
    pub oracle_pc_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Swap {
    type ArrangedAccounts = SwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, amm, user_transfer_authority, source_info, destination_info, swap_source, swap_destination, pool_mint, fee_account, token_program, oracle_main_account, oracle_sub_account, oracle_pc_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SwapInstructionAccounts {
            authority: authority.pubkey,
            amm: amm.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            source_info: source_info.pubkey,
            destination_info: destination_info.pubkey,
            swap_source: swap_source.pubkey,
            swap_destination: swap_destination.pubkey,
            pool_mint: pool_mint.pubkey,
            fee_account: fee_account.pubkey,
            token_program: token_program.pubkey,
            oracle_main_account: oracle_main_account.pubkey,
            oracle_sub_account: oracle_sub_account.pubkey,
            oracle_pc_account: oracle_pc_account.pubkey,
        })
    }
}
