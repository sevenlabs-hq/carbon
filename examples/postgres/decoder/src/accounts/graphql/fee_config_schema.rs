use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U8;
use crate::types::graphql::FeeTierGraphQL;
use crate::types::graphql::FeesGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "FeeConfig")]
pub struct FeeConfigGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
        pub bump: U8,
        pub admin: Pubkey,
        pub flat_fees: FeesGraphQL,
        pub fee_tiers: Vec<FeeTierGraphQL>,
}

impl From<crate::accounts::postgres::FeeConfigRow> for FeeConfigGraphQL {
    fn from(row: crate::accounts::postgres::FeeConfigRow) -> Self {
        Self {
            metadata: row.metadata.into(),
            bump: carbon_core::graphql::primitives::U8((*row.bump) as u8),
            admin: carbon_core::graphql::primitives::Pubkey(*row.admin),
            flat_fees: row.flat_fees.0.into(),
            fee_tiers: row.fee_tiers.0.into_iter().map(|item| item.into()).collect(),
        }
    }
}