use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Sell")]
pub struct SellGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub base_amount_in: U64,
        pub min_quote_amount_out: U64,
}

impl From<crate::instructions::postgres::SellRow> for SellGraphQL {
    fn from(row: crate::instructions::postgres::SellRow) -> Self {
        Self {
            metadata: row.metadata.into(),
            base_amount_in: carbon_core::graphql::primitives::U64(*row.base_amount_in),
            min_quote_amount_out: carbon_core::graphql::primitives::U64(*row.min_quote_amount_out),
        }
    }
}