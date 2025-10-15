use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "GoToABin")]
pub struct GoToABinGraphQL {
        pub lb_pair: Pubkey,
        pub from_bin_id: i32,
        pub to_bin_id: i32,
}

impl From<crate::types::GoToABin> for GoToABinGraphQL {
    fn from(original: crate::types::GoToABin) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            from_bin_id: original.from_bin_id,
            to_bin_id: original.to_bin_id,
        }
    }
}