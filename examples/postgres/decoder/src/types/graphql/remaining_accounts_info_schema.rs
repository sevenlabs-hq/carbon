use juniper::GraphQLObject;
use crate::types::graphql::RemainingAccountsSliceGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "RemainingAccountsInfo")]
pub struct RemainingAccountsInfoGraphQL {
        pub slices: Vec<RemainingAccountsSliceGraphQL>,
}

impl From<crate::types::RemainingAccountsInfo> for RemainingAccountsInfoGraphQL {
    fn from(original: crate::types::RemainingAccountsInfo) -> Self {
        Self {
            slices: original.slices.into_iter().map(|item| item.into()).collect(),
        }
    }
}