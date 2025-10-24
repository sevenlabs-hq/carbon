use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x11")]
pub struct SwapBaseOut2 {
    pub max_amount_in: u64,
    pub amount_out: u64,
}

#[derive(Debug)]
pub struct SwapBaseOut2InstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub amm: solana_pubkey::Pubkey,
    pub amm_authority: solana_pubkey::Pubkey,
    pub pool_coin_token_account: solana_pubkey::Pubkey,
    pub pool_pc_token_account: solana_pubkey::Pubkey,
    pub user_source_token_account: solana_pubkey::Pubkey,
    pub user_destination_token_account: solana_pubkey::Pubkey,
    pub user_source_owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapBaseOut2 {
    type ArrangedAccounts = SwapBaseOut2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, amm, amm_authority, pool_coin_token_account, pool_pc_token_account, user_source_token_account, user_destination_token_account, user_source_owner, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SwapBaseOut2InstructionAccounts {
            token_program: token_program.pubkey,
            amm: amm.pubkey,
            amm_authority: amm_authority.pubkey,
            pool_coin_token_account: pool_coin_token_account.pubkey,
            pool_pc_token_account: pool_pc_token_account.pubkey,
            user_source_token_account: user_source_token_account.pubkey,
            user_destination_token_account: user_destination_token_account.pubkey,
            user_source_owner: user_source_owner.pubkey,
        })
    }
}
