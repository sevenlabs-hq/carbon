use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;
use crate::types::graphql::RemainingAccountsInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SwapWithPriceImpact2")]
pub struct SwapWithPriceImpact2GraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub amount_in: U64,
        pub active_id: Option<i32>,
        pub max_price_impact_bps: i32,
        pub remaining_accounts_info: RemainingAccountsInfoGraphQL,
}

impl TryFrom<crate::instructions::postgres::SwapWithPriceImpact2Row> for SwapWithPriceImpact2GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::SwapWithPriceImpact2Row) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            amount_in: carbon_core::graphql::primitives::U64(*row.amount_in),
            active_id: row.active_id.map(|v| v),
            max_price_impact_bps: (*row.max_price_impact_bps) as i32,
            remaining_accounts_info: row.remaining_accounts_info.0.into(),
        })
    }
}