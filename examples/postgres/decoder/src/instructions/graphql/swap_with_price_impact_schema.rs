use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SwapWithPriceImpact")]
pub struct SwapWithPriceImpactGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub amount_in: U64,
        pub active_id: Option<i32>,
        pub max_price_impact_bps: i32,
}

impl TryFrom<crate::instructions::postgres::SwapWithPriceImpactRow> for SwapWithPriceImpactGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::SwapWithPriceImpactRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            amount_in: carbon_core::graphql::primitives::U64(*row.amount_in),
            active_id: row.active_id.map(|v| v),
            max_price_impact_bps: (*row.max_price_impact_bps) as i32,
        })
    }
}