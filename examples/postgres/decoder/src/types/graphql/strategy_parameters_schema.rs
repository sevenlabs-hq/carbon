use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U8;
use crate::types::graphql::StrategyTypeGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "StrategyParameters")]
pub struct StrategyParametersGraphQL {
            /// min bin id
            pub min_bin_id: i32,
            /// max bin id
            pub max_bin_id: i32,
            /// strategy type
            pub strategy_type: StrategyTypeGraphQL,
            /// parameters
            pub parameteres: Vec<U8>,
}

impl From<crate::types::StrategyParameters> for StrategyParametersGraphQL {
    fn from(original: crate::types::StrategyParameters) -> Self {
        Self {
            min_bin_id: original.min_bin_id,
            max_bin_id: original.max_bin_id,
            strategy_type: original.strategy_type.into(),
            parameteres: original.parameteres.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
        }
    }
}