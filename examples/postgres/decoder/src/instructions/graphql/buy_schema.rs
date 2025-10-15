use carbon_core::graphql::primitives::U64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Buy")]
pub struct BuyGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
    pub base_amount_out: U64,
    pub max_quote_amount_in: U64,
    pub track_volume: bool,
}

impl From<crate::instructions::postgres::BuyRow> for BuyGraphQL {
    fn from(row: crate::instructions::postgres::BuyRow) -> Self {
        Self {
            metadata: row.metadata.into(),
            base_amount_out: carbon_core::graphql::primitives::U64(*row.base_amount_out),
            max_quote_amount_in: carbon_core::graphql::primitives::U64(*row.max_quote_amount_in),
            track_volume: row.track_volume.into(),
        }
    }
}
