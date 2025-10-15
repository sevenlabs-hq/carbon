use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U32;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "DynamicFeeParameterUpdate")]
pub struct DynamicFeeParameterUpdateGraphQL {
        pub lb_pair: Pubkey,
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
}

impl From<crate::types::DynamicFeeParameterUpdate> for DynamicFeeParameterUpdateGraphQL {
    fn from(original: crate::types::DynamicFeeParameterUpdate) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            filter_period: original.filter_period as i32,
            decay_period: original.decay_period as i32,
            reduction_factor: original.reduction_factor as i32,
            variable_fee_control: carbon_core::graphql::primitives::U32(original.variable_fee_control),
            max_volatility_accumulator: carbon_core::graphql::primitives::U32(original.max_volatility_accumulator),
        }
    }
}