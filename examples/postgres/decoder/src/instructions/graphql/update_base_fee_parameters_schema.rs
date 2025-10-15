use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdateBaseFeeParameters")]
pub struct UpdateBaseFeeParametersGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub protocol_share: i32,
        pub base_factor: i32,
        pub base_fee_power_factor: U8,
}

impl TryFrom<crate::instructions::postgres::UpdateBaseFeeParametersRow> for UpdateBaseFeeParametersGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::UpdateBaseFeeParametersRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            protocol_share: (*row.protocol_share) as i32,
            base_factor: (*row.base_factor) as i32,
            base_fee_power_factor: carbon_core::graphql::primitives::U8((*row.base_fee_power_factor) as u8),
        })
    }
}