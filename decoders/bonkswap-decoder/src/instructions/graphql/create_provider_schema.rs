use crate::types::graphql::TokenGraphQL;
use carbon_core::graphql::primitives::U8;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CreateProvider")]
pub struct CreateProviderGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
    pub token_x_amount: TokenGraphQL,
    pub token_y_amount: TokenGraphQL,
    pub bump: U8,
}

impl TryFrom<crate::instructions::postgres::CreateProviderRow> for CreateProviderGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(
        row: crate::instructions::postgres::CreateProviderRow,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            token_x_amount: row.token_x_amount.0.into(),
            token_y_amount: row.token_y_amount.0.into(),
            bump: carbon_core::graphql::primitives::U8((*row.bump) as u8),
        })
    }
}
