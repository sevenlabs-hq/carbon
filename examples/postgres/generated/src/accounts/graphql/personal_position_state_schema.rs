use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;
use crate::types::graphql::PositionRewardInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "PersonalPositionState")]
pub struct PersonalPositionStateGraphQL {
        pub bump: U8,
        pub nft_mint: Pubkey,
        pub pool_id: Pubkey,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub liquidity: U128,
        pub fee_growth_inside0_last_x64: U128,
        pub fee_growth_inside1_last_x64: U128,
        pub token_fees_owed0: U64,
        pub token_fees_owed1: U64,
        pub reward_infos: Vec<PositionRewardInfoGraphQL>,
        pub recent_epoch: U64,
        pub padding: Vec<U64>,
}

impl From<crate::accounts::postgres::PersonalPositionStateRow> for PersonalPositionStateGraphQL {
    fn from(row: crate::accounts::postgres::PersonalPositionStateRow) -> Self {
        Self {
            bump: carbon_core::graphql::primitives::U8((*row.bump) as u8),
            nft_mint: carbon_core::graphql::primitives::Pubkey(*row.nft_mint),
            pool_id: carbon_core::graphql::primitives::Pubkey(*row.pool_id),
            tick_lower_index: row.tick_lower_index,
            tick_upper_index: row.tick_upper_index,
            liquidity: carbon_core::graphql::primitives::U128(*row.liquidity),
            fee_growth_inside0_last_x64: carbon_core::graphql::primitives::U128(*row.fee_growth_inside0_last_x64),
            fee_growth_inside1_last_x64: carbon_core::graphql::primitives::U128(*row.fee_growth_inside1_last_x64),
            token_fees_owed0: carbon_core::graphql::primitives::U64(*row.token_fees_owed0),
            token_fees_owed1: carbon_core::graphql::primitives::U64(*row.token_fees_owed1),
            reward_infos: row.reward_infos.0.into_iter().map(|item| item.into()).collect(),
            recent_epoch: carbon_core::graphql::primitives::U64(*row.recent_epoch),
            padding: row.padding.into_iter().map(|item| carbon_core::graphql::primitives::U64(*item)).collect(),
        }
    }
}