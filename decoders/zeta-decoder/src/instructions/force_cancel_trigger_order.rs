use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x78ecd81cc04fffbc")]
pub struct ForceCancelTriggerOrder {
    pub trigger_order_bit: u8,
    pub enforce_tpsl_conditions: bool,
}

pub struct ForceCancelTriggerOrderInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub trigger_order: solana_sdk::pubkey::Pubkey,
    pub margin_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ForceCancelTriggerOrder {
    type ArrangedAccounts = ForceCancelTriggerOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, trigger_order, margin_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ForceCancelTriggerOrderInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
            trigger_order: trigger_order.pubkey,
            margin_account: margin_account.pubkey,
        })
    }
}
