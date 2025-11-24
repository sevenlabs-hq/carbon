use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SubAccountSignV1")]
pub struct SubAccountSignV1GraphQL {
    pub instruction_metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
}

impl TryFrom<crate::instructions::postgres::sub_account_sign_v1_row::SubAccountSignV1Row>
    for SubAccountSignV1GraphQL
{
    type Error = carbon_core::error::Error;
    fn try_from(
        row: crate::instructions::postgres::sub_account_sign_v1_row::SubAccountSignV1Row,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            instruction_metadata: row.instruction_metadata.into(),
        })
    }
}
