use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdateRewardDuration")]
pub struct UpdateRewardDurationGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub reward_index: U64,
        pub new_duration: U64,
}

impl TryFrom<crate::instructions::postgres::UpdateRewardDurationRow> for UpdateRewardDurationGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::UpdateRewardDurationRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            reward_index: carbon_core::graphql::primitives::U64(*row.reward_index),
            new_duration: carbon_core::graphql::primitives::U64(*row.new_duration),
        })
    }
}