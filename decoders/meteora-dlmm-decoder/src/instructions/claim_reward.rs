use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x955fb5f25e5a9ea2")]
pub struct ClaimReward {
    pub reward_index: u64,
}

pub struct ClaimRewardInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub bin_array_lower: solana_sdk::pubkey::Pubkey,
    pub bin_array_upper: solana_sdk::pubkey::Pubkey,
    pub sender: solana_sdk::pubkey::Pubkey,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
    pub user_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimReward {
    type ArrangedAccounts = ClaimRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let lb_pair = accounts.get(0)?;
        let position = accounts.get(1)?;
        let bin_array_lower = accounts.get(2)?;
        let bin_array_upper = accounts.get(3)?;
        let sender = accounts.get(4)?;
        let reward_vault = accounts.get(5)?;
        let reward_mint = accounts.get(6)?;
        let user_token_account = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let event_authority = accounts.get(9)?;
        let program = accounts.get(10)?;

        Some(ClaimRewardInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            position: position.pubkey,
            bin_array_lower: bin_array_lower.pubkey,
            bin_array_upper: bin_array_upper.pubkey,
            sender: sender.pubkey,
            reward_vault: reward_vault.pubkey,
            reward_mint: reward_mint.pubkey,
            user_token_account: user_token_account.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
