use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U128;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Bin")]
pub struct BinGraphQL {
            /// Amount of token X in the bin. This already excluded protocol fees.
            pub amount_x: U64,
            /// Amount of token Y in the bin. This already excluded protocol fees.
            pub amount_y: U64,
            /// Bin price
            pub price: U128,
            /// Liquidities of the bin. This is the same as LP mint supply. q-number
            pub liquidity_supply: U128,
            /// reward_a_per_token_stored
            pub reward_per_token_stored: Vec<U128>,
            /// Swap fee amount of token X per liquidity deposited.
            pub fee_amount_x_per_token_stored: U128,
            /// Swap fee amount of token Y per liquidity deposited.
            pub fee_amount_y_per_token_stored: U128,
            /// Total token X swap into the bin. Only used for tracking purpose.
            pub amount_x_in: U128,
            /// Total token Y swap into he bin. Only used for tracking purpose.
            pub amount_y_in: U128,
}

impl From<crate::types::Bin> for BinGraphQL {
    fn from(original: crate::types::Bin) -> Self {
        Self {
            amount_x: carbon_core::graphql::primitives::U64(original.amount_x),
            amount_y: carbon_core::graphql::primitives::U64(original.amount_y),
            price: carbon_core::graphql::primitives::U128(original.price),
            liquidity_supply: carbon_core::graphql::primitives::U128(original.liquidity_supply),
            reward_per_token_stored: original.reward_per_token_stored.into_iter().map(|item| carbon_core::graphql::primitives::U128(item)).collect(),
            fee_amount_x_per_token_stored: carbon_core::graphql::primitives::U128(original.fee_amount_x_per_token_stored),
            fee_amount_y_per_token_stored: carbon_core::graphql::primitives::U128(original.fee_amount_y_per_token_stored),
            amount_x_in: carbon_core::graphql::primitives::U128(original.amount_x_in),
            amount_y_in: carbon_core::graphql::primitives::U128(original.amount_y_in),
        }
    }
}