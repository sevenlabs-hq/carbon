use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SetPreActivationDuration")]
pub struct SetPreActivationDurationGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub pre_activation_duration: U64,
}

impl TryFrom<crate::instructions::postgres::SetPreActivationDurationRow> for SetPreActivationDurationGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::SetPreActivationDurationRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            pre_activation_duration: carbon_core::graphql::primitives::U64(*row.pre_activation_duration),
        })
    }
}