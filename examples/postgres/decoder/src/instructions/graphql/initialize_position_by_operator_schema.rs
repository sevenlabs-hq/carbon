use juniper::GraphQLObject;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "InitializePositionByOperator")]
pub struct InitializePositionByOperatorGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub lower_bin_id: i32,
        pub width: i32,
        pub fee_owner: Pubkey,
        pub lock_release_point: U64,
}

impl TryFrom<crate::instructions::postgres::InitializePositionByOperatorRow> for InitializePositionByOperatorGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::InitializePositionByOperatorRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            lower_bin_id: row.lower_bin_id,
            width: row.width,
            fee_owner: carbon_core::graphql::primitives::Pubkey(*row.fee_owner),
            lock_release_point: carbon_core::graphql::primitives::U64(*row.lock_release_point),
        })
    }
}