use crate::types::graphql::FixedPointGraphQL;
use crate::types::graphql::ProductGraphQL;
use crate::types::graphql::TokenGraphQL;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Pool")]
pub struct PoolGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
    pub token_x: Pubkey,
    pub token_y: Pubkey,
    pub pool_x_account: Pubkey,
    pub pool_y_account: Pubkey,
    pub admin: Pubkey,
    pub project_owner: Pubkey,
    pub token_x_reserve: TokenGraphQL,
    pub token_y_reserve: TokenGraphQL,
    pub self_shares: TokenGraphQL,
    pub all_shares: TokenGraphQL,
    pub buyback_amount_x: TokenGraphQL,
    pub buyback_amount_y: TokenGraphQL,
    pub project_amount_x: TokenGraphQL,
    pub project_amount_y: TokenGraphQL,
    pub mercanti_amount_x: TokenGraphQL,
    pub mercanti_amount_y: TokenGraphQL,
    pub lp_accumulator_x: FixedPointGraphQL,
    pub lp_accumulator_y: FixedPointGraphQL,
    pub const_k: ProductGraphQL,
    pub price: FixedPointGraphQL,
    pub lp_fee: FixedPointGraphQL,
    pub buyback_fee: FixedPointGraphQL,
    pub project_fee: FixedPointGraphQL,
    pub mercanti_fee: FixedPointGraphQL,
    pub farm_count: U64,
    pub bump: U8,
}

impl TryFrom<crate::accounts::postgres::PoolRow> for PoolGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::accounts::postgres::PoolRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            token_x: carbon_core::graphql::primitives::Pubkey(*row.token_x),
            token_y: carbon_core::graphql::primitives::Pubkey(*row.token_y),
            pool_x_account: carbon_core::graphql::primitives::Pubkey(*row.pool_x_account),
            pool_y_account: carbon_core::graphql::primitives::Pubkey(*row.pool_y_account),
            admin: carbon_core::graphql::primitives::Pubkey(*row.admin),
            project_owner: carbon_core::graphql::primitives::Pubkey(*row.project_owner),
            token_x_reserve: row.token_x_reserve.0.into(),
            token_y_reserve: row.token_y_reserve.0.into(),
            self_shares: row.self_shares.0.into(),
            all_shares: row.all_shares.0.into(),
            buyback_amount_x: row.buyback_amount_x.0.into(),
            buyback_amount_y: row.buyback_amount_y.0.into(),
            project_amount_x: row.project_amount_x.0.into(),
            project_amount_y: row.project_amount_y.0.into(),
            mercanti_amount_x: row.mercanti_amount_x.0.into(),
            mercanti_amount_y: row.mercanti_amount_y.0.into(),
            lp_accumulator_x: row.lp_accumulator_x.0.into(),
            lp_accumulator_y: row.lp_accumulator_y.0.into(),
            const_k: row.const_k.0.into(),
            price: row.price.0.into(),
            lp_fee: row.lp_fee.0.into(),
            buyback_fee: row.buyback_fee.0.into(),
            project_fee: row.project_fee.0.into(),
            mercanti_fee: row.mercanti_fee.0.into(),
            farm_count: carbon_core::graphql::primitives::U64(*row.farm_count),
            bump: carbon_core::graphql::primitives::U8((*row.bump) as u8),
        })
    }
}
