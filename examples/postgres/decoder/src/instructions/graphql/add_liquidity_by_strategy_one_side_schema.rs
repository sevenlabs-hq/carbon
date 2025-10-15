use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;
use crate::types::graphql::StrategyParametersGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AddLiquidityByStrategyOneSide")]
pub struct AddLiquidityByStrategyOneSideGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub amount: U64,
        pub active_id: i32,
        pub max_active_bin_slippage: i32,
        pub strategy_parameters: StrategyParametersGraphQL,
}

impl TryFrom<crate::instructions::postgres::AddLiquidityByStrategyOneSideRow> for AddLiquidityByStrategyOneSideGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::AddLiquidityByStrategyOneSideRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            amount: carbon_core::graphql::primitives::U64(*row.amount),
            active_id: row.active_id,
            max_active_bin_slippage: row.max_active_bin_slippage,
            strategy_parameters: row.strategy_parameters.0.into(),
        })
    }
}