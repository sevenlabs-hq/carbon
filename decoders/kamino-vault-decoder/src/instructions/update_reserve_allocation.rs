

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0536d5704be87525")]
pub struct UpdateReserveAllocation{
    pub weight: u64,
    pub cap: u64,
}

pub struct UpdateReserveAllocationInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub vault_state: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub reserve_collateral_mint: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub ctoken_vault: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateReserveAllocation {
    type ArrangedAccounts = UpdateReserveAllocationInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let admin_authority = accounts.get(0)?;
        let vault_state = accounts.get(1)?;
        let base_vault_authority = accounts.get(2)?;
        let reserve_collateral_mint = accounts.get(3)?;
        let reserve = accounts.get(4)?;
        let ctoken_vault = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let rent = accounts.get(7)?;
        let token_program = accounts.get(8)?;

        Some(UpdateReserveAllocationInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            vault_state: vault_state.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            reserve_collateral_mint: reserve_collateral_mint.pubkey,
            reserve: reserve.pubkey,
            ctoken_vault: ctoken_vault.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            token_program: token_program.pubkey,
        })
    }
}