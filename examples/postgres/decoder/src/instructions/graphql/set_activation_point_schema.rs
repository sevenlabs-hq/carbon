use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SetActivationPoint")]
pub struct SetActivationPointGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub activation_point: U64,
}

impl TryFrom<crate::instructions::postgres::SetActivationPointRow> for SetActivationPointGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::SetActivationPointRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            activation_point: carbon_core::graphql::primitives::U64(*row.activation_point),
        })
    }
}