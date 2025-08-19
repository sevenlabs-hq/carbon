use {
    super::super::types::*,
    alloc::string::String,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0718b6199b904b32")]
pub struct PlacePerpOrderV5 {
    pub price: u64,
    pub size: u64,
    pub side: Side,
    pub order_type: OrderType,
    pub reduce_only: bool,
    pub client_order_id: Option<u64>,
    pub tag: Option<String>,
    pub tif_offset: Option<u16>,
    pub asset: Asset,
    pub self_trade_behavior: Option<SelfTradeBehaviorZeta>,
}

pub struct PlacePerpOrderV5InstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub place_order_accounts: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PlacePerpOrderV5 {
    type ArrangedAccounts = PlacePerpOrderV5InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, place_order_accounts, _remaining @ ..] = accounts else {
            return None;
        };

        Some(PlacePerpOrderV5InstructionAccounts {
            authority: authority.pubkey,
            place_order_accounts: place_order_accounts.pubkey,
        })
    }
}
