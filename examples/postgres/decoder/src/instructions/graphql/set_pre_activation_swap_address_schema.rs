use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SetPreActivationSwapAddress")]
pub struct SetPreActivationSwapAddressGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub pre_activation_swap_address: Pubkey,
}

impl TryFrom<crate::instructions::postgres::SetPreActivationSwapAddressRow> for SetPreActivationSwapAddressGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::SetPreActivationSwapAddressRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            pre_activation_swap_address: carbon_core::graphql::primitives::Pubkey(*row.pre_activation_swap_address),
        })
    }
}