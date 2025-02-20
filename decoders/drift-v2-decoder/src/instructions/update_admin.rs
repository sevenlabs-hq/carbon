use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa1b028d53cb8b3e4")]
pub struct UpdateAdmin {
    pub admin: solana_sdk::pubkey::Pubkey,
}

pub struct UpdateAdminInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateAdmin {
    type ArrangedAccounts = UpdateAdminInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateAdminInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
