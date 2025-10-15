use crate::types::graphql::FeesGraphQL;
use carbon_core::graphql::primitives::U128;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "FeeTier")]
pub struct FeeTierGraphQL {
    pub market_cap_lamports_threshold: U128,
    pub fees: FeesGraphQL,
}

impl From<crate::types::FeeTier> for FeeTierGraphQL {
    fn from(original: crate::types::FeeTier) -> Self {
        Self {
            market_cap_lamports_threshold: carbon_core::graphql::primitives::U128(
                original.market_cap_lamports_threshold,
            ),
            fees: original.fees.into(),
        }
    }
}
