use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;
use crate::types::graphql::RemainingAccountsInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "WithdrawIneligibleReward")]
pub struct WithdrawIneligibleRewardGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub reward_index: U64,
        pub remaining_accounts_info: RemainingAccountsInfoGraphQL,
}

impl TryFrom<crate::instructions::postgres::WithdrawIneligibleRewardRow> for WithdrawIneligibleRewardGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::WithdrawIneligibleRewardRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            reward_index: carbon_core::graphql::primitives::U64(*row.reward_index),
            remaining_accounts_info: row.remaining_accounts_info.0.into(),
        })
    }
}