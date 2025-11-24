use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CreateV1")]
pub struct CreateV1GraphQL {
    pub instruction_metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
}

impl TryFrom<crate::instructions::postgres::create_v1_row::CreateV1Row> for CreateV1GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(
        row: crate::instructions::postgres::create_v1_row::CreateV1Row,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            instruction_metadata: row.instruction_metadata.into(),
        })
    }
}
