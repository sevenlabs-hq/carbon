use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Deposit")]
pub struct DepositGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub lp_token_amount_out: U64,
        pub max_base_amount_in: U64,
        pub max_quote_amount_in: U64,
}

impl From<crate::instructions::postgres::DepositRow> for DepositGraphQL {
    fn from(row: crate::instructions::postgres::DepositRow) -> Self {
        Self {
            metadata: row.metadata.into(),
            lp_token_amount_out: carbon_core::graphql::primitives::U64(*row.lp_token_amount_out),
            max_base_amount_in: carbon_core::graphql::primitives::U64(*row.max_base_amount_in),
            max_quote_amount_in: carbon_core::graphql::primitives::U64(*row.max_quote_amount_in),
        }
    }
}