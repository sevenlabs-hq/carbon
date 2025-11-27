use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xce56fb1b5b6f17d3")]
pub struct InitializeSpreadAccount {}

pub struct InitializeSpreadAccountInstructionAccounts {
    pub spread_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub zeta_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeSpreadAccount {
    type ArrangedAccounts = InitializeSpreadAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            spread_account,
            authority,
            payer,
            zeta_program,
            system_program,
            zeta_group,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(InitializeSpreadAccountInstructionAccounts {
            spread_account: spread_account.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            zeta_program: zeta_program.pubkey,
            system_program: system_program.pubkey,
            zeta_group: zeta_group.pubkey,
        })
    }
}
