use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x30")]
pub struct BondingCurveInitialize {
    pub start_price: u128,
    pub end_price: u128,
    pub control_points: [u16; 4],
    pub creator: solana_pubkey::Pubkey,
    pub graduation_methods: [GraduationMethod; 8],
    pub swap_fee_bps: u16,
    pub quote_fee_bps: u16,
    pub base_fee_bps: u16,
    pub launch_slot: u64,
    pub creator_reward: u64,
    pub graduation_reward: u64,
    pub graduation_target: u64,
    pub graduation_slot: u64,
    pub min_reserve_bps: u16,
    pub buy_requires_permission: bool,
    pub buy_permission_bitmap: [u8; 32],
    pub sell_requires_permission: bool,
    pub sell_permission_bitmap: [u8; 32],
    pub max_buy_amount: u64,
    pub max_sell_amount: u64,
    pub retain_mint_authority: bool,
    pub base_allocation_bps: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BondingCurveInitializeInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub authority_config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub ata_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BondingCurveInitialize {
    type ArrangedAccounts = BondingCurveInitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let authority_config = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let ata_program = next_account(&mut iter)?;

        Some(BondingCurveInitializeInstructionAccounts {
            authority,
            bonding_curve,
            base_mint,
            quote_mint,
            quote_vault,
            authority_config,
            system_program,
            base_token_program,
            quote_token_program,
            ata_program,
        })
    }
}
