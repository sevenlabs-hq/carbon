use juniper::GraphQLObject;


#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "BinLiquidityReduction")]
pub struct BinLiquidityReductionGraphQL {
        pub bin_id: i32,
        pub bps_to_remove: i32,
}

impl From<crate::types::BinLiquidityReduction> for BinLiquidityReductionGraphQL {
    fn from(original: crate::types::BinLiquidityReduction) -> Self {
        Self {
            bin_id: original.bin_id,
            bps_to_remove: original.bps_to_remove as i32,
        }
    }
}