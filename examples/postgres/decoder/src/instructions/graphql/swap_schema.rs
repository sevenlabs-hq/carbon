use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Swap")]
pub struct SwapGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub amount_in: U64,
        pub min_amount_out: U64,
}

impl TryFrom<crate::instructions::postgres::SwapRow> for SwapGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::SwapRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            amount_in: carbon_core::graphql::primitives::U64(*row.amount_in),
            min_amount_out: carbon_core::graphql::primitives::U64(*row.min_amount_out),
        })
    }
}