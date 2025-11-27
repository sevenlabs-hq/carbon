use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x460632f8de018f31")]
pub struct SignedMsgUserOrders {
    pub authority_pubkey: solana_pubkey::Pubkey,
    pub padding: u32,
    pub signed_msg_order_data: Vec<SignedMsgOrderId>,
}
