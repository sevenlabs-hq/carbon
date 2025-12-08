use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "ToggleSubAccountV1")]
pub struct ToggleSubAccountV1GraphQL {
    pub instruction_metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
}

impl TryFrom<crate::instructions::postgres::toggle_sub_account_v1_row::ToggleSubAccountV1Row>
    for ToggleSubAccountV1GraphQL
{
    type Error = carbon_core::error::Error;
    fn try_from(
        row: crate::instructions::postgres::toggle_sub_account_v1_row::ToggleSubAccountV1Row,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            instruction_metadata: row.instruction_metadata.into(),
        })
    }
}
