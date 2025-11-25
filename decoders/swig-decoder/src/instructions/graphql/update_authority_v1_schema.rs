use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdateAuthorityV1")]
pub struct UpdateAuthorityV1GraphQL {
    pub instruction_metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
}

impl TryFrom<crate::instructions::postgres::update_authority_v1_row::UpdateAuthorityV1Row>
    for UpdateAuthorityV1GraphQL
{
    type Error = carbon_core::error::Error;
    fn try_from(
        row: crate::instructions::postgres::update_authority_v1_row::UpdateAuthorityV1Row,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            instruction_metadata: row.instruction_metadata.into(),
        })
    }
}
