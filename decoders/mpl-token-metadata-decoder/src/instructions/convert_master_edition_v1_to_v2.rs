use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0c")]
pub struct ConvertMasterEditionV1ToV2 {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ConvertMasterEditionV1ToV2InstructionAccounts {
    pub master_edition: solana_pubkey::Pubkey,
    pub one_time_auth: solana_pubkey::Pubkey,
    pub printing_mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ConvertMasterEditionV1ToV2 {
    type ArrangedAccounts = ConvertMasterEditionV1ToV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let master_edition = next_account(&mut iter)?;
        let one_time_auth = next_account(&mut iter)?;
        let printing_mint = next_account(&mut iter)?;

        Some(ConvertMasterEditionV1ToV2InstructionAccounts {
            master_edition,
            one_time_auth,
            printing_mint,
        })
    }
}
