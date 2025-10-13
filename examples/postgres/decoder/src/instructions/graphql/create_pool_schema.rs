use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CreatePool")]
pub struct CreatePoolGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub index: i32,
        pub base_amount_in: U64,
        pub quote_amount_in: U64,
        pub coin_creator: Pubkey,
}

impl From<crate::instructions::postgres::CreatePoolRow> for CreatePoolGraphQL {
    fn from(row: crate::instructions::postgres::CreatePoolRow) -> Self {
        Self {
            metadata: row.metadata.into(),
            index: (*row.index) as i32,
            base_amount_in: carbon_core::graphql::primitives::U64(*row.base_amount_in),
            quote_amount_in: carbon_core::graphql::primitives::U64(*row.quote_amount_in),
            coin_creator: carbon_core::graphql::primitives::Pubkey(*row.coin_creator),
        }
    }
}