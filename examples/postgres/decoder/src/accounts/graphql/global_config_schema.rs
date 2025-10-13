use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "GlobalConfig")]
pub struct GlobalConfigGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
            /// The admin pubkey
            pub admin: Pubkey,
        pub lp_fee_basis_points: U64,
        pub protocol_fee_basis_points: U64,
            /// Flags to disable certain functionality
        /// bit 0 - Disable create pool
        /// bit 1 - Disable deposit
        /// bit 2 - Disable withdraw
        /// bit 3 - Disable buy
        /// bit 4 - Disable sell
            pub disable_flags: U8,
            /// Addresses of the protocol fee recipients
            pub protocol_fee_recipients: Vec<Pubkey>,
        pub coin_creator_fee_basis_points: U64,
            /// The admin authority for setting coin creators
            pub admin_set_coin_creator_authority: Pubkey,
}

impl From<crate::accounts::postgres::GlobalConfigRow> for GlobalConfigGraphQL {
    fn from(row: crate::accounts::postgres::GlobalConfigRow) -> Self {
        Self {
            metadata: row.metadata.into(),
            admin: carbon_core::graphql::primitives::Pubkey(*row.admin),
            lp_fee_basis_points: carbon_core::graphql::primitives::U64(*row.lp_fee_basis_points),
            protocol_fee_basis_points: carbon_core::graphql::primitives::U64(*row.protocol_fee_basis_points),
            disable_flags: carbon_core::graphql::primitives::U8((*row.disable_flags) as u8),
            protocol_fee_recipients: row.protocol_fee_recipients.into_iter().map(|item| carbon_core::graphql::primitives::Pubkey(*item)).collect(),
            coin_creator_fee_basis_points: carbon_core::graphql::primitives::U64(*row.coin_creator_fee_basis_points),
            admin_set_coin_creator_authority: carbon_core::graphql::primitives::Pubkey(*row.admin_set_coin_creator_authority),
        }
    }
}