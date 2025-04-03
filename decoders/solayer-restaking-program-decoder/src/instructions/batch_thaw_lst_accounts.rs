use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb7ae4d28b686cad5")]
pub struct BatchThawLstAccounts {}

pub struct BatchThawLstAccountsInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub solayer_admin: solana_pubkey::Pubkey,
    pub lst_mint: solana_pubkey::Pubkey,
    pub rst_mint: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BatchThawLstAccounts {
    type ArrangedAccounts = BatchThawLstAccountsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [signer, solayer_admin, lst_mint, rst_mint, pool, associated_token_program, token_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(BatchThawLstAccountsInstructionAccounts {
            signer: signer.pubkey,
            solayer_admin: solayer_admin.pubkey,
            lst_mint: lst_mint.pubkey,
            rst_mint: rst_mint.pubkey,
            pool: pool.pubkey,
            associated_token_program: associated_token_program.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
