use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "ClaimReward")]
pub struct ClaimRewardGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub reward_index: U64,
}

impl TryFrom<crate::instructions::postgres::ClaimRewardRow> for ClaimRewardGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::ClaimRewardRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            reward_index: carbon_core::graphql::primitives::U64(*row.reward_index),
        })
    }
}