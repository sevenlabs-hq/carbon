use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0c")]
pub struct ConvertMasterEditionV1ToV2 {}

pub struct ConvertMasterEditionV1ToV2InstructionAccounts {
    pub master_edition: solana_pubkey::Pubkey,
    pub one_time_auth: solana_pubkey::Pubkey,
    pub printing_mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConvertMasterEditionV1ToV2 {
    type ArrangedAccounts = ConvertMasterEditionV1ToV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [master_edition, one_time_auth, printing_mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ConvertMasterEditionV1ToV2InstructionAccounts {
            master_edition: master_edition.pubkey,
            one_time_auth: one_time_auth.pubkey,
            printing_mint: printing_mint.pubkey,
        })
    }
}
