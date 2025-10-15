use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Swap")]
pub struct SwapGraphQL {
        pub lb_pair: Pubkey,
        pub from: Pubkey,
        pub start_bin_id: i32,
        pub end_bin_id: i32,
        pub amount_in: U64,
        pub amount_out: U64,
        pub swap_for_y: bool,
        pub fee: U64,
        pub protocol_fee: U64,
        pub fee_bps: U128,
        pub host_fee: U64,
}

impl From<crate::types::Swap> for SwapGraphQL {
    fn from(original: crate::types::Swap) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            from: carbon_core::graphql::primitives::Pubkey(original.from),
            start_bin_id: original.start_bin_id,
            end_bin_id: original.end_bin_id,
            amount_in: carbon_core::graphql::primitives::U64(original.amount_in),
            amount_out: carbon_core::graphql::primitives::U64(original.amount_out),
            swap_for_y: original.swap_for_y,
            fee: carbon_core::graphql::primitives::U64(original.fee),
            protocol_fee: carbon_core::graphql::primitives::U64(original.protocol_fee),
            fee_bps: carbon_core::graphql::primitives::U128(original.fee_bps),
            host_fee: carbon_core::graphql::primitives::U64(original.host_fee),
        }
    }
}