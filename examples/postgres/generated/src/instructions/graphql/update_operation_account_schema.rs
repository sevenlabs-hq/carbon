use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdateOperationAccount")]
pub struct UpdateOperationAccountGraphQL {
        pub param: U8,
        pub keys: Vec<Pubkey>,
}

impl From<crate::instructions::postgres::UpdateOperationAccountRow> for UpdateOperationAccountGraphQL {
    fn from(row: crate::instructions::postgres::UpdateOperationAccountRow) -> Self {
        Self {
            param: carbon_core::graphql::primitives::U8((*row.param) as u8),
            keys: row.keys.into_iter().map(|item| carbon_core::graphql::primitives::Pubkey(*item)).collect(),
        }
    }
}