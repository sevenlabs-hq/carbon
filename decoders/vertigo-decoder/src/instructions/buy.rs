use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x66063d1201daebea")]
pub struct Buy {
    pub params: SwapParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BuyInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub mint_a: solana_pubkey::Pubkey,
    pub mint_b: solana_pubkey::Pubkey,
    pub user_ta_a: solana_pubkey::Pubkey,
    pub user_ta_b: solana_pubkey::Pubkey,
    pub vault_a: solana_pubkey::Pubkey,
    pub vault_b: solana_pubkey::Pubkey,
    pub token_program_a: solana_pubkey::Pubkey,
    pub token_program_b: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Buy {
    type ArrangedAccounts = BuyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, user, owner, mint_a, mint_b, user_ta_a, user_ta_b, vault_a, vault_b, token_program_a, token_program_b, system_program, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(BuyInstructionAccounts {
            pool: pool.pubkey,
            user: user.pubkey,
            owner: owner.pubkey,
            mint_a: mint_a.pubkey,
            mint_b: mint_b.pubkey,
            user_ta_a: user_ta_a.pubkey,
            user_ta_b: user_ta_b.pubkey,
            vault_a: vault_a.pubkey,
            vault_b: vault_b.pubkey,
            token_program_a: token_program_a.pubkey,
            token_program_b: token_program_b.pubkey,
            system_program: system_program.pubkey,
            program: program.pubkey,
        })
    }
}
