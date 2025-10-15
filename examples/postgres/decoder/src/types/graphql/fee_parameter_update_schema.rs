use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "FeeParameterUpdate")]
pub struct FeeParameterUpdateGraphQL {
        pub lb_pair: Pubkey,
        pub protocol_share: i32,
        pub base_factor: i32,
}

impl From<crate::types::FeeParameterUpdate> for FeeParameterUpdateGraphQL {
    fn from(original: crate::types::FeeParameterUpdate) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            protocol_share: original.protocol_share as i32,
            base_factor: original.base_factor as i32,
        }
    }
}