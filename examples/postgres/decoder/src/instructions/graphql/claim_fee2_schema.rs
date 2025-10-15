use juniper::GraphQLObject;
use crate::types::graphql::RemainingAccountsInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "ClaimFee2")]
pub struct ClaimFee2GraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub min_bin_id: i32,
        pub max_bin_id: i32,
        pub remaining_accounts_info: RemainingAccountsInfoGraphQL,
}

impl TryFrom<crate::instructions::postgres::ClaimFee2Row> for ClaimFee2GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::ClaimFee2Row) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            min_bin_id: row.min_bin_id,
            max_bin_id: row.max_bin_id,
            remaining_accounts_info: row.remaining_accounts_info.0.into(),
        })
    }
}