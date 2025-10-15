use carbon_core::graphql::primitives::U64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Fees")]
pub struct FeesGraphQL {
    pub lp_fee_bps: U64,
    pub protocol_fee_bps: U64,
    pub creator_fee_bps: U64,
}

impl From<crate::types::Fees> for FeesGraphQL {
    fn from(original: crate::types::Fees) -> Self {
        Self {
            lp_fee_bps: carbon_core::graphql::primitives::U64(original.lp_fee_bps),
            protocol_fee_bps: carbon_core::graphql::primitives::U64(original.protocol_fee_bps),
            creator_fee_bps: carbon_core::graphql::primitives::U64(original.creator_fee_bps),
        }
    }
}
