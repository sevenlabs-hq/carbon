use juniper::GraphQLEnum;


#[derive(Debug, Clone, GraphQLEnum)]
#[graphql(name = "PoolStatusBitIndex")]
pub enum PoolStatusBitIndexGraphQL {
        OpenPositionOrIncreaseLiquidity,
        DecreaseLiquidity,
        CollectFee,
        CollectReward,
        Swap,
}

impl From<crate::types::PoolStatusBitIndex> for PoolStatusBitIndexGraphQL {
    fn from(original: crate::types::PoolStatusBitIndex) -> Self {
        match original {
            crate::types::PoolStatusBitIndex::OpenPositionOrIncreaseLiquidity => Self::OpenPositionOrIncreaseLiquidity,
            crate::types::PoolStatusBitIndex::DecreaseLiquidity => Self::DecreaseLiquidity,
            crate::types::PoolStatusBitIndex::CollectFee => Self::CollectFee,
            crate::types::PoolStatusBitIndex::CollectReward => Self::CollectReward,
            crate::types::PoolStatusBitIndex::Swap => Self::Swap,
        }
    }
}


