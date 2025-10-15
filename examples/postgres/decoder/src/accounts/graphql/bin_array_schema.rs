use juniper::GraphQLObject;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U8;
use crate::types::graphql::BinGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "BinArray")]
pub struct BinArrayGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
        pub index: I64,
            /// Version of binArray
            pub version: U8,
        pub padding: Vec<U8>,
        pub lb_pair: Pubkey,
        pub bins: Vec<BinGraphQL>,
}

impl TryFrom<crate::accounts::postgres::BinArrayRow> for BinArrayGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::accounts::postgres::BinArrayRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            index: carbon_core::graphql::primitives::I64(row.index),
            version: carbon_core::graphql::primitives::U8((*row.version) as u8),
            padding: row.padding.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
            lb_pair: carbon_core::graphql::primitives::Pubkey(*row.lb_pair),
            bins: row.bins.0.into_iter().map(|item| item.into()).collect(),
        })
    }
}