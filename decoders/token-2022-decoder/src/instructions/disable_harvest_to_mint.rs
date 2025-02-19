use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x25")]
pub struct DisableHarvestToMint {
    pub confidential_transfer_fee_discriminator: u8,
}

pub struct DisableHarvestToMintInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DisableHarvestToMint {
    type ArrangedAccounts = DisableHarvestToMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DisableHarvestToMintInstructionAccounts {
            mint: mint.pubkey,
            authority: authority.pubkey,
        })
    }
}
