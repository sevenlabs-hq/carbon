use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "DecreasePositionLength")]
pub struct DecreasePositionLengthGraphQL {
        pub lb_pair: Pubkey,
        pub position: Pubkey,
        pub owner: Pubkey,
        pub length_to_remove: i32,
        pub side: U8,
}

impl From<crate::types::DecreasePositionLength> for DecreasePositionLengthGraphQL {
    fn from(original: crate::types::DecreasePositionLength) -> Self {
        Self {
            lb_pair: carbon_core::graphql::primitives::Pubkey(original.lb_pair),
            position: carbon_core::graphql::primitives::Pubkey(original.position),
            owner: carbon_core::graphql::primitives::Pubkey(original.owner),
            length_to_remove: original.length_to_remove as i32,
            side: carbon_core::graphql::primitives::U8(original.side),
        }
    }
}