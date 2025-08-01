use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x06")]
pub struct PermissionRefund {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PermissionRefundInstructionAccounts {
    pub consumed_permission: solana_pubkey::Pubkey,
    pub refund_destination: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PermissionRefund {
    type ArrangedAccounts = PermissionRefundInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let consumed_permission = next_account(&mut iter)?;
        let refund_destination = next_account(&mut iter)?;

        Some(PermissionRefundInstructionAccounts {
            consumed_permission,
            refund_destination,
        })
    }
}
