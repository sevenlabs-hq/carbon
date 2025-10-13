use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "OperationState")]
pub struct OperationStateGraphQL {
        pub bump: U8,
        pub operation_owners: Vec<Pubkey>,
        pub whitelist_mints: Vec<Pubkey>,
}

impl From<crate::accounts::postgres::OperationStateRow> for OperationStateGraphQL {
    fn from(row: crate::accounts::postgres::OperationStateRow) -> Self {
        Self {
            bump: carbon_core::graphql::primitives::U8((*row.bump) as u8),
            operation_owners: row.operation_owners.into_iter().map(|item| carbon_core::graphql::primitives::Pubkey(*item)).collect(),
            whitelist_mints: row.whitelist_mints.into_iter().map(|item| carbon_core::graphql::primitives::Pubkey(*item)).collect(),
        }
    }
}