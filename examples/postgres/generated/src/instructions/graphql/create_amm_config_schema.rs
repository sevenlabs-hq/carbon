use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U32;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CreateAmmConfig")]
pub struct CreateAmmConfigGraphQL {
        pub index: i32,
        pub tick_spacing: i32,
        pub trade_fee_rate: U32,
        pub protocol_fee_rate: U32,
        pub fund_fee_rate: U32,
}

impl From<crate::instructions::postgres::CreateAmmConfigRow> for CreateAmmConfigGraphQL {
    fn from(row: crate::instructions::postgres::CreateAmmConfigRow) -> Self {
        Self {
            index: (*row.index) as i32,
            tick_spacing: (*row.tick_spacing) as i32,
            trade_fee_rate: carbon_core::graphql::primitives::U32((*row.trade_fee_rate) as u32),
            protocol_fee_rate: carbon_core::graphql::primitives::U32((*row.protocol_fee_rate) as u32),
            fund_fee_rate: carbon_core::graphql::primitives::U32((*row.fund_fee_rate) as u32),
        }
    }
}