use juniper::GraphQLObject;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::U32;
use carbon_core::graphql::primitives::U8;

/// Parameters that changes based on dynamic of the market
#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "VariableParameters")]
pub struct VariableParametersGraphQL {
            /// Volatility accumulator measure the number of bin crossed since reference bin ID. Normally (without filter period taken into consideration), reference bin ID is the active bin of last swap.
        /// It affects the variable fee rate
            pub volatility_accumulator: U32,
            /// Volatility reference is decayed volatility accumulator. It is always <= volatility_accumulator
            pub volatility_reference: U32,
            /// Active bin id of last swap.
            pub index_reference: i32,
            /// Padding for bytemuck safe alignment
            pub padding: Vec<U8>,
            /// Last timestamp the variable parameters was updated
            pub last_update_timestamp: I64,
            /// Padding for bytemuck safe alignment
            pub padding1: Vec<U8>,
}

impl From<crate::types::VariableParameters> for VariableParametersGraphQL {
    fn from(original: crate::types::VariableParameters) -> Self {
        Self {
            volatility_accumulator: carbon_core::graphql::primitives::U32(original.volatility_accumulator),
            volatility_reference: carbon_core::graphql::primitives::U32(original.volatility_reference),
            index_reference: original.index_reference,
            padding: original.padding.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
            last_update_timestamp: carbon_core::graphql::primitives::I64(original.last_update_timestamp),
            padding1: original.padding1.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
        }
    }
}