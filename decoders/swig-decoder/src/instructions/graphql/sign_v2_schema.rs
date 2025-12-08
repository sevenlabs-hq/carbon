use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SignV2")]
pub struct SignV2GraphQL {
    pub instruction_metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
}

impl TryFrom<crate::instructions::postgres::sign_v2_row::SignV2Row> for SignV2GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(
        row: crate::instructions::postgres::sign_v2_row::SignV2Row,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            instruction_metadata: row.instruction_metadata.into(),
        })
    }
}
