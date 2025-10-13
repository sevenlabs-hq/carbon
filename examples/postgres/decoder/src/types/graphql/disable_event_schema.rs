use juniper::GraphQLObject;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::Pubkey;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "DisableEvent")]
pub struct DisableEventGraphQL {
        pub timestamp: I64,
        pub admin: Pubkey,
        pub disable_create_pool: bool,
        pub disable_deposit: bool,
        pub disable_withdraw: bool,
        pub disable_buy: bool,
        pub disable_sell: bool,
}

impl From<crate::types::DisableEvent> for DisableEventGraphQL {
    fn from(original: crate::types::DisableEvent) -> Self {
        Self {
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
            admin: carbon_core::graphql::primitives::Pubkey(original.admin),
            disable_create_pool: original.disable_create_pool,
            disable_deposit: original.disable_deposit,
            disable_withdraw: original.disable_withdraw,
            disable_buy: original.disable_buy,
            disable_sell: original.disable_sell,
        }
    }
}