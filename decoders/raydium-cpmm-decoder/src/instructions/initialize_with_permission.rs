use super::super::types::*;

use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3f37fe4131b25979")]
pub struct InitializeWithPermission {
    pub init_amount_0: u64,
    pub init_amount_1: u64,
    pub open_time: u64,
    pub creator_fee_on: CreatorFeeOn,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeWithPermissionInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub token_0_mint: solana_pubkey::Pubkey,
    pub token_1_mint: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub payer_token_0: solana_pubkey::Pubkey,
    pub payer_token_1: solana_pubkey::Pubkey,
    pub payer_lp_token: solana_pubkey::Pubkey,
    pub token_0_vault: solana_pubkey::Pubkey,
    pub token_1_vault: solana_pubkey::Pubkey,
    pub create_pool_fee: solana_pubkey::Pubkey,
    pub observation_state: solana_pubkey::Pubkey,
    pub permission: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_0_program: solana_pubkey::Pubkey,
    pub token_1_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeWithPermission {
    type ArrangedAccounts = InitializeWithPermissionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let creator = next_account(&mut iter)?;
        let amm_config = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let token_0_mint = next_account(&mut iter)?;
        let token_1_mint = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;
        let payer_token_0 = next_account(&mut iter)?;
        let payer_token_1 = next_account(&mut iter)?;
        let payer_lp_token = next_account(&mut iter)?;
        let token_0_vault = next_account(&mut iter)?;
        let token_1_vault = next_account(&mut iter)?;
        let create_pool_fee = next_account(&mut iter)?;
        let observation_state = next_account(&mut iter)?;
        let permission = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let token_0_program = next_account(&mut iter)?;
        let token_1_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(InitializeWithPermissionInstructionAccounts {
            payer,
            creator,
            amm_config,
            authority,
            pool_state,
            token_0_mint,
            token_1_mint,
            lp_mint,
            payer_token_0,
            payer_token_1,
            payer_lp_token,
            token_0_vault,
            token_1_vault,
            create_pool_fee,
            observation_state,
            permission,
            token_program,
            token_0_program,
            token_1_program,
            associated_token_program,
            system_program,
        })
    }
}
