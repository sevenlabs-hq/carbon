use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "InitializeLbPair2")]
pub struct InitializeLbPair2GraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub active_id: i32,
        pub padding: Vec<U8>,
}

impl TryFrom<crate::instructions::postgres::InitializeLbPair2Row> for InitializeLbPair2GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::InitializeLbPair2Row) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            active_id: row.active_id,
            padding: row.padding.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
        })
    }
}