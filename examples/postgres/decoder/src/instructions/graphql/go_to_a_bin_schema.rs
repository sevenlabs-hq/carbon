use juniper::GraphQLObject;


#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "GoToABin")]
pub struct GoToABinGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub bin_id: i32,
}

impl TryFrom<crate::instructions::postgres::GoToABinRow> for GoToABinGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::GoToABinRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            bin_id: row.bin_id,
        })
    }
}