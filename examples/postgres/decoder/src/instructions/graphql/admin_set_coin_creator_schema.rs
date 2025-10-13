use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;

/// Overrides the coin creator for a canonical pump pool
#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AdminSetCoinCreator")]
pub struct AdminSetCoinCreatorGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub coin_creator: Pubkey,
}

impl From<crate::instructions::postgres::AdminSetCoinCreatorRow> for AdminSetCoinCreatorGraphQL {
    fn from(row: crate::instructions::postgres::AdminSetCoinCreatorRow) -> Self {
        Self {
            metadata: row.metadata.into(),
            coin_creator: carbon_core::graphql::primitives::Pubkey(*row.coin_creator),
        }
    }
}