use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;
use crate::types::graphql::StrategyParametersGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "LiquidityParameterByStrategy")]
pub struct LiquidityParameterByStrategyGraphQL {
            /// Amount of X token to deposit
            pub amount_x: U64,
            /// Amount of Y token to deposit
            pub amount_y: U64,
            /// Active bin that integrator observe off-chain
            pub active_id: i32,
            /// max active bin slippage allowed
            pub max_active_bin_slippage: i32,
            /// strategy parameters
            pub strategy_parameters: StrategyParametersGraphQL,
}

impl From<crate::types::LiquidityParameterByStrategy> for LiquidityParameterByStrategyGraphQL {
    fn from(original: crate::types::LiquidityParameterByStrategy) -> Self {
        Self {
            amount_x: carbon_core::graphql::primitives::U64(original.amount_x),
            amount_y: carbon_core::graphql::primitives::U64(original.amount_y),
            active_id: original.active_id,
            max_active_bin_slippage: original.max_active_bin_slippage,
            strategy_parameters: original.strategy_parameters.into(),
        }
    }
}