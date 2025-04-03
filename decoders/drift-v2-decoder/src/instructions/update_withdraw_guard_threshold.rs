use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3812273d9bd32c85")]
pub struct UpdateWithdrawGuardThreshold {
    pub withdraw_guard_threshold: u64,
}

pub struct UpdateWithdrawGuardThresholdInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateWithdrawGuardThreshold {
    type ArrangedAccounts = UpdateWithdrawGuardThresholdInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, state, spot_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateWithdrawGuardThresholdInstructionAccounts {
            admin: admin.pubkey,
            state: state.pubkey,
            spot_market: spot_market.pubkey,
        })
    }
}
