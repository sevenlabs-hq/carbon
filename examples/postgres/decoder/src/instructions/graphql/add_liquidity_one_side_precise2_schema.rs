use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;
use crate::types::graphql::CompressedBinDepositAmountGraphQL;
use crate::types::graphql::RemainingAccountsInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AddLiquidityOneSidePrecise2")]
pub struct AddLiquidityOneSidePrecise2GraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub bins: Vec<CompressedBinDepositAmountGraphQL>,
        pub decompress_multiplier: U64,
        pub max_amount: U64,
        pub remaining_accounts_info: RemainingAccountsInfoGraphQL,
}

impl TryFrom<crate::instructions::postgres::AddLiquidityOneSidePrecise2Row> for AddLiquidityOneSidePrecise2GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::AddLiquidityOneSidePrecise2Row) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            bins: row.bins.0.into_iter().map(|item| item.into()).collect(),
            decompress_multiplier: carbon_core::graphql::primitives::U64(*row.decompress_multiplier),
            max_amount: carbon_core::graphql::primitives::U64(*row.max_amount),
            remaining_accounts_info: row.remaining_accounts_info.0.into(),
        })
    }
}