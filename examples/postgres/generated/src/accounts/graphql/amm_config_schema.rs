use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U32;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AmmConfig")]
pub struct AmmConfigGraphQL {
        pub bump: U8,
        pub index: i32,
        pub owner: Pubkey,
        pub protocol_fee_rate: U32,
        pub trade_fee_rate: U32,
        pub tick_spacing: i32,
        pub fund_fee_rate: U32,
        pub padding_u32: U32,
        pub fund_owner: Pubkey,
        pub padding: Vec<U64>,
}

impl From<crate::accounts::postgres::AmmConfigRow> for AmmConfigGraphQL {
    fn from(row: crate::accounts::postgres::AmmConfigRow) -> Self {
        Self {
            bump: carbon_core::graphql::primitives::U8((*row.bump) as u8),
            index: (*row.index) as i32,
            owner: carbon_core::graphql::primitives::Pubkey(*row.owner),
            protocol_fee_rate: carbon_core::graphql::primitives::U32((*row.protocol_fee_rate) as u32),
            trade_fee_rate: carbon_core::graphql::primitives::U32((*row.trade_fee_rate) as u32),
            tick_spacing: (*row.tick_spacing) as i32,
            fund_fee_rate: carbon_core::graphql::primitives::U32((*row.fund_fee_rate) as u32),
            padding_u32: carbon_core::graphql::primitives::U32((*row.padding_u32) as u32),
            fund_owner: carbon_core::graphql::primitives::Pubkey(*row.fund_owner),
            padding: row.padding.into_iter().map(|item| carbon_core::graphql::primitives::U64(*item)).collect(),
        }
    }
}