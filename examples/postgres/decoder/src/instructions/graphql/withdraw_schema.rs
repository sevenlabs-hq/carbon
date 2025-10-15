use carbon_core::graphql::primitives::U64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Withdraw")]
pub struct WithdrawGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
    pub lp_token_amount_in: U64,
    pub min_base_amount_out: U64,
    pub min_quote_amount_out: U64,
}

impl From<crate::instructions::postgres::WithdrawRow> for WithdrawGraphQL {
    fn from(row: crate::instructions::postgres::WithdrawRow) -> Self {
        Self {
            metadata: row.metadata.into(),
            lp_token_amount_in: carbon_core::graphql::primitives::U64(*row.lp_token_amount_in),
            min_base_amount_out: carbon_core::graphql::primitives::U64(*row.min_base_amount_out),
            min_quote_amount_out: carbon_core::graphql::primitives::U64(*row.min_quote_amount_out),
        }
    }
}
