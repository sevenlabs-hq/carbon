use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;
use crate::types::graphql::BinLiquidityDistributionByWeightGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AddLiquidityOneSide")]
pub struct AddLiquidityOneSideGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub amount: U64,
        pub active_id: i32,
        pub max_active_bin_slippage: i32,
        pub bin_liquidity_dist: Vec<BinLiquidityDistributionByWeightGraphQL>,
}

impl TryFrom<crate::instructions::postgres::AddLiquidityOneSideRow> for AddLiquidityOneSideGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::AddLiquidityOneSideRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            amount: carbon_core::graphql::primitives::U64(*row.amount),
            active_id: row.active_id,
            max_active_bin_slippage: row.max_active_bin_slippage,
            bin_liquidity_dist: row.bin_liquidity_dist.0.into_iter().map(|item| item.into()).collect(),
        })
    }
}