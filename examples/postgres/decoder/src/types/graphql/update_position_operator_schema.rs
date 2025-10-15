use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdatePositionOperator")]
pub struct UpdatePositionOperatorGraphQL {
        pub position: Pubkey,
        pub old_operator: Pubkey,
        pub new_operator: Pubkey,
}

impl From<crate::types::UpdatePositionOperator> for UpdatePositionOperatorGraphQL {
    fn from(original: crate::types::UpdatePositionOperator) -> Self {
        Self {
            position: carbon_core::graphql::primitives::Pubkey(original.position),
            old_operator: carbon_core::graphql::primitives::Pubkey(original.old_operator),
            new_operator: carbon_core::graphql::primitives::Pubkey(original.new_operator),
        }
    }
}