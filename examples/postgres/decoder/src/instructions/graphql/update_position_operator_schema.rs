use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdatePositionOperator")]
pub struct UpdatePositionOperatorGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub operator: Pubkey,
}

impl TryFrom<crate::instructions::postgres::UpdatePositionOperatorRow> for UpdatePositionOperatorGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::UpdatePositionOperatorRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            operator: carbon_core::graphql::primitives::Pubkey(*row.operator),
        })
    }
}