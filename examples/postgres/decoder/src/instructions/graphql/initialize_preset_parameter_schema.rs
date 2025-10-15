use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U32;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "InitializePresetParameter")]
pub struct InitializePresetParameterGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub bin_step: i32,
        pub base_factor: i32,
        pub filter_period: i32,
        pub decay_period: i32,
        pub reduction_factor: i32,
        pub variable_fee_control: U32,
        pub max_volatility_accumulator: U32,
        pub protocol_share: i32,
}

impl TryFrom<crate::instructions::postgres::InitializePresetParameterRow> for InitializePresetParameterGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::InitializePresetParameterRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            bin_step: (*row.bin_step) as i32,
            base_factor: (*row.base_factor) as i32,
            filter_period: (*row.filter_period) as i32,
            decay_period: (*row.decay_period) as i32,
            reduction_factor: (*row.reduction_factor) as i32,
            variable_fee_control: carbon_core::graphql::primitives::U32((*row.variable_fee_control) as u32),
            max_volatility_accumulator: carbon_core::graphql::primitives::U32((*row.max_volatility_accumulator) as u32),
            protocol_share: (*row.protocol_share) as i32,
        })
    }
}