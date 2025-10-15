use juniper::GraphQLObject;
use crate::types::graphql::LiquidityParameterGraphQL;
use crate::types::graphql::RemainingAccountsInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AddLiquidity2")]
pub struct AddLiquidity2GraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub liquidity_parameter: LiquidityParameterGraphQL,
        pub remaining_accounts_info: RemainingAccountsInfoGraphQL,
}

impl TryFrom<crate::instructions::postgres::AddLiquidity2Row> for AddLiquidity2GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::AddLiquidity2Row) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            liquidity_parameter: row.liquidity_parameter.0.into(),
            remaining_accounts_info: row.remaining_accounts_info.0.into(),
        })
    }
}