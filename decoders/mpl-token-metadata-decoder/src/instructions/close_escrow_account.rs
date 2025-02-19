use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x27")]
pub struct CloseEscrowAccount {}

pub struct CloseEscrowAccountInstructionAccounts {
    pub escrow: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub edition: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub sysvar_instructions: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseEscrowAccount {
    type ArrangedAccounts = CloseEscrowAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [escrow, metadata, mint, token_account, edition, payer, system_program, sysvar_instructions] =
            accounts
        else {
            return None;
        };

        Some(CloseEscrowAccountInstructionAccounts {
            escrow: escrow.pubkey,
            metadata: metadata.pubkey,
            mint: mint.pubkey,
            token_account: token_account.pubkey,
            edition: edition.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
        })
    }
}
