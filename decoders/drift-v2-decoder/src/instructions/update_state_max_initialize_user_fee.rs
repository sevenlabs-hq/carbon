use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xede119edc12d4d61")]
pub struct UpdateStateMaxInitializeUserFee {
    pub max_initialize_user_fee: u16,
}

pub struct UpdateStateMaxInitializeUserFeeInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateStateMaxInitializeUserFee {
    type ArrangedAccounts = UpdateStateMaxInitializeUserFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateStateMaxInitializeUserFeeInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
        })
    }
}
