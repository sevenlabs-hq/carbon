use juniper::GraphQLObject;


#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "BinLiquidityDistribution")]
pub struct BinLiquidityDistributionGraphQL {
            /// Define the bin ID wish to deposit to.
            pub bin_id: i32,
            /// DistributionX (or distributionY) is the percentages of amountX (or amountY) you want to add to each bin.
            pub distribution_x: i32,
            /// DistributionX (or distributionY) is the percentages of amountX (or amountY) you want to add to each bin.
            pub distribution_y: i32,
}

impl From<crate::types::BinLiquidityDistribution> for BinLiquidityDistributionGraphQL {
    fn from(original: crate::types::BinLiquidityDistribution) -> Self {
        Self {
            bin_id: original.bin_id,
            distribution_x: original.distribution_x as i32,
            distribution_y: original.distribution_y as i32,
        }
    }
}