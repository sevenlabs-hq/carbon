use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1a")]
pub struct HarvestWithheldTokensToMint {
    pub transfer_fee_discriminator: u8,
}

pub struct HarvestWithheldTokensToMintInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for HarvestWithheldTokensToMint {
    type ArrangedAccounts = HarvestWithheldTokensToMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(HarvestWithheldTokensToMintInstructionAccounts { mint: mint.pubkey })
    }
}
