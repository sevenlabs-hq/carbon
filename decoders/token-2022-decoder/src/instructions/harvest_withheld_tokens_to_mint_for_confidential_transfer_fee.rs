use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x25")]
pub struct HarvestWithheldTokensToMintForConfidentialTransferFee {
    pub confidential_transfer_fee_discriminator: u8,
}

pub struct HarvestWithheldTokensToMintForConfidentialTransferFeeInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts
    for HarvestWithheldTokensToMintForConfidentialTransferFee
{
    type ArrangedAccounts =
        HarvestWithheldTokensToMintForConfidentialTransferFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(
            HarvestWithheldTokensToMintForConfidentialTransferFeeInstructionAccounts {
                mint: mint.pubkey,
            },
        )
    }
}
