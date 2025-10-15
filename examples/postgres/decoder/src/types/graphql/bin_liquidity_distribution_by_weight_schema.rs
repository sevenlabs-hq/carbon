use juniper::GraphQLObject;


#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "BinLiquidityDistributionByWeight")]
pub struct BinLiquidityDistributionByWeightGraphQL {
            /// Define the bin ID wish to deposit to.
            pub bin_id: i32,
            /// weight of liquidity distributed for this bin id
            pub weight: i32,
}

impl From<crate::types::BinLiquidityDistributionByWeight> for BinLiquidityDistributionByWeightGraphQL {
    fn from(original: crate::types::BinLiquidityDistributionByWeight) -> Self {
        Self {
            bin_id: original.bin_id,
            weight: original.weight as i32,
        }
    }
}