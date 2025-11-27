use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x61a1f1a70620d535")]
pub struct Restake {
    pub amount: u64,
}

pub struct RestakeInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub lst_mint: solana_pubkey::Pubkey,
    pub lst_ata: solana_pubkey::Pubkey,
    pub rst_ata: solana_pubkey::Pubkey,
    pub rst_mint: solana_pubkey::Pubkey,
    pub vault: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Restake {
    type ArrangedAccounts = RestakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            signer,
            lst_mint,
            lst_ata,
            rst_ata,
            rst_mint,
            vault,
            pool,
            associated_token_program,
            token_program,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(RestakeInstructionAccounts {
            signer: signer.pubkey,
            lst_mint: lst_mint.pubkey,
            lst_ata: lst_ata.pubkey,
            rst_ata: rst_ata.pubkey,
            rst_mint: rst_mint.pubkey,
            vault: vault.pubkey,
            pool: pool.pubkey,
            associated_token_program: associated_token_program.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
