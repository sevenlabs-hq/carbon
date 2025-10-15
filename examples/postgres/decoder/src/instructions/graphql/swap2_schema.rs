use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;
use crate::types::graphql::RemainingAccountsInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Swap2")]
pub struct Swap2GraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub amount_in: U64,
        pub min_amount_out: U64,
        pub remaining_accounts_info: RemainingAccountsInfoGraphQL,
}

impl TryFrom<crate::instructions::postgres::Swap2Row> for Swap2GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::Swap2Row) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            amount_in: carbon_core::graphql::primitives::U64(*row.amount_in),
            min_amount_out: carbon_core::graphql::primitives::U64(*row.min_amount_out),
            remaining_accounts_info: row.remaining_accounts_info.0.into(),
        })
    }
}