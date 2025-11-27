use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0536d5704be87525")]
pub struct UpdateReserveAllocation {
    pub weight: u64,
    pub cap: u64,
}

pub struct UpdateReserveAllocationInstructionAccounts {
    pub admin_authority: solana_pubkey::Pubkey,
    pub vault_state: solana_pubkey::Pubkey,
    pub base_vault_authority: solana_pubkey::Pubkey,
    pub reserve_collateral_mint: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub ctoken_vault: solana_pubkey::Pubkey,
    pub reserve_collateral_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateReserveAllocation {
    type ArrangedAccounts = UpdateReserveAllocationInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            admin_authority,
            vault_state,
            base_vault_authority,
            reserve_collateral_mint,
            reserve,
            ctoken_vault,
            reserve_collateral_token_program,
            system_program,
            rent,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(UpdateReserveAllocationInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            vault_state: vault_state.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            reserve_collateral_mint: reserve_collateral_mint.pubkey,
            reserve: reserve.pubkey,
            ctoken_vault: ctoken_vault.pubkey,
            reserve_collateral_token_program: reserve_collateral_token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
