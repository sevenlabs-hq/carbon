use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x26f1156b783bb8f9")]
pub struct CykuraSwap {}

pub struct CykuraSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub signer: solana_sdk::pubkey::Pubkey,
    pub factory_state: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub input_token_account: solana_sdk::pubkey::Pubkey,
    pub output_token_account: solana_sdk::pubkey::Pubkey,
    pub input_vault: solana_sdk::pubkey::Pubkey,
    pub output_vault: solana_sdk::pubkey::Pubkey,
    pub last_observation_state: solana_sdk::pubkey::Pubkey,
    pub core_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CykuraSwap {
    type ArrangedAccounts = CykuraSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let signer = accounts.get(1)?;
        let factory_state = accounts.get(2)?;
        let pool_state = accounts.get(3)?;
        let input_token_account = accounts.get(4)?;
        let output_token_account = accounts.get(5)?;
        let input_vault = accounts.get(6)?;
        let output_vault = accounts.get(7)?;
        let last_observation_state = accounts.get(8)?;
        let core_program = accounts.get(9)?;
        let token_program = accounts.get(10)?;

        Some(CykuraSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            signer: signer.pubkey,
            factory_state: factory_state.pubkey,
            pool_state: pool_state.pubkey,
            input_token_account: input_token_account.pubkey,
            output_token_account: output_token_account.pubkey,
            input_vault: input_vault.pubkey,
            output_vault: output_vault.pubkey,
            last_observation_state: last_observation_state.pubkey,
            core_program: core_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
