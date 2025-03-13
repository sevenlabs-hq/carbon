use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x10e883739c64ef32")]
pub struct UpdateDeactivated {
    pub stake_index: u32,
}

pub struct UpdateDeactivatedInstructionAccounts {
    pub common: solana_sdk::pubkey::Pubkey,
    pub operational_sol_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateDeactivated {
    type ArrangedAccounts = UpdateDeactivatedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [common, operational_sol_account, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateDeactivatedInstructionAccounts {
            common: common.pubkey,
            operational_sol_account: operational_sol_account.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
