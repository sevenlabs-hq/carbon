use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0443514088f55d98")]
pub struct UpdateActive {
    pub stake_index: u32,
    pub validator_index: u32,
}

pub struct UpdateActiveInstructionAccounts {
    pub common: solana_sdk::pubkey::Pubkey,
    pub validator_list: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateActive {
    type ArrangedAccounts = UpdateActiveInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [common, validator_list, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateActiveInstructionAccounts {
            common: common.pubkey,
            validator_list: validator_list.pubkey,
        })
    }
}
