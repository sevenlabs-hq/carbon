use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "BondingCurve")]
pub struct BondingCurveGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
    pub virtual_token_reserves: U64,
    pub virtual_sol_reserves: U64,
    pub real_token_reserves: U64,
    pub real_sol_reserves: U64,
    pub token_total_supply: U64,
    pub complete: bool,
    pub creator: Pubkey,
}

impl From<crate::accounts::postgres::BondingCurveRow> for BondingCurveGraphQL {
    fn from(row: crate::accounts::postgres::BondingCurveRow) -> Self {
        Self {
            metadata: row.metadata.into(),
            virtual_token_reserves: carbon_core::graphql::primitives::U64(
                *row.virtual_token_reserves,
            ),
            virtual_sol_reserves: carbon_core::graphql::primitives::U64(*row.virtual_sol_reserves),
            real_token_reserves: carbon_core::graphql::primitives::U64(*row.real_token_reserves),
            real_sol_reserves: carbon_core::graphql::primitives::U64(*row.real_sol_reserves),
            token_total_supply: carbon_core::graphql::primitives::U64(*row.token_total_supply),
            complete: row.complete,
            creator: carbon_core::graphql::primitives::Pubkey(*row.creator),
        }
    }
}
