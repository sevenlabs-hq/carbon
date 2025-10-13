use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;
use crate::types::graphql::TickStateGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "TickArrayState")]
pub struct TickArrayStateGraphQL {
        pub pool_id: Pubkey,
        pub start_tick_index: i32,
        pub ticks: Vec<TickStateGraphQL>,
        pub initialized_tick_count: U8,
        pub recent_epoch: U64,
        pub padding: Vec<U8>,
}

impl From<crate::accounts::postgres::TickArrayStateRow> for TickArrayStateGraphQL {
    fn from(row: crate::accounts::postgres::TickArrayStateRow) -> Self {
        Self {
            pool_id: carbon_core::graphql::primitives::Pubkey(*row.pool_id),
            start_tick_index: row.start_tick_index,
            ticks: row.ticks.0.into_iter().map(|item| item.into()).collect(),
            initialized_tick_count: carbon_core::graphql::primitives::U8((*row.initialized_tick_count) as u8),
            recent_epoch: carbon_core::graphql::primitives::U64(*row.recent_epoch),
            padding: row.padding.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
        }
    }
}