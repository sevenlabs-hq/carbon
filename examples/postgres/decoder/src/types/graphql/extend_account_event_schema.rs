use juniper::GraphQLObject;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "ExtendAccountEvent")]
pub struct ExtendAccountEventGraphQL {
        pub timestamp: I64,
        pub account: Pubkey,
        pub user: Pubkey,
        pub current_size: U64,
        pub new_size: U64,
}

impl From<crate::types::ExtendAccountEvent> for ExtendAccountEventGraphQL {
    fn from(original: crate::types::ExtendAccountEvent) -> Self {
        Self {
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
            account: carbon_core::graphql::primitives::Pubkey(original.account),
            user: carbon_core::graphql::primitives::Pubkey(original.user),
            current_size: carbon_core::graphql::primitives::U64(original.current_size),
            new_size: carbon_core::graphql::primitives::U64(original.new_size),
        }
    }
}