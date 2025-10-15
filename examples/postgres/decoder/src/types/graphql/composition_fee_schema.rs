use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CompositionFee")]
pub struct CompositionFeeGraphQL {
        pub from: Pubkey,
        pub bin_id: i32,
        pub token_x_fee_amount: U64,
        pub token_y_fee_amount: U64,
        pub protocol_token_x_fee_amount: U64,
        pub protocol_token_y_fee_amount: U64,
}

impl From<crate::types::CompositionFee> for CompositionFeeGraphQL {
    fn from(original: crate::types::CompositionFee) -> Self {
        Self {
            from: carbon_core::graphql::primitives::Pubkey(original.from),
            bin_id: original.bin_id as i32,
            token_x_fee_amount: carbon_core::graphql::primitives::U64(original.token_x_fee_amount),
            token_y_fee_amount: carbon_core::graphql::primitives::U64(original.token_y_fee_amount),
            protocol_token_x_fee_amount: carbon_core::graphql::primitives::U64(original.protocol_token_x_fee_amount),
            protocol_token_y_fee_amount: carbon_core::graphql::primitives::U64(original.protocol_token_y_fee_amount),
        }
    }
}