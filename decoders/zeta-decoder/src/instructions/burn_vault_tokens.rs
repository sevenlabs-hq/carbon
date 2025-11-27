use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe9cba5c9af2bbc9f")]
pub struct BurnVaultTokens {}

pub struct BurnVaultTokensInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BurnVaultTokens {
    type ArrangedAccounts = BurnVaultTokensInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            mint,
            vault,
            serum_authority,
            token_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(BurnVaultTokensInstructionAccounts {
            state: state.pubkey,
            mint: mint.pubkey,
            vault: vault.pubkey,
            serum_authority: serum_authority.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
