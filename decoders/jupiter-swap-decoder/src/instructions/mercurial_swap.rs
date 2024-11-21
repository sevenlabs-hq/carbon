use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x02054dadc500079d")]
pub struct MercurialSwap {}

pub struct MercurialSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub swap_state: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub pool_authority: solana_sdk::pubkey::Pubkey,
    pub user_transfer_authority: solana_sdk::pubkey::Pubkey,
    pub source_token_account: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MercurialSwap {
    type ArrangedAccounts = MercurialSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let swap_state = accounts.get(1)?;
        let token_program = accounts.get(2)?;
        let pool_authority = accounts.get(3)?;
        let user_transfer_authority = accounts.get(4)?;
        let source_token_account = accounts.get(5)?;
        let destination_token_account = accounts.get(6)?;

        Some(MercurialSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            swap_state: swap_state.pubkey,
            token_program: token_program.pubkey,
            pool_authority: pool_authority.pubkey,
            user_transfer_authority: user_transfer_authority.pubkey,
            source_token_account: source_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
        })
    }
}
