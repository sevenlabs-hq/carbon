use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5f87c0c4f281e644")]
pub struct InitializeReward {
    pub param: InitializeRewardParam,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeRewardInstructionAccounts {
    pub reward_funder: solana_pubkey::Pubkey,
    pub funder_token_account: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub operation_state: solana_pubkey::Pubkey,
    pub permissionless_farm_switch: solana_pubkey::Pubkey,
    pub reward_token_mint: solana_pubkey::Pubkey,
    pub reward_token_vault: solana_pubkey::Pubkey,
    pub reward_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeReward {
    type ArrangedAccounts = InitializeRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [reward_funder, funder_token_account, amm_config, pool_state, operation_state, permissionless_farm_switch, reward_token_mint, reward_token_vault, reward_token_program, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeRewardInstructionAccounts {
            reward_funder: reward_funder.pubkey,
            funder_token_account: funder_token_account.pubkey,
            amm_config: amm_config.pubkey,
            pool_state: pool_state.pubkey,
            operation_state: operation_state.pubkey,
            permissionless_farm_switch: permissionless_farm_switch.pubkey,
            reward_token_mint: reward_token_mint.pubkey,
            reward_token_vault: reward_token_vault.pubkey,
            reward_token_program: reward_token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
