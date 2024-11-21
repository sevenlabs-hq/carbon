

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xda063ee90121e852")]
pub struct InitFarmsForReserve{
    pub mode: u8,
}

pub struct InitFarmsForReserveInstructionAccounts {
    pub lending_market_owner: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub farms_program: solana_sdk::pubkey::Pubkey,
    pub farms_global_config: solana_sdk::pubkey::Pubkey,
    pub farm_state: solana_sdk::pubkey::Pubkey,
    pub farms_vault_authority: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitFarmsForReserve {
    type ArrangedAccounts = InitFarmsForReserveInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let lending_market_owner = accounts.get(0)?;
        let lending_market = accounts.get(1)?;
        let lending_market_authority = accounts.get(2)?;
        let reserve = accounts.get(3)?;
        let farms_program = accounts.get(4)?;
        let farms_global_config = accounts.get(5)?;
        let farm_state = accounts.get(6)?;
        let farms_vault_authority = accounts.get(7)?;
        let rent = accounts.get(8)?;
        let system_program = accounts.get(9)?;

        Some(InitFarmsForReserveInstructionAccounts {
            lending_market_owner: lending_market_owner.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            reserve: reserve.pubkey,
            farms_program: farms_program.pubkey,
            farms_global_config: farms_global_config.pubkey,
            farm_state: farm_state.pubkey,
            farms_vault_authority: farms_vault_authority.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}