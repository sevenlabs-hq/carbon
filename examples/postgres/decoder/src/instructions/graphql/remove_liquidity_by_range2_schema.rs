use juniper::GraphQLObject;
use crate::types::graphql::RemainingAccountsInfoGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "RemoveLiquidityByRange2")]
pub struct RemoveLiquidityByRange2GraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub from_bin_id: i32,
        pub to_bin_id: i32,
        pub bps_to_remove: i32,
        pub remaining_accounts_info: RemainingAccountsInfoGraphQL,
}

impl TryFrom<crate::instructions::postgres::RemoveLiquidityByRange2Row> for RemoveLiquidityByRange2GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::RemoveLiquidityByRange2Row) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            from_bin_id: row.from_bin_id,
            to_bin_id: row.to_bin_id,
            bps_to_remove: (*row.bps_to_remove) as i32,
            remaining_accounts_info: row.remaining_accounts_info.0.into(),
        })
    }
}