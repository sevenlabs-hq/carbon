use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xdf414307bd033f8e")]
pub struct CancelTriggerOrderV2 {
    pub trigger_order_bit: u8,
}

pub struct CancelTriggerOrderV2InstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub trigger_order: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelTriggerOrderV2 {
    type ArrangedAccounts = CancelTriggerOrderV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, trigger_order, margin_account, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CancelTriggerOrderV2InstructionAccounts {
            authority: authority.pubkey,
            trigger_order: trigger_order.pubkey,
            margin_account: margin_account.pubkey,
        })
    }
}
