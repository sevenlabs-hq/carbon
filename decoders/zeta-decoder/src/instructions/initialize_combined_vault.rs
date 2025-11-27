use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3b6369114977e5fc")]
pub struct InitializeCombinedVault {
    pub nonce: u8,
}

pub struct InitializeCombinedVaultInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub usdc_mint: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeCombinedVault {
    type ArrangedAccounts = InitializeCombinedVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            vault,
            token_program,
            usdc_mint,
            admin,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(InitializeCombinedVaultInstructionAccounts {
            state: state.pubkey,
            vault: vault.pubkey,
            token_program: token_program.pubkey,
            usdc_mint: usdc_mint.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
