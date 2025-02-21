use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb4ecfd13e7e7dc41")]
pub struct UpdateMakerRebatePercentage {
    pub native_maker_rebate_percentage: u64,
}

pub struct UpdateMakerRebatePercentageInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateMakerRebatePercentage {
    type ArrangedAccounts = UpdateMakerRebatePercentageInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateMakerRebatePercentageInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
        })
    }
}
