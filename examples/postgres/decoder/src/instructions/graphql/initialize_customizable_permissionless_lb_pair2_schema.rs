use juniper::GraphQLObject;
use crate::types::graphql::CustomizableParamsGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "InitializeCustomizablePermissionlessLbPair2")]
pub struct InitializeCustomizablePermissionlessLbPair2GraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub params: CustomizableParamsGraphQL,
}

impl TryFrom<crate::instructions::postgres::InitializeCustomizablePermissionlessLbPair2Row> for InitializeCustomizablePermissionlessLbPair2GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::InitializeCustomizablePermissionlessLbPair2Row) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            params: row.params.0.into(),
        })
    }
}