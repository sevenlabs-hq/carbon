use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize {}

pub struct InitializeInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub virtuals_mint: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub vpool_virtuals_ata: solana_pubkey::Pubkey,
    pub vpool_token_ata: solana_pubkey::Pubkey,
    pub vpool: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            payer,
            virtuals_mint,
            token_mint,
            vpool_virtuals_ata,
            vpool_token_ata,
            vpool,
            token_program,
            associated_token_program,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(InitializeInstructionAccounts {
            payer: payer.pubkey,
            virtuals_mint: virtuals_mint.pubkey,
            token_mint: token_mint.pubkey,
            vpool_virtuals_ata: vpool_virtuals_ata.pubkey,
            vpool_token_ata: vpool_token_ata.pubkey,
            vpool: vpool.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
