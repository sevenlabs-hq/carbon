use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AddAuthorityV1")]
pub struct AddAuthorityV1GraphQL {
    pub instruction_metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
}

impl TryFrom<crate::instructions::postgres::add_authority_v1_row::AddAuthorityV1Row>
    for AddAuthorityV1GraphQL
{
    type Error = carbon_core::error::Error;
    fn try_from(
        row: crate::instructions::postgres::add_authority_v1_row::AddAuthorityV1Row,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            instruction_metadata: row.instruction_metadata.into(),
        })
    }
}
