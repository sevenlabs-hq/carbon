use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;
use crate::types::graphql::RemainingAccountsInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "FundReward")]
pub struct FundRewardGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub reward_index: U64,
        pub amount: U64,
        pub carry_forward: bool,
        pub remaining_accounts_info: RemainingAccountsInfoGraphQL,
}

impl TryFrom<crate::instructions::postgres::FundRewardRow> for FundRewardGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::FundRewardRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            reward_index: carbon_core::graphql::primitives::U64(*row.reward_index),
            amount: carbon_core::graphql::primitives::U64(*row.amount),
            carry_forward: row.carry_forward,
            remaining_accounts_info: row.remaining_accounts_info.0.into(),
        })
    }
}