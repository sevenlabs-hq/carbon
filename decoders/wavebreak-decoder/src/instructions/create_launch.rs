use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x29")]
pub struct CreateLaunch {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub start_price: Option<u128>,
    pub end_price: Option<u128>,
    pub control_points: Option<[u16; 4]>,
    pub graduation_target: Option<u64>,
    pub graduation_methods: Option<[GraduationMethod; 8]>,
    pub launch_slot: Option<u64>,
    pub graduation_slot: Option<u64>,
    pub base_allocation_bps: Option<u16>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateLaunchInstructionAccounts {
    pub creator: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub mint_config: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub ata_program: solana_pubkey::Pubkey,
    pub metaplex_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateLaunch {
    type ArrangedAccounts = CreateLaunchInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let creator = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let mint_config = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let ata_program = next_account(&mut iter)?;
        let metaplex_program = next_account(&mut iter)?;

        Some(CreateLaunchInstructionAccounts {
            creator,
            bonding_curve,
            base_mint,
            quote_mint,
            quote_vault,
            mint_config,
            metadata,
            system_program,
            base_token_program,
            quote_token_program,
            ata_program,
            metaplex_program,
        })
    }
}
