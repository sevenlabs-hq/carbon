use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdatePoolStatus")]
pub struct UpdatePoolStatusGraphQL {
        pub status: U8,
}

impl From<crate::instructions::postgres::UpdatePoolStatusRow> for UpdatePoolStatusGraphQL {
    fn from(row: crate::instructions::postgres::UpdatePoolStatusRow) -> Self {
        Self {
            status: carbon_core::graphql::primitives::U8((*row.status) as u8),
        }
    }
}