use juniper::GraphQLObject;
use crate::types::graphql::LiquidityParameterByStrategyGraphQL;
use crate::types::graphql::RemainingAccountsInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AddLiquidityByStrategy2")]
pub struct AddLiquidityByStrategy2GraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub liquidity_parameter: LiquidityParameterByStrategyGraphQL,
        pub remaining_accounts_info: RemainingAccountsInfoGraphQL,
}

impl TryFrom<crate::instructions::postgres::AddLiquidityByStrategy2Row> for AddLiquidityByStrategy2GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::AddLiquidityByStrategy2Row) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            liquidity_parameter: row.liquidity_parameter.0.into(),
            remaining_accounts_info: row.remaining_accounts_info.0.into(),
        })
    }
}