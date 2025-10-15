use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdateRewardFunder")]
pub struct UpdateRewardFunderGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub reward_index: U64,
        pub new_funder: Pubkey,
}

impl TryFrom<crate::instructions::postgres::UpdateRewardFunderRow> for UpdateRewardFunderGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::UpdateRewardFunderRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            reward_index: carbon_core::graphql::primitives::U64(*row.reward_index),
            new_funder: carbon_core::graphql::primitives::Pubkey(*row.new_funder),
        })
    }
}