use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x489a0f1ca5d7d1c7")]
pub struct InitializeCrossMarginAccountManager {}

pub struct InitializeCrossMarginAccountManagerInstructionAccounts {
    pub cross_margin_account_manager: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub zeta_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeCrossMarginAccountManager {
    type ArrangedAccounts = InitializeCrossMarginAccountManagerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            cross_margin_account_manager,
            authority,
            payer,
            zeta_program,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(InitializeCrossMarginAccountManagerInstructionAccounts {
            cross_margin_account_manager: cross_margin_account_manager.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            zeta_program: zeta_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
