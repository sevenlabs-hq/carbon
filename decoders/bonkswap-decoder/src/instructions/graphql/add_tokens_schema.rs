use crate::types::graphql::TokenGraphQL;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AddTokens")]
pub struct AddTokensGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
    pub delta_x: TokenGraphQL,
    pub delta_y: TokenGraphQL,
}

impl TryFrom<crate::instructions::postgres::AddTokensRow> for AddTokensGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::AddTokensRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            delta_x: row.delta_x.0.into(),
            delta_y: row.delta_y.0.into(),
        })
    }
}
