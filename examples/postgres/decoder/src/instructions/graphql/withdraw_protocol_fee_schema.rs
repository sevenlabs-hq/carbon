use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;
use crate::types::graphql::RemainingAccountsInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "WithdrawProtocolFee")]
pub struct WithdrawProtocolFeeGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub max_amount_x: U64,
        pub max_amount_y: U64,
        pub remaining_accounts_info: RemainingAccountsInfoGraphQL,
}

impl TryFrom<crate::instructions::postgres::WithdrawProtocolFeeRow> for WithdrawProtocolFeeGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::WithdrawProtocolFeeRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            max_amount_x: carbon_core::graphql::primitives::U64(*row.max_amount_x),
            max_amount_y: carbon_core::graphql::primitives::U64(*row.max_amount_y),
            remaining_accounts_info: row.remaining_accounts_info.0.into(),
        })
    }
}