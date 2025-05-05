use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfdb87ec7ebe8aca2")]
pub struct CreateTokenFallback {
    pub salt: u64,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateTokenFallbackInstructionAccounts {
    pub config: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_metadata_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateTokenFallback {
    type ArrangedAccounts = CreateTokenFallbackInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config, metadata, mint, payer, rent, system_program, token_program, token_metadata_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateTokenFallbackInstructionAccounts {
            config: config.pubkey,
            metadata: metadata.pubkey,
            mint: mint.pubkey,
            payer: payer.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            token_metadata_program: token_metadata_program.pubkey,
        })
    }
}
