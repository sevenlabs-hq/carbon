use juniper::GraphQLObject;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::Pubkey;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdateAdminEvent")]
pub struct UpdateAdminEventGraphQL {
        pub timestamp: I64,
        pub admin: Pubkey,
        pub new_admin: Pubkey,
}

impl From<crate::types::UpdateAdminEvent> for UpdateAdminEventGraphQL {
    fn from(original: crate::types::UpdateAdminEvent) -> Self {
        Self {
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
            admin: carbon_core::graphql::primitives::Pubkey(original.admin),
            new_admin: carbon_core::graphql::primitives::Pubkey(original.new_admin),
        }
    }
}