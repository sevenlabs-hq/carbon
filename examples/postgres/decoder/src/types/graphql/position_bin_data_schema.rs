use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U128;
use crate::types::graphql::FeeInfoGraphQL;
use crate::types::graphql::UserRewardInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "PositionBinData")]
pub struct PositionBinDataGraphQL {
        pub liquidity_share: U128,
        pub reward_info: UserRewardInfoGraphQL,
        pub fee_info: FeeInfoGraphQL,
}

impl From<crate::types::PositionBinData> for PositionBinDataGraphQL {
    fn from(original: crate::types::PositionBinData) -> Self {
        Self {
            liquidity_share: carbon_core::graphql::primitives::U128(original.liquidity_share),
            reward_info: original.reward_info.into(),
            fee_info: original.fee_info.into(),
        }
    }
}