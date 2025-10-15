use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "TokenBadge")]
pub struct TokenBadgeGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
            /// token mint
            pub token_mint: Pubkey,
            /// Reserve
            pub padding: Vec<U8>,
}

impl TryFrom<crate::accounts::postgres::TokenBadgeRow> for TokenBadgeGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::accounts::postgres::TokenBadgeRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            token_mint: carbon_core::graphql::primitives::Pubkey(*row.token_mint),
            padding: row.padding.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
        })
    }
}