use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "ProtocolPositionState")]
pub struct ProtocolPositionStateGraphQL {
        pub bump: U8,
        pub pool_id: Pubkey,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub liquidity: U128,
        pub fee_growth_inside0_last_x64: U128,
        pub fee_growth_inside1_last_x64: U128,
        pub token_fees_owed0: U64,
        pub token_fees_owed1: U64,
        pub reward_growth_inside: Vec<U128>,
        pub recent_epoch: U64,
        pub padding: Vec<U64>,
}

impl From<crate::accounts::postgres::ProtocolPositionStateRow> for ProtocolPositionStateGraphQL {
    fn from(row: crate::accounts::postgres::ProtocolPositionStateRow) -> Self {
        Self {
            bump: carbon_core::graphql::primitives::U8((*row.bump) as u8),
            pool_id: carbon_core::graphql::primitives::Pubkey(*row.pool_id),
            tick_lower_index: row.tick_lower_index,
            tick_upper_index: row.tick_upper_index,
            liquidity: carbon_core::graphql::primitives::U128(*row.liquidity),
            fee_growth_inside0_last_x64: carbon_core::graphql::primitives::U128(*row.fee_growth_inside0_last_x64),
            fee_growth_inside1_last_x64: carbon_core::graphql::primitives::U128(*row.fee_growth_inside1_last_x64),
            token_fees_owed0: carbon_core::graphql::primitives::U64(*row.token_fees_owed0),
            token_fees_owed1: carbon_core::graphql::primitives::U64(*row.token_fees_owed1),
            reward_growth_inside: row.reward_growth_inside.into_iter().map(|item| carbon_core::graphql::primitives::U128(*item)).collect(),
            recent_epoch: carbon_core::graphql::primitives::U64(*row.recent_epoch),
            padding: row.padding.into_iter().map(|item| carbon_core::graphql::primitives::U64(*item)).collect(),
        }
    }
}