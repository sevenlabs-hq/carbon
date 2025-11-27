use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa6cbb0d2b0348c69")]
pub struct InitializeCrossMarginAccountManagerV2 {
    pub referrer: Option<solana_pubkey::Pubkey>,
}

pub struct InitializeCrossMarginAccountManagerV2InstructionAccounts {
    pub cross_margin_account_manager: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub zeta_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeCrossMarginAccountManagerV2 {
    type ArrangedAccounts = InitializeCrossMarginAccountManagerV2InstructionAccounts;

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

        Some(InitializeCrossMarginAccountManagerV2InstructionAccounts {
            cross_margin_account_manager: cross_margin_account_manager.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            zeta_program: zeta_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
