use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x890a579612734fa8")]
pub struct ResizeSignedMsgUserOrders {
    pub num_orders: u16,
}

pub struct ResizeSignedMsgUserOrdersInstructionAccounts {
    pub signed_msg_user_orders: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ResizeSignedMsgUserOrders {
    type ArrangedAccounts = ResizeSignedMsgUserOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [signed_msg_user_orders, authority, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ResizeSignedMsgUserOrdersInstructionAccounts {
            signed_msg_user_orders: signed_msg_user_orders.pubkey,
            authority: authority.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
