

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x5f87c0c4f281e644")]
pub struct InitializeReward{
}

pub struct InitializeRewardInstructionAccounts {
    pub farm_admin: solana_sdk::pubkey::Pubkey,
    pub farm_state: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
    pub reward_treasury_vault: solana_sdk::pubkey::Pubkey,
    pub farm_vaults_authority: solana_sdk::pubkey::Pubkey,
    pub treasury_vaults_authority: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeReward {
    type ArrangedAccounts = InitializeRewardInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            farm_admin,
            farm_state,
            global_config,
            reward_mint,
            reward_vault,
            reward_treasury_vault,
            farm_vaults_authority,
            treasury_vaults_authority,
            token_program,
            system_program,
            rent,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(InitializeRewardInstructionAccounts {
            farm_admin: farm_admin.pubkey,
            farm_state: farm_state.pubkey,
            global_config: global_config.pubkey,
            reward_mint: reward_mint.pubkey,
            reward_vault: reward_vault.pubkey,
            reward_treasury_vault: reward_treasury_vault.pubkey,
            farm_vaults_authority: farm_vaults_authority.pubkey,
            treasury_vaults_authority: treasury_vaults_authority.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}