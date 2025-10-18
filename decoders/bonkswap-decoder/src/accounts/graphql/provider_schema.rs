use crate::types::graphql::FixedPointGraphQL;
use crate::types::graphql::TokenGraphQL;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Provider")]
pub struct ProviderGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub owner: Pubkey,
    pub shares: TokenGraphQL,
    pub last_fee_accumulator_x: FixedPointGraphQL,
    pub last_fee_accumulator_y: FixedPointGraphQL,
    pub last_seconds_per_share: FixedPointGraphQL,
    pub last_withdraw_time: U64,
    pub tokens_owed_x: TokenGraphQL,
    pub tokens_owed_y: TokenGraphQL,
    pub current_farm_count: U64,
    pub bump: U8,
}

impl TryFrom<crate::accounts::postgres::ProviderRow> for ProviderGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::accounts::postgres::ProviderRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            token_x: carbon_core::graphql::primitives::Pubkey(*row.token_x),
            token_y: carbon_core::graphql::primitives::Pubkey(*row.token_y),
            owner: carbon_core::graphql::primitives::Pubkey(*row.owner),
            shares: row.shares.0.into(),
            last_fee_accumulator_x: row.last_fee_accumulator_x.0.into(),
            last_fee_accumulator_y: row.last_fee_accumulator_y.0.into(),
            last_seconds_per_share: row.last_seconds_per_share.0.into(),
            last_withdraw_time: carbon_core::graphql::primitives::U64(*row.last_withdraw_time),
            tokens_owed_x: row.tokens_owed_x.0.into(),
            tokens_owed_y: row.tokens_owed_y.0.into(),
            current_farm_count: carbon_core::graphql::primitives::U64(*row.current_farm_count),
            bump: carbon_core::graphql::primitives::U8((*row.bump) as u8),
        })
    }
}
