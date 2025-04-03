use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xddf780fdd4fe2e99")]
pub struct DeleteSignedMsgUserOrders {}

pub struct DeleteSignedMsgUserOrdersInstructionAccounts {
    pub signed_msg_user_orders: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeleteSignedMsgUserOrders {
    type ArrangedAccounts = DeleteSignedMsgUserOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [signed_msg_user_orders, state, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DeleteSignedMsgUserOrdersInstructionAccounts {
            signed_msg_user_orders: signed_msg_user_orders.pubkey,
            state: state.pubkey,
            authority: authority.pubkey,
        })
    }
}
