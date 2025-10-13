use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "TransferRewardOwner")]
pub struct TransferRewardOwnerGraphQL {
        pub new_owner: Pubkey,
}

impl From<crate::instructions::postgres::TransferRewardOwnerRow> for TransferRewardOwnerGraphQL {
    fn from(row: crate::instructions::postgres::TransferRewardOwnerRow) -> Self {
        Self {
            new_owner: carbon_core::graphql::primitives::Pubkey(*row.new_owner),
        }
    }
}