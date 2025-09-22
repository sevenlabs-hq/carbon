use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc360ed6c44a2dbe6")]
pub struct TwoHopSwap {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub amount_specified_is_input: bool,
    pub a_to_b_one: bool,
    pub a_to_b_two: bool,
    pub sqrt_price_limit_one: u128,
    pub sqrt_price_limit_two: u128,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TwoHopSwapInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub token_authority: solana_pubkey::Pubkey,
    pub whirlpool_one: solana_pubkey::Pubkey,
    pub whirlpool_two: solana_pubkey::Pubkey,
    pub token_owner_account_one_a: solana_pubkey::Pubkey,
    pub token_vault_one_a: solana_pubkey::Pubkey,
    pub token_owner_account_one_b: solana_pubkey::Pubkey,
    pub token_vault_one_b: solana_pubkey::Pubkey,
    pub token_owner_account_two_a: solana_pubkey::Pubkey,
    pub token_vault_two_a: solana_pubkey::Pubkey,
    pub token_owner_account_two_b: solana_pubkey::Pubkey,
    pub token_vault_two_b: solana_pubkey::Pubkey,
    pub tick_array_one0: solana_pubkey::Pubkey,
    pub tick_array_one1: solana_pubkey::Pubkey,
    pub tick_array_one2: solana_pubkey::Pubkey,
    pub tick_array_two0: solana_pubkey::Pubkey,
    pub tick_array_two1: solana_pubkey::Pubkey,
    pub tick_array_two2: solana_pubkey::Pubkey,
    pub oracle_one: solana_pubkey::Pubkey,
    pub oracle_two: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TwoHopSwap {
    type ArrangedAccounts = TwoHopSwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let token_program = next_account(&mut iter)?;
        let token_authority = next_account(&mut iter)?;
        let whirlpool_one = next_account(&mut iter)?;
        let whirlpool_two = next_account(&mut iter)?;
        let token_owner_account_one_a = next_account(&mut iter)?;
        let token_vault_one_a = next_account(&mut iter)?;
        let token_owner_account_one_b = next_account(&mut iter)?;
        let token_vault_one_b = next_account(&mut iter)?;
        let token_owner_account_two_a = next_account(&mut iter)?;
        let token_vault_two_a = next_account(&mut iter)?;
        let token_owner_account_two_b = next_account(&mut iter)?;
        let token_vault_two_b = next_account(&mut iter)?;
        let tick_array_one0 = next_account(&mut iter)?;
        let tick_array_one1 = next_account(&mut iter)?;
        let tick_array_one2 = next_account(&mut iter)?;
        let tick_array_two0 = next_account(&mut iter)?;
        let tick_array_two1 = next_account(&mut iter)?;
        let tick_array_two2 = next_account(&mut iter)?;
        let oracle_one = next_account(&mut iter)?;
        let oracle_two = next_account(&mut iter)?;

        Some(TwoHopSwapInstructionAccounts {
            token_program,
            token_authority,
            whirlpool_one,
            whirlpool_two,
            token_owner_account_one_a,
            token_vault_one_a,
            token_owner_account_one_b,
            token_vault_one_b,
            token_owner_account_two_a,
            token_vault_two_a,
            token_owner_account_two_b,
            token_vault_two_b,
            tick_array_one0,
            tick_array_one1,
            tick_array_one2,
            tick_array_two0,
            tick_array_two1,
            tick_array_two2,
            oracle_one,
            oracle_two,
        })
    }
}
