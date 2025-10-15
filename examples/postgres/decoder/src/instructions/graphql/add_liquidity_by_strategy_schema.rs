use juniper::GraphQLObject;
use crate::types::graphql::LiquidityParameterByStrategyGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AddLiquidityByStrategy")]
pub struct AddLiquidityByStrategyGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub liquidity_parameter: LiquidityParameterByStrategyGraphQL,
}

impl TryFrom<crate::instructions::postgres::AddLiquidityByStrategyRow> for AddLiquidityByStrategyGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::AddLiquidityByStrategyRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            liquidity_parameter: row.liquidity_parameter.0.into(),
        })
    }
}