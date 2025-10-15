use juniper::GraphQLObject;


#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdateFeesAndReward2")]
pub struct UpdateFeesAndReward2GraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub min_bin_id: i32,
        pub max_bin_id: i32,
}

impl TryFrom<crate::instructions::postgres::UpdateFeesAndReward2Row> for UpdateFeesAndReward2GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::UpdateFeesAndReward2Row) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            min_bin_id: row.min_bin_id,
            max_bin_id: row.max_bin_id,
        })
    }
}