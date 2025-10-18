use crate::types::graphql::TokenGraphQL;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "WithdrawShares")]
pub struct WithdrawSharesGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
    pub shares: TokenGraphQL,
}

impl TryFrom<crate::instructions::postgres::WithdrawSharesRow> for WithdrawSharesGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(
        row: crate::instructions::postgres::WithdrawSharesRow,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            shares: row.shares.0.into(),
        })
    }
}
