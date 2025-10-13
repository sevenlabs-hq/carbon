use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U32;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdateAmmConfig")]
pub struct UpdateAmmConfigGraphQL {
        pub param: U8,
        pub value: U32,
}

impl From<crate::instructions::postgres::UpdateAmmConfigRow> for UpdateAmmConfigGraphQL {
    fn from(row: crate::instructions::postgres::UpdateAmmConfigRow) -> Self {
        Self {
            param: carbon_core::graphql::primitives::U8((*row.param) as u8),
            value: carbon_core::graphql::primitives::U32((*row.value) as u32),
        }
    }
}