use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;
use crate::types::graphql::RemainingAccountsInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SwapExactOut2")]
pub struct SwapExactOut2GraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub max_in_amount: U64,
        pub out_amount: U64,
        pub remaining_accounts_info: RemainingAccountsInfoGraphQL,
}

impl TryFrom<crate::instructions::postgres::SwapExactOut2Row> for SwapExactOut2GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::SwapExactOut2Row) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            max_in_amount: carbon_core::graphql::primitives::U64(*row.max_in_amount),
            out_amount: carbon_core::graphql::primitives::U64(*row.out_amount),
            remaining_accounts_info: row.remaining_accounts_info.0.into(),
        })
    }
}