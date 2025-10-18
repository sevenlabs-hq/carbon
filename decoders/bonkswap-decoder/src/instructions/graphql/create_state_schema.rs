use carbon_core::graphql::primitives::U8;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CreateState")]
pub struct CreateStateGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
    pub nonce: U8,
}

impl TryFrom<crate::instructions::postgres::CreateStateRow> for CreateStateGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::CreateStateRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            nonce: carbon_core::graphql::primitives::U8((*row.nonce) as u8),
        })
    }
}
