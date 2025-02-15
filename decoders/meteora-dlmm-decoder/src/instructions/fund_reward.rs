use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbc32f9a55d97263f")]
pub struct FundReward {
    pub reward_index: u64,
    pub amount: u64,
    pub carry_forward: bool,
}

pub struct FundRewardInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
    pub funder_token_account: solana_sdk::pubkey::Pubkey,
    pub funder: solana_sdk::pubkey::Pubkey,
    pub bin_array: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FundReward {
    type ArrangedAccounts = FundRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let lb_pair = accounts.get(0)?;
        let reward_vault = accounts.get(1)?;
        let reward_mint = accounts.get(2)?;
        let funder_token_account = accounts.get(3)?;
        let funder = accounts.get(4)?;
        let bin_array = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let event_authority = accounts.get(7)?;
        let program = accounts.get(8)?;

        Some(FundRewardInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            reward_vault: reward_vault.pubkey,
            reward_mint: reward_mint.pubkey,
            funder_token_account: funder_token_account.pubkey,
            funder: funder.pubkey,
            bin_array: bin_array.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
