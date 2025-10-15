use juniper::GraphQLObject;
use crate::types::graphql::BinLiquidityReductionGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "RemoveLiquidity")]
pub struct RemoveLiquidityGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub bin_liquidity_removal: Vec<BinLiquidityReductionGraphQL>,
}

impl TryFrom<crate::instructions::postgres::RemoveLiquidityRow> for RemoveLiquidityGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::RemoveLiquidityRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            bin_liquidity_removal: row.bin_liquidity_removal.0.into_iter().map(|item| item.into()).collect(),
        })
    }
}