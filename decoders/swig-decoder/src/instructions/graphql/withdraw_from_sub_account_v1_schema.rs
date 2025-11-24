use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "WithdrawFromSubAccountV1")]
pub struct WithdrawFromSubAccountV1GraphQL {
    pub instruction_metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
}

impl TryFrom<crate::instructions::postgres::withdraw_from_sub_account_v1_row::WithdrawFromSubAccountV1Row> for WithdrawFromSubAccountV1GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::withdraw_from_sub_account_v1_row::WithdrawFromSubAccountV1Row) -> Result<Self, Self::Error> {
        Ok(Self {
            instruction_metadata: row.instruction_metadata.into(),
        })
    }
}
