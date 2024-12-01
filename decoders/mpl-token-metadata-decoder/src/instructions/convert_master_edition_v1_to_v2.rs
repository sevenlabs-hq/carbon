

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0c")]
pub struct ConvertMasterEditionV1ToV2{
}

pub struct ConvertMasterEditionV1ToV2InstructionAccounts {
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub one_time_auth: solana_sdk::pubkey::Pubkey,
    pub printing_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConvertMasterEditionV1ToV2 {
    type ArrangedAccounts = ConvertMasterEditionV1ToV2InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let master_edition = accounts.get(0)?;
        let one_time_auth = accounts.get(1)?;
        let printing_mint = accounts.get(2)?;

        Some(ConvertMasterEditionV1ToV2InstructionAccounts {
            master_edition: master_edition.pubkey,
            one_time_auth: one_time_auth.pubkey,
            printing_mint: printing_mint.pubkey,
        })
    }
}