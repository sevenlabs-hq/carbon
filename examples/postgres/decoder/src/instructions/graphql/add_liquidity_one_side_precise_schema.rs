use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;
use crate::types::graphql::CompressedBinDepositAmountGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AddLiquidityOneSidePrecise")]
pub struct AddLiquidityOneSidePreciseGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub bins: Vec<CompressedBinDepositAmountGraphQL>,
        pub decompress_multiplier: U64,
}

impl TryFrom<crate::instructions::postgres::AddLiquidityOneSidePreciseRow> for AddLiquidityOneSidePreciseGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::AddLiquidityOneSidePreciseRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            bins: row.bins.0.into_iter().map(|item| item.into()).collect(),
            decompress_multiplier: carbon_core::graphql::primitives::U64(*row.decompress_multiplier),
        })
    }
}