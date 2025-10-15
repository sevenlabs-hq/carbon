use juniper::GraphQLObject;


#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "InitializePositionPda")]
pub struct InitializePositionPdaGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub lower_bin_id: i32,
        pub width: i32,
}

impl TryFrom<crate::instructions::postgres::InitializePositionPdaRow> for InitializePositionPdaGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::InitializePositionPdaRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            lower_bin_id: row.lower_bin_id,
            width: row.width,
        })
    }
}