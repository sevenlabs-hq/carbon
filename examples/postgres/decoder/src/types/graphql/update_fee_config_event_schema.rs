use juniper::GraphQLObject;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdateFeeConfigEvent")]
pub struct UpdateFeeConfigEventGraphQL {
        pub timestamp: I64,
        pub admin: Pubkey,
        pub lp_fee_basis_points: U64,
        pub protocol_fee_basis_points: U64,
        pub protocol_fee_recipients: Vec<Pubkey>,
        pub coin_creator_fee_basis_points: U64,
        pub admin_set_coin_creator_authority: Pubkey,
}

impl From<crate::types::UpdateFeeConfigEvent> for UpdateFeeConfigEventGraphQL {
    fn from(original: crate::types::UpdateFeeConfigEvent) -> Self {
        Self {
            timestamp: carbon_core::graphql::primitives::I64(original.timestamp),
            admin: carbon_core::graphql::primitives::Pubkey(original.admin),
            lp_fee_basis_points: carbon_core::graphql::primitives::U64(original.lp_fee_basis_points),
            protocol_fee_basis_points: carbon_core::graphql::primitives::U64(original.protocol_fee_basis_points),
            protocol_fee_recipients: original.protocol_fee_recipients.into_iter().map(|item| carbon_core::graphql::primitives::Pubkey(item)).collect(),
            coin_creator_fee_basis_points: carbon_core::graphql::primitives::U64(original.coin_creator_fee_basis_points),
            admin_set_coin_creator_authority: carbon_core::graphql::primitives::Pubkey(original.admin_set_coin_creator_authority),
        }
    }
}