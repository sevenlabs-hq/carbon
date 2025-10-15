use juniper::GraphQLObject;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;
use crate::types::graphql::FeeInfoGraphQL;
use crate::types::graphql::UserRewardInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Position")]
pub struct PositionGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
            /// The LB pair of this position
            pub lb_pair: Pubkey,
            /// Owner of the position. Client rely on this to to fetch their positions.
            pub owner: Pubkey,
            /// Liquidity shares of this position in bins (lower_bin_id <-> upper_bin_id). This is the same as LP concept.
            pub liquidity_shares: Vec<U64>,
            /// Farming reward information
            pub reward_infos: Vec<UserRewardInfoGraphQL>,
            /// Swap fee to claim information
            pub fee_infos: Vec<FeeInfoGraphQL>,
            /// Lower bin ID
            pub lower_bin_id: i32,
            /// Upper bin ID
            pub upper_bin_id: i32,
            /// Last updated timestamp
            pub last_updated_at: I64,
            /// Total claimed token fee X
            pub total_claimed_fee_x_amount: U64,
            /// Total claimed token fee Y
            pub total_claimed_fee_y_amount: U64,
            /// Total claimed rewards
            pub total_claimed_rewards: Vec<U64>,
            /// Reserved space for future use
            pub reserved: Vec<U8>,
}

impl TryFrom<crate::accounts::postgres::PositionRow> for PositionGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::accounts::postgres::PositionRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            lb_pair: carbon_core::graphql::primitives::Pubkey(*row.lb_pair),
            owner: carbon_core::graphql::primitives::Pubkey(*row.owner),
            liquidity_shares: row.liquidity_shares.into_iter().map(|item| carbon_core::graphql::primitives::U64(*item)).collect(),
            reward_infos: row.reward_infos.0.into_iter().map(|item| item.into()).collect(),
            fee_infos: row.fee_infos.0.into_iter().map(|item| item.into()).collect(),
            lower_bin_id: row.lower_bin_id,
            upper_bin_id: row.upper_bin_id,
            last_updated_at: carbon_core::graphql::primitives::I64(row.last_updated_at),
            total_claimed_fee_x_amount: carbon_core::graphql::primitives::U64(*row.total_claimed_fee_x_amount),
            total_claimed_fee_y_amount: carbon_core::graphql::primitives::U64(*row.total_claimed_fee_y_amount),
            total_claimed_rewards: row.total_claimed_rewards.into_iter().map(|item| carbon_core::graphql::primitives::U64(*item)).collect(),
            reserved: row.reserved.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
        })
    }
}