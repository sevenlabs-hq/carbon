use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x99f15de116454a3d")]
pub struct Launch {
    pub symbol: String,
    pub name: String,
    pub uri: String,
}

pub struct LaunchInstructionAccounts {
    pub creator: solana_sdk::pubkey::Pubkey,
    pub creator_virtuals_ata: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub platform_prototype: solana_sdk::pubkey::Pubkey,
    pub platform_prototype_virtuals_ata: solana_sdk::pubkey::Pubkey,
    pub vpool: solana_sdk::pubkey::Pubkey,
    pub token_metadata: solana_sdk::pubkey::Pubkey,
    pub metadata_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Launch {
    type ArrangedAccounts = LaunchInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [creator, creator_virtuals_ata, token_mint, platform_prototype, platform_prototype_virtuals_ata, vpool, token_metadata, metadata_program, token_program, associated_token_program, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(LaunchInstructionAccounts {
            creator: creator.pubkey,
            creator_virtuals_ata: creator_virtuals_ata.pubkey,
            token_mint: token_mint.pubkey,
            platform_prototype: platform_prototype.pubkey,
            platform_prototype_virtuals_ata: platform_prototype_virtuals_ata.pubkey,
            vpool: vpool.pubkey,
            token_metadata: token_metadata.pubkey,
            metadata_program: metadata_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
