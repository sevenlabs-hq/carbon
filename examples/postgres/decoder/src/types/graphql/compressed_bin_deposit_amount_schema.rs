use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U32;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CompressedBinDepositAmount")]
pub struct CompressedBinDepositAmountGraphQL {
        pub bin_id: i32,
        pub amount: U32,
}

impl From<crate::types::CompressedBinDepositAmount> for CompressedBinDepositAmountGraphQL {
    fn from(original: crate::types::CompressedBinDepositAmount) -> Self {
        Self {
            bin_id: original.bin_id,
            amount: carbon_core::graphql::primitives::U32(original.amount),
        }
    }
}