use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "BinArrayBitmapExtension")]
pub struct BinArrayBitmapExtensionGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
        pub lb_pair: Pubkey,
            /// Packed initialized bin array state for start_bin_index is positive
            pub positive_bin_array_bitmap: Vec<Vec<U64>>,
            /// Packed initialized bin array state for start_bin_index is negative
            pub negative_bin_array_bitmap: Vec<Vec<U64>>,
}

impl TryFrom<crate::accounts::postgres::BinArrayBitmapExtensionRow> for BinArrayBitmapExtensionGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::accounts::postgres::BinArrayBitmapExtensionRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            lb_pair: carbon_core::graphql::primitives::Pubkey(*row.lb_pair),
            positive_bin_array_bitmap: row.positive_bin_array_bitmap.0.into_iter().map(|item| item.into_iter().map(|item| carbon_core::graphql::primitives::U64(*item)).collect()).collect(),
            negative_bin_array_bitmap: row.negative_bin_array_bitmap.0.into_iter().map(|item| item.into_iter().map(|item| carbon_core::graphql::primitives::U64(*item)).collect()).collect(),
        })
    }
}