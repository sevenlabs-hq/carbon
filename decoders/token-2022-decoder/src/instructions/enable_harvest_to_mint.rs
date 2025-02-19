use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x25")]
pub struct EnableHarvestToMint {
    pub confidential_transfer_fee_discriminator: u8,
}

pub struct EnableHarvestToMintInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EnableHarvestToMint {
    type ArrangedAccounts = EnableHarvestToMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(EnableHarvestToMintInstructionAccounts {
            mint: mint.pubkey,
            authority: authority.pubkey,
        })
    }
}
