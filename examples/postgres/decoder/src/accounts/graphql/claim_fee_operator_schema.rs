use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "ClaimFeeOperator")]
pub struct ClaimFeeOperatorGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
            /// operator
            pub operator: Pubkey,
            /// Reserve
            pub padding: Vec<U8>,
}

impl TryFrom<crate::accounts::postgres::ClaimFeeOperatorRow> for ClaimFeeOperatorGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::accounts::postgres::ClaimFeeOperatorRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            operator: carbon_core::graphql::primitives::Pubkey(*row.operator),
            padding: row.padding.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
        })
    }
}