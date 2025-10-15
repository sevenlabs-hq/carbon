use juniper::GraphQLObject;
use crate::types::graphql::LiquidityParameterGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AddLiquidity")]
pub struct AddLiquidityGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub liquidity_parameter: LiquidityParameterGraphQL,
}

impl TryFrom<crate::instructions::postgres::AddLiquidityRow> for AddLiquidityGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::AddLiquidityRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            liquidity_parameter: row.liquidity_parameter.0.into(),
        })
    }
}