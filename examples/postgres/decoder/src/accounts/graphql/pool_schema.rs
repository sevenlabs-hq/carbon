use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Pool")]
pub struct PoolGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
        pub pool_bump: U8,
        pub index: i32,
        pub creator: Pubkey,
        pub base_mint: Pubkey,
        pub quote_mint: Pubkey,
        pub lp_mint: Pubkey,
        pub pool_base_token_account: Pubkey,
        pub pool_quote_token_account: Pubkey,
            /// True circulating supply without burns and lock-ups
            pub lp_supply: U64,
        pub coin_creator: Pubkey,
}

impl From<crate::accounts::postgres::PoolRow> for PoolGraphQL {
    fn from(row: crate::accounts::postgres::PoolRow) -> Self {
        Self {
            metadata: row.metadata.into(),
            pool_bump: carbon_core::graphql::primitives::U8((*row.pool_bump) as u8),
            index: (*row.index) as i32,
            creator: carbon_core::graphql::primitives::Pubkey(*row.creator),
            base_mint: carbon_core::graphql::primitives::Pubkey(*row.base_mint),
            quote_mint: carbon_core::graphql::primitives::Pubkey(*row.quote_mint),
            lp_mint: carbon_core::graphql::primitives::Pubkey(*row.lp_mint),
            pool_base_token_account: carbon_core::graphql::primitives::Pubkey(*row.pool_base_token_account),
            pool_quote_token_account: carbon_core::graphql::primitives::Pubkey(*row.pool_quote_token_account),
            lp_supply: carbon_core::graphql::primitives::U64(*row.lp_supply),
            coin_creator: carbon_core::graphql::primitives::Pubkey(*row.coin_creator),
        }
    }
}