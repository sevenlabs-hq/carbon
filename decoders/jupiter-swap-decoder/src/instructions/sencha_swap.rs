use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x19320715cff8e6c2")]
pub struct SenchaSwap {}

pub struct SenchaSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub swap: solana_sdk::pubkey::Pubkey,
    pub user_authority: solana_sdk::pubkey::Pubkey,
    pub input_user_account: solana_sdk::pubkey::Pubkey,
    pub input_token_account: solana_sdk::pubkey::Pubkey,
    pub input_fees_account: solana_sdk::pubkey::Pubkey,
    pub output_user_account: solana_sdk::pubkey::Pubkey,
    pub output_token_account: solana_sdk::pubkey::Pubkey,
    pub output_fees_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SenchaSwap {
    type ArrangedAccounts = SenchaSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let token_program = accounts.get(1)?;
        let swap = accounts.get(2)?;
        let user_authority = accounts.get(3)?;
        let input_user_account = accounts.get(4)?;
        let input_token_account = accounts.get(5)?;
        let input_fees_account = accounts.get(6)?;
        let output_user_account = accounts.get(7)?;
        let output_token_account = accounts.get(8)?;
        let output_fees_account = accounts.get(9)?;

        Some(SenchaSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            token_program: token_program.pubkey,
            swap: swap.pubkey,
            user_authority: user_authority.pubkey,
            input_user_account: input_user_account.pubkey,
            input_token_account: input_token_account.pubkey,
            input_fees_account: input_fees_account.pubkey,
            output_user_account: output_user_account.pubkey,
            output_token_account: output_token_account.pubkey,
            output_fees_account: output_fees_account.pubkey,
        })
    }
}
