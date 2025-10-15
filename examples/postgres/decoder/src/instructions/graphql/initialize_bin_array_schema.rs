use juniper::GraphQLObject;
use carbon_core::graphql::primitives::I64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "InitializeBinArray")]
pub struct InitializeBinArrayGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub index: I64,
}

impl TryFrom<crate::instructions::postgres::InitializeBinArrayRow> for InitializeBinArrayGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::InitializeBinArrayRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            index: carbon_core::graphql::primitives::I64(row.index),
        })
    }
}