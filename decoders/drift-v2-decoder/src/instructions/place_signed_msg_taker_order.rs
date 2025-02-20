use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x204f658b1906620f")]
pub struct PlaceSignedMsgTakerOrder {
    pub signed_msg_order_params_message_bytes: Vec<u8>,
    pub is_delegate_signer: bool,
}

pub struct PlaceSignedMsgTakerOrderInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub user_stats: solana_sdk::pubkey::Pubkey,
    pub signed_msg_user_orders: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub ix_sysvar: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PlaceSignedMsgTakerOrder {
    type ArrangedAccounts = PlaceSignedMsgTakerOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, user, user_stats, signed_msg_user_orders, authority, ix_sysvar, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PlaceSignedMsgTakerOrderInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
            signed_msg_user_orders: signed_msg_user_orders.pubkey,
            authority: authority.pubkey,
            ix_sysvar: ix_sysvar.pubkey,
        })
    }
}
