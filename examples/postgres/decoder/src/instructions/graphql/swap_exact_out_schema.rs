use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "SwapExactOut")]
pub struct SwapExactOutGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub max_in_amount: U64,
        pub out_amount: U64,
}

impl TryFrom<crate::instructions::postgres::SwapExactOutRow> for SwapExactOutGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::SwapExactOutRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            max_in_amount: carbon_core::graphql::primitives::U64(*row.max_in_amount),
            out_amount: carbon_core::graphql::primitives::U64(*row.out_amount),
        })
    }
}