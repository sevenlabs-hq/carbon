use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CreatePool")]
pub struct CreatePoolGraphQL {
        pub sqrt_price_x64: U128,
        pub open_time: U64,
}

impl From<crate::instructions::postgres::CreatePoolRow> for CreatePoolGraphQL {
    fn from(row: crate::instructions::postgres::CreatePoolRow) -> Self {
        Self {
            sqrt_price_x64: carbon_core::graphql::primitives::U128(*row.sqrt_price_x64),
            open_time: carbon_core::graphql::primitives::U64(*row.open_time),
        }
    }
}