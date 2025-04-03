use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb1c878866ed99351")]
pub struct GiveUpPendingFees {
    pub max_amount_to_give_up: u64,
}

pub struct GiveUpPendingFeesInstructionAccounts {
    pub admin_authority: solana_pubkey::Pubkey,
    pub vault_state: solana_pubkey::Pubkey,
    pub klend_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GiveUpPendingFees {
    type ArrangedAccounts = GiveUpPendingFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority, vault_state, klend_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(GiveUpPendingFeesInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            vault_state: vault_state.pubkey,
            klend_program: klend_program.pubkey,
        })
    }
}
