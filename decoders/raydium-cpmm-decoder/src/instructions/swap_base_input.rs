use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8fbe5adac41e33de")]
pub struct SwapBaseInput {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

pub struct SwapBaseInputInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub pool_state: solana_sdk::pubkey::Pubkey,
    pub input_token_account: solana_sdk::pubkey::Pubkey,
    pub output_token_account: solana_sdk::pubkey::Pubkey,
    pub input_vault: solana_sdk::pubkey::Pubkey,
    pub output_vault: solana_sdk::pubkey::Pubkey,
    pub input_token_program: solana_sdk::pubkey::Pubkey,
    pub output_token_program: solana_sdk::pubkey::Pubkey,
    pub input_token_mint: solana_sdk::pubkey::Pubkey,
    pub output_token_mint: solana_sdk::pubkey::Pubkey,
    pub observation_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapBaseInput {
    type ArrangedAccounts = SwapBaseInputInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, authority, amm_config, pool_state, input_token_account, output_token_account, input_vault, output_vault, input_token_program, output_token_program, input_token_mint, output_token_mint, observation_state, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SwapBaseInputInstructionAccounts {
            payer: payer.pubkey,
            authority: authority.pubkey,
            amm_config: amm_config.pubkey,
            pool_state: pool_state.pubkey,
            input_token_account: input_token_account.pubkey,
            output_token_account: output_token_account.pubkey,
            input_vault: input_vault.pubkey,
            output_vault: output_vault.pubkey,
            input_token_program: input_token_program.pubkey,
            output_token_program: output_token_program.pubkey,
            input_token_mint: input_token_mint.pubkey,
            output_token_mint: output_token_mint.pubkey,
            observation_state: observation_state.pubkey,
        })
    }
}
