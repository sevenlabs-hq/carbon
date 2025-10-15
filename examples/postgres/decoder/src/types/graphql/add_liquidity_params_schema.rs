use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AddLiquidityParams")]
pub struct AddLiquidityParamsGraphQL {
        pub min_delta_id: i32,
        pub max_delta_id: i32,
        pub x0: U64,
        pub y0: U64,
        pub delta_x: U64,
        pub delta_y: U64,
        pub bit_flag: U8,
        pub favor_x_in_active_id: bool,
        pub padding: Vec<U8>,
}

impl From<crate::types::AddLiquidityParams> for AddLiquidityParamsGraphQL {
    fn from(original: crate::types::AddLiquidityParams) -> Self {
        Self {
            min_delta_id: original.min_delta_id,
            max_delta_id: original.max_delta_id,
            x0: carbon_core::graphql::primitives::U64(original.x0),
            y0: carbon_core::graphql::primitives::U64(original.y0),
            delta_x: carbon_core::graphql::primitives::U64(original.delta_x),
            delta_y: carbon_core::graphql::primitives::U64(original.delta_y),
            bit_flag: carbon_core::graphql::primitives::U8(original.bit_flag),
            favor_x_in_active_id: original.favor_x_in_active_id,
            padding: original.padding.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
        }
    }
}