use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "IncreasePositionLength")]
pub struct IncreasePositionLengthGraphQL {
        pub lb_pair: Pubkey,
        pub position: Pubkey,
        pub owner: Pubkey,
        pub length_to_add: i32,
        pub side: U8,
}

impl From<crate::types::IncreasePositionLength> for IncreasePositionLengthGraphQL {
    fn from(original: crate::types::IncreasePositionLength) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            position: carbon_core::graphql::primitives::Pubkey(original.position),
            owner: carbon_core::graphql::primitives::Pubkey(original.owner),
            length_to_add: original.length_to_add as i32,
            side: carbon_core::graphql::primitives::U8(original.side),
        }
    }
}