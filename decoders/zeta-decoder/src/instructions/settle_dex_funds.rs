use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa5678e26d3a60ee2")]
pub struct SettleDexFunds {}

pub struct SettleDexFundsInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub zeta_base_vault: solana_pubkey::Pubkey,
    pub zeta_quote_vault: solana_pubkey::Pubkey,
    pub dex_base_vault: solana_pubkey::Pubkey,
    pub dex_quote_vault: solana_pubkey::Pubkey,
    pub vault_owner: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SettleDexFunds {
    type ArrangedAccounts = SettleDexFundsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            market,
            zeta_base_vault,
            zeta_quote_vault,
            dex_base_vault,
            dex_quote_vault,
            vault_owner,
            mint_authority,
            serum_authority,
            dex_program,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(SettleDexFundsInstructionAccounts {
            state: state.pubkey,
            market: market.pubkey,
            zeta_base_vault: zeta_base_vault.pubkey,
            zeta_quote_vault: zeta_quote_vault.pubkey,
            dex_base_vault: dex_base_vault.pubkey,
            dex_quote_vault: dex_quote_vault.pubkey,
            vault_owner: vault_owner.pubkey,
            mint_authority: mint_authority.pubkey,
            serum_authority: serum_authority.pubkey,
            dex_program: dex_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
