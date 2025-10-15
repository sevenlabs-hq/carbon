use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U8;
use crate::types::graphql::AccountsTypeGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "RemainingAccountsSlice")]
pub struct RemainingAccountsSliceGraphQL {
        pub accounts_type: AccountsTypeGraphQL,
        pub length: U8,
}

impl From<crate::types::RemainingAccountsSlice> for RemainingAccountsSliceGraphQL {
    fn from(original: crate::types::RemainingAccountsSlice) -> Self {
        Self {
            accounts_type: original.accounts_type.into(),
            length: carbon_core::graphql::primitives::U8(original.length),
        }
    }
}