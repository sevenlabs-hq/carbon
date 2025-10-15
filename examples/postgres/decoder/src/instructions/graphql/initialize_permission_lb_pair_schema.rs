use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "InitializePermissionLbPair")]
pub struct InitializePermissionLbPairGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub active_id: i32,
        pub bin_step: i32,
        pub base_factor: i32,
        pub base_fee_power_factor: U8,
        pub activation_type: U8,
        pub protocol_share: i32,
}

impl TryFrom<crate::instructions::postgres::InitializePermissionLbPairRow> for InitializePermissionLbPairGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::InitializePermissionLbPairRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            active_id: row.active_id,
            bin_step: (*row.bin_step) as i32,
            base_factor: (*row.base_factor) as i32,
            base_fee_power_factor: carbon_core::graphql::primitives::U8((*row.base_fee_power_factor) as u8),
            activation_type: carbon_core::graphql::primitives::U8((*row.activation_type) as u8),
            protocol_share: (*row.protocol_share) as i32,
        })
    }
}