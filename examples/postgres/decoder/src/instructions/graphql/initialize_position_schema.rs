use juniper::GraphQLObject;


#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "InitializePosition")]
pub struct InitializePositionGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub lower_bin_id: i32,
        pub width: i32,
}

impl TryFrom<crate::instructions::postgres::InitializePositionRow> for InitializePositionGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::InitializePositionRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            lower_bin_id: row.lower_bin_id,
            width: row.width,
        })
    }
}