use juniper::GraphQLObject;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AdminUpdateTokenIncentivesEvent")]
pub struct AdminUpdateTokenIncentivesEventGraphQL {
        pub start_time: I64,
        pub end_time: I64,
        pub day_number: U64,
        pub token_supply_per_day: U64,
        pub mint: Pubkey,
        pub seconds_in_a_day: I64,
        pub timestamp: I64,
}

impl From<crate::types::AdminUpdateTokenIncentivesEvent> for AdminUpdateTokenIncentivesEventGraphQL {
    fn from(original: crate::types::AdminUpdateTokenIncentivesEvent) -> Self {
        Self {
            start_time: carbon_core::graphql::primitives::I64(original.start_time),
            end_time: carbon_core::graphql::primitives::I64(original.end_time),
            day_number: carbon_core::graphql::primitives::U64(original.day_number),
            token_supply_per_day: carbon_core::graphql::primitives::U64(original.token_supply_per_day),
            mint: carbon_core::graphql::primitives::Pubkey(original.mint),
            seconds_in_a_day: carbon_core::graphql::primitives::I64(original.seconds_in_a_day),
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
        }
    }
}