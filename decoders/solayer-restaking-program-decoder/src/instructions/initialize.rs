use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize {}

pub struct InitializeInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub solayer_admin: solana_pubkey::Pubkey,
    pub lst_mint: solana_pubkey::Pubkey,
    pub lst_vault: solana_pubkey::Pubkey,
    pub rst_mint: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [signer, solayer_admin, lst_mint, lst_vault, rst_mint, pool, associated_token_program, token_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeInstructionAccounts {
            signer: signer.pubkey,
            solayer_admin: solayer_admin.pubkey,
            lst_mint: lst_mint.pubkey,
            lst_vault: lst_vault.pubkey,
            rst_mint: rst_mint.pubkey,
            pool: pool.pubkey,
            associated_token_program: associated_token_program.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
