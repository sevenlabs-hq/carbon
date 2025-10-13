use juniper::GraphQLObject;
use carbon_core::graphql::primitives::I128;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U32;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "TickState")]
pub struct TickStateGraphQL {
        pub tick: i32,
        pub liquidity_net: I128,
        pub liquidity_gross: U128,
        pub fee_growth_outside0_x64: U128,
        pub fee_growth_outside1_x64: U128,
        pub reward_growths_outside_x64: Vec<U128>,
        pub padding: Vec<U32>,
}

impl From<crate::types::TickState> for TickStateGraphQL {
    fn from(original: crate::types::TickState) -> Self {
        Self {
            tick: original.tick,
            liquidity_net: carbon_core::graphql::primitives::I128(original.liquidity_net),
            liquidity_gross: carbon_core::graphql::primitives::U128(original.liquidity_gross),
            fee_growth_outside0_x64: carbon_core::graphql::primitives::U128(original.fee_growth_outside0_x64),
            fee_growth_outside1_x64: carbon_core::graphql::primitives::U128(original.fee_growth_outside1_x64),
            reward_growths_outside_x64: original.reward_growths_outside_x64.into_iter().map(|item| carbon_core::graphql::primitives::U128(item)).collect(),
            padding: original.padding.into_iter().map(|item| carbon_core::graphql::primitives::U32(item)).collect(),
        }
    }
}