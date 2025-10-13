use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "TickArrayBitmapExtension")]
pub struct TickArrayBitmapExtensionGraphQL {
        pub pool_id: Pubkey,
        pub positive_tick_array_bitmap: Vec<Vec<U64>>,
        pub negative_tick_array_bitmap: Vec<Vec<U64>>,
}

impl From<crate::accounts::postgres::TickArrayBitmapExtensionRow> for TickArrayBitmapExtensionGraphQL {
    fn from(row: crate::accounts::postgres::TickArrayBitmapExtensionRow) -> Self {
        Self {
            pool_id: carbon_core::graphql::primitives::Pubkey(*row.pool_id),
            positive_tick_array_bitmap: row.positive_tick_array_bitmap.0.into_iter().map(|item| item.into_iter().map(|item| carbon_core::graphql::primitives::U64(*item)).collect()).collect(),
            negative_tick_array_bitmap: row.negative_tick_array_bitmap.0.into_iter().map(|item| item.into_iter().map(|item| carbon_core::graphql::primitives::U64(*item)).collect()).collect(),
        }
    }
}