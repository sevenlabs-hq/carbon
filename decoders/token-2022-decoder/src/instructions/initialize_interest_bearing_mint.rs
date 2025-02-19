use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x21")]
pub struct InitializeInterestBearingMint {
    pub interest_bearing_mint_discriminator: u8,
    pub rate_authority: Option<solana_sdk::pubkey::Pubkey>,
    pub rate: i16,
}

pub struct InitializeInterestBearingMintInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeInterestBearingMint {
    type ArrangedAccounts = InitializeInterestBearingMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeInterestBearingMintInstructionAccounts { mint: mint.pubkey })
    }
}
