use crate::types::graphql::FarmTypeGraphQL;
use crate::types::graphql::FixedPointGraphQL;
use crate::types::graphql::TokenGraphQL;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Farm")]
pub struct FarmGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
    pub pool: Pubkey,
    pub tokens: Vec<Pubkey>,
    pub token_accounts: Vec<Pubkey>,
    pub supply: Vec<TokenGraphQL>,
    pub supply_left: Vec<TokenGraphQL>,
    pub accumulated_seconds_per_share: FixedPointGraphQL,
    pub offset_seconds_per_share: FixedPointGraphQL,
    pub start_time: U64,
    pub end_time: U64,
    pub last_update: U64,
    pub bump: U8,
    pub farm_type: FarmTypeGraphQL,
}

impl TryFrom<crate::accounts::postgres::FarmRow> for FarmGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::accounts::postgres::FarmRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            pool: carbon_core::graphql::primitives::Pubkey(*row.pool),
            tokens: row
                .tokens
                .into_iter()
                .map(|item| carbon_core::graphql::primitives::Pubkey(*item))
                .collect(),
            token_accounts: row
                .token_accounts
                .into_iter()
                .map(|item| carbon_core::graphql::primitives::Pubkey(*item))
                .collect(),
            supply: row.supply.0.into_iter().map(|item| item.into()).collect(),
            supply_left: row
                .supply_left
                .0
                .into_iter()
                .map(|item| item.into())
                .collect(),
            accumulated_seconds_per_share: row.accumulated_seconds_per_share.0.into(),
            offset_seconds_per_share: row.offset_seconds_per_share.0.into(),
            start_time: carbon_core::graphql::primitives::U64(*row.start_time),
            end_time: carbon_core::graphql::primitives::U64(*row.end_time),
            last_update: carbon_core::graphql::primitives::U64(*row.last_update),
            bump: carbon_core::graphql::primitives::U8((*row.bump) as u8),
            farm_type: row.farm_type.0.into(),
        })
    }
}
