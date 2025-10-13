use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Swap")]
pub struct SwapGraphQL {
        pub amount: U64,
        pub other_amount_threshold: U64,
        pub sqrt_price_limit_x64: U128,
        pub is_base_input: bool,
}

impl From<crate::instructions::postgres::SwapRow> for SwapGraphQL {
    fn from(row: crate::instructions::postgres::SwapRow) -> Self {
        Self {
            amount: carbon_core::graphql::primitives::U64(*row.amount),
            other_amount_threshold: carbon_core::graphql::primitives::U64(*row.other_amount_threshold),
            sqrt_price_limit_x64: carbon_core::graphql::primitives::U128(*row.sqrt_price_limit_x64),
            is_base_input: row.is_base_input,
        }
    }
}