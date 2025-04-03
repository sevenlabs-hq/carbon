use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x905443271b19ca8d")]
pub struct CancelTriggerOrder {
    pub trigger_order_bit: u8,
}

pub struct CancelTriggerOrderInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub trigger_order: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelTriggerOrder {
    type ArrangedAccounts = CancelTriggerOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, payer, trigger_order, margin_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CancelTriggerOrderInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
            payer: payer.pubkey,
            trigger_order: trigger_order.pubkey,
            margin_account: margin_account.pubkey,
        })
    }
}
