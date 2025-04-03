use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x43eb4266a7ab78c5")]
pub struct InitializeMarginAccount {}

pub struct InitializeMarginAccountInstructionAccounts {
    pub margin_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub zeta_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeMarginAccount {
    type ArrangedAccounts = InitializeMarginAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [margin_account, authority, payer, zeta_program, system_program, zeta_group, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeMarginAccountInstructionAccounts {
            margin_account: margin_account.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            zeta_program: zeta_program.pubkey,
            system_program: system_program.pubkey,
            zeta_group: zeta_group.pubkey,
        })
    }
}
