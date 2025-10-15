use juniper::GraphQLObject;
use crate::types::graphql::CustomizableParamsGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "InitializeCustomizablePermissionlessLbPair")]
pub struct InitializeCustomizablePermissionlessLbPairGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub params: CustomizableParamsGraphQL,
}

impl TryFrom<crate::instructions::postgres::InitializeCustomizablePermissionlessLbPairRow> for InitializeCustomizablePermissionlessLbPairGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::InitializeCustomizablePermissionlessLbPairRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            params: row.params.0.into(),
        })
    }
}