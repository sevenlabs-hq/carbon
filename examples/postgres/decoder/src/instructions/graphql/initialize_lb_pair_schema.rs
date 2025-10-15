use juniper::GraphQLObject;


#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "InitializeLbPair")]
pub struct InitializeLbPairGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub active_id: i32,
        pub bin_step: i32,
}

impl TryFrom<crate::instructions::postgres::InitializeLbPairRow> for InitializeLbPairGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::InitializeLbPairRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            active_id: row.active_id,
            bin_step: (*row.bin_step) as i32,
        })
    }
}