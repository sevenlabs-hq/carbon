use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;
use crate::types::graphql::RemainingAccountsInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "ClaimReward2")]
pub struct ClaimReward2GraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub reward_index: U64,
        pub min_bin_id: i32,
        pub max_bin_id: i32,
        pub remaining_accounts_info: RemainingAccountsInfoGraphQL,
}

impl TryFrom<crate::instructions::postgres::ClaimReward2Row> for ClaimReward2GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::ClaimReward2Row) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            reward_index: carbon_core::graphql::primitives::U64(*row.reward_index),
            min_bin_id: row.min_bin_id,
            max_bin_id: row.max_bin_id,
            remaining_accounts_info: row.remaining_accounts_info.0.into(),
        })
    }
}