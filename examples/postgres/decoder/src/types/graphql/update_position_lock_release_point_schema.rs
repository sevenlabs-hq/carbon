use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdatePositionLockReleasePoint")]
pub struct UpdatePositionLockReleasePointGraphQL {
        pub position: Pubkey,
        pub current_point: U64,
        pub new_lock_release_point: U64,
        pub old_lock_release_point: U64,
        pub sender: Pubkey,
}

impl From<crate::types::UpdatePositionLockReleasePoint> for UpdatePositionLockReleasePointGraphQL {
    fn from(original: crate::types::UpdatePositionLockReleasePoint) -> Self {
        Self {
            position: carbon_core::graphql::primitives::Pubkey(original.position),
            current_point: carbon_core::graphql::primitives::U64(original.current_point),
            new_lock_release_point: carbon_core::graphql::primitives::U64(original.new_lock_release_point),
            old_lock_release_point: carbon_core::graphql::primitives::U64(original.old_lock_release_point),
            sender: carbon_core::graphql::primitives::Pubkey(original.sender),
        }
    }
}