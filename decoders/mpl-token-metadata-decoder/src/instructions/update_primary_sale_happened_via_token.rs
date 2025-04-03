use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x04")]
pub struct UpdatePrimarySaleHappenedViaToken {}

pub struct UpdatePrimarySaleHappenedViaTokenInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub token: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePrimarySaleHappenedViaToken {
    type ArrangedAccounts = UpdatePrimarySaleHappenedViaTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, owner, token, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdatePrimarySaleHappenedViaTokenInstructionAccounts {
            metadata: metadata.pubkey,
            owner: owner.pubkey,
            token: token.pubkey,
        })
    }
}
