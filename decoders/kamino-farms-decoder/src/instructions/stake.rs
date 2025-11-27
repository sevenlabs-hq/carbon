use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xceb0ca12c8d1b36c")]
pub struct Stake {
    pub amount: u64,
}

pub struct StakeInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub user_state: solana_pubkey::Pubkey,
    pub farm_state: solana_pubkey::Pubkey,
    pub farm_vault: solana_pubkey::Pubkey,
    pub user_ata: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub scope_prices: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Stake {
    type ArrangedAccounts = StakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            owner,
            user_state,
            farm_state,
            farm_vault,
            user_ata,
            token_mint,
            scope_prices,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(StakeInstructionAccounts {
            owner: owner.pubkey,
            user_state: user_state.pubkey,
            farm_state: farm_state.pubkey,
            farm_vault: farm_vault.pubkey,
            user_ata: user_ata.pubkey,
            token_mint: token_mint.pubkey,
            scope_prices: scope_prices.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
