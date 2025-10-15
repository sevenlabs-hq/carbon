use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SetPairStatus")]
pub struct SetPairStatusGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub status: U8,
}

impl TryFrom<crate::instructions::postgres::SetPairStatusRow> for SetPairStatusGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::SetPairStatusRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            status: carbon_core::graphql::primitives::U8((*row.status) as u8),
        })
    }
}