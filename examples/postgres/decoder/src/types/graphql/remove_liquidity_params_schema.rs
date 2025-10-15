use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "RemoveLiquidityParams")]
pub struct RemoveLiquidityParamsGraphQL {
        pub min_bin_id: Option<i32>,
        pub max_bin_id: Option<i32>,
        pub bps: i32,
        pub padding: Vec<U8>,
}

impl From<crate::types::RemoveLiquidityParams> for RemoveLiquidityParamsGraphQL {
    fn from(original: crate::types::RemoveLiquidityParams) -> Self {
        Self {
            min_bin_id: original.min_bin_id.map(|v| v),
            max_bin_id: original.max_bin_id.map(|v| v),
            bps: original.bps as i32,
            padding: original.padding.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
        }
    }
}