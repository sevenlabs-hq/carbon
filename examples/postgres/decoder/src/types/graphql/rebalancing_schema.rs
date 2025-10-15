use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Rebalancing")]
pub struct RebalancingGraphQL {
        pub lb_pair: Pubkey,
        pub position: Pubkey,
        pub x_withdrawn_amount: U64,
        pub x_added_amount: U64,
        pub y_withdrawn_amount: U64,
        pub y_added_amount: U64,
        pub x_fee_amount: U64,
        pub y_fee_amount: U64,
        pub old_min_id: i32,
        pub old_max_id: i32,
        pub new_min_id: i32,
        pub new_max_id: i32,
        pub rewards: Vec<U64>,
}

impl From<crate::types::Rebalancing> for RebalancingGraphQL {
    fn from(original: crate::types::Rebalancing) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            position: carbon_core::graphql::primitives::Pubkey(original.position),
            x_withdrawn_amount: carbon_core::graphql::primitives::U64(original.x_withdrawn_amount),
            x_added_amount: carbon_core::graphql::primitives::U64(original.x_added_amount),
            y_withdrawn_amount: carbon_core::graphql::primitives::U64(original.y_withdrawn_amount),
            y_added_amount: carbon_core::graphql::primitives::U64(original.y_added_amount),
            x_fee_amount: carbon_core::graphql::primitives::U64(original.x_fee_amount),
            y_fee_amount: carbon_core::graphql::primitives::U64(original.y_fee_amount),
            old_min_id: original.old_min_id,
            old_max_id: original.old_max_id,
            new_min_id: original.new_min_id,
            new_max_id: original.new_max_id,
            rewards: original.rewards.into_iter().map(|item| carbon_core::graphql::primitives::U64(item)).collect(),
        }
    }
}