use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "LbPairCreate")]
pub struct LbPairCreateGraphQL {
        pub lb_pair: Pubkey,
        pub bin_step: i32,
        pub token_x: Pubkey,
        pub token_y: Pubkey,
}

impl From<crate::types::LbPairCreate> for LbPairCreateGraphQL {
    fn from(original: crate::types::LbPairCreate) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            bin_step: original.bin_step as i32,
            token_x: carbon_core::graphql::primitives::Pubkey(original.token_x),
            token_y: carbon_core::graphql::primitives::Pubkey(original.token_y),
        }
    }
}