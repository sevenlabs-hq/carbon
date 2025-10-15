use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U32;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "PresetParameter")]
pub struct PresetParameterGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
            /// Bin step. Represent the price increment / decrement.
            pub bin_step: i32,
            /// Used for base fee calculation. base_fee_rate = base_factor * bin_step * 10 * 10^base_fee_power_factor
            pub base_factor: i32,
            /// Filter period determine high frequency trading time window.
            pub filter_period: i32,
            /// Decay period determine when the volatile fee start decay / decrease.
            pub decay_period: i32,
            /// Reduction factor controls the volatile fee rate decrement rate.
            pub reduction_factor: i32,
            /// Used to scale the variable fee component depending on the dynamic of the market
            pub variable_fee_control: U32,
            /// Maximum number of bin crossed can be accumulated. Used to cap volatile fee rate.
            pub max_volatility_accumulator: U32,
            /// Min bin id supported by the pool based on the configured bin step.
            pub min_bin_id: i32,
            /// Max bin id supported by the pool based on the configured bin step.
            pub max_bin_id: i32,
            /// Portion of swap fees retained by the protocol by controlling protocol_share parameter. protocol_swap_fee = protocol_share * total_swap_fee
            pub protocol_share: i32,
}

impl TryFrom<crate::accounts::postgres::PresetParameterRow> for PresetParameterGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::accounts::postgres::PresetParameterRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            bin_step: (*row.bin_step) as i32,
            base_factor: (*row.base_factor) as i32,
            filter_period: (*row.filter_period) as i32,
            decay_period: (*row.decay_period) as i32,
            reduction_factor: (*row.reduction_factor) as i32,
            variable_fee_control: carbon_core::graphql::primitives::U32((*row.variable_fee_control) as u32),
            max_volatility_accumulator: carbon_core::graphql::primitives::U32((*row.max_volatility_accumulator) as u32),
            min_bin_id: row.min_bin_id,
            max_bin_id: row.max_bin_id,
            protocol_share: (*row.protocol_share) as i32,
        })
    }
}