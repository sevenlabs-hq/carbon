use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x05b231ce8ce78391")]
pub struct QuoteSell {
    pub params: SwapParams,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct QuoteSellInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub mint_a: solana_pubkey::Pubkey,
    pub mint_b: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for QuoteSell {
    type ArrangedAccounts = QuoteSellInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, owner, user, mint_a, mint_b, program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(QuoteSellInstructionAccounts {
            pool: pool.pubkey,
            owner: owner.pubkey,
            user: user.pubkey,
            mint_a: mint_a.pubkey,
            mint_b: mint_b.pubkey,
            program: program.pubkey,
        })
    }
}
