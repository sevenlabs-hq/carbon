use juniper::GraphQLObject;
use crate::types::graphql::BinLiquidityReductionGraphQL;
use crate::types::graphql::RemainingAccountsInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "RemoveLiquidity2")]
pub struct RemoveLiquidity2GraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub bin_liquidity_removal: Vec<BinLiquidityReductionGraphQL>,
        pub remaining_accounts_info: RemainingAccountsInfoGraphQL,
}

impl TryFrom<crate::instructions::postgres::RemoveLiquidity2Row> for RemoveLiquidity2GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::RemoveLiquidity2Row) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            bin_liquidity_removal: row.bin_liquidity_removal.0.into_iter().map(|item| item.into()).collect(),
            remaining_accounts_info: row.remaining_accounts_info.0.into(),
        })
    }
}