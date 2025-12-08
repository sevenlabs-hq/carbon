use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SignV1")]
pub struct SignV1GraphQL {
    pub instruction_metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
}

impl TryFrom<crate::instructions::postgres::sign_v1_row::SignV1Row> for SignV1GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(
        row: crate::instructions::postgres::sign_v1_row::SignV1Row,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            instruction_metadata: row.instruction_metadata.into(),
        })
    }
}
