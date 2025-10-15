use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U32;
use carbon_core::graphql::primitives::U8;

/// Parameter that set by the protocol
#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "StaticParameters")]
pub struct StaticParametersGraphQL {
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
            /// Base fee power factor
            pub base_fee_power_factor: U8,
            /// Padding for bytemuck safe alignment
            pub padding: Vec<U8>,
}

impl From<crate::types::StaticParameters> for StaticParametersGraphQL {
    fn from(original: crate::types::StaticParameters) -> Self {
        Self {
            base_factor: original.base_factor as i32,
            filter_period: original.filter_period as i32,
            decay_period: original.decay_period as i32,
            reduction_factor: original.reduction_factor as i32,
            variable_fee_control: carbon_core::graphql::primitives::U32(original.variable_fee_control),
            max_volatility_accumulator: carbon_core::graphql::primitives::U32(original.max_volatility_accumulator),
            min_bin_id: original.min_bin_id,
            max_bin_id: original.max_bin_id,
            protocol_share: original.protocol_share as i32,
            base_fee_power_factor: carbon_core::graphql::primitives::U8(original.base_fee_power_factor),
            padding: original.padding.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
        }
    }
}