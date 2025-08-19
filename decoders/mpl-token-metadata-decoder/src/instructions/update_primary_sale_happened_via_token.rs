use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x04")]
pub struct UpdatePrimarySaleHappenedViaToken {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdatePrimarySaleHappenedViaTokenInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub token: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePrimarySaleHappenedViaToken {
    type ArrangedAccounts = UpdatePrimarySaleHappenedViaTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let metadata = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let token = next_account(&mut iter)?;

        Some(UpdatePrimarySaleHappenedViaTokenInstructionAccounts {
            metadata,
            owner,
            token,
        })
    }
}
