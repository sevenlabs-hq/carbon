use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;
use crate::types::graphql::AddLiquidityParamsGraphQL;
use crate::types::graphql::RemainingAccountsInfoGraphQL;
use crate::types::graphql::RemoveLiquidityParamsGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "RebalanceLiquidity")]
pub struct RebalanceLiquidityGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub active_id: i32,
        pub max_active_bin_slippage: i32,
        pub should_claim_fee: bool,
        pub should_claim_reward: bool,
        pub min_withdraw_x_amount: U64,
        pub max_deposit_x_amount: U64,
        pub min_withdraw_y_amount: U64,
        pub max_deposit_y_amount: U64,
        pub padding: Vec<U8>,
        pub removes: Vec<RemoveLiquidityParamsGraphQL>,
        pub adds: Vec<AddLiquidityParamsGraphQL>,
        pub remaining_accounts_info: RemainingAccountsInfoGraphQL,
}

impl TryFrom<crate::instructions::postgres::RebalanceLiquidityRow> for RebalanceLiquidityGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::RebalanceLiquidityRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            active_id: row.active_id,
            max_active_bin_slippage: (*row.max_active_bin_slippage) as i32,
            should_claim_fee: row.should_claim_fee,
            should_claim_reward: row.should_claim_reward,
            min_withdraw_x_amount: carbon_core::graphql::primitives::U64(*row.min_withdraw_x_amount),
            max_deposit_x_amount: carbon_core::graphql::primitives::U64(*row.max_deposit_x_amount),
            min_withdraw_y_amount: carbon_core::graphql::primitives::U64(*row.min_withdraw_y_amount),
            max_deposit_y_amount: carbon_core::graphql::primitives::U64(*row.max_deposit_y_amount),
            padding: row.padding.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
            removes: row.removes.0.into_iter().map(|item| item.into()).collect(),
            adds: row.adds.0.into_iter().map(|item| item.into()).collect(),
            remaining_accounts_info: row.remaining_accounts_info.0.into(),
        })
    }
}