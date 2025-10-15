use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "InitializeReward")]
pub struct InitializeRewardGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub reward_index: U64,
        pub reward_duration: U64,
        pub funder: Pubkey,
}

impl TryFrom<crate::instructions::postgres::InitializeRewardRow> for InitializeRewardGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::InitializeRewardRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            reward_index: carbon_core::graphql::primitives::U64(*row.reward_index),
            reward_duration: carbon_core::graphql::primitives::U64(*row.reward_duration),
            funder: carbon_core::graphql::primitives::Pubkey(*row.funder),
        })
    }
}