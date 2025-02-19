use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6c25ab8ff81e126e")]
pub struct UpdateTokenGroupMaxSize {
    pub max_size: u64,
}

pub struct UpdateTokenGroupMaxSizeInstructionAccounts {
    pub group: solana_sdk::pubkey::Pubkey,
    pub update_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateTokenGroupMaxSize {
    type ArrangedAccounts = UpdateTokenGroupMaxSizeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [group, update_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateTokenGroupMaxSizeInstructionAccounts {
            group: group.pubkey,
            update_authority: update_authority.pubkey,
        })
    }
}
