use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dd3c519128e56711b")]
pub struct SignedMsgOrderRecordEvent {
    pub user: solana_pubkey::Pubkey,
    pub hash: String,
    pub matching_order_params: OrderParams,
    pub user_order_id: u32,
    pub signed_msg_order_max_slot: u64,
    pub signed_msg_order_uuid: [u8; 8],
    pub ts: i64,
}
