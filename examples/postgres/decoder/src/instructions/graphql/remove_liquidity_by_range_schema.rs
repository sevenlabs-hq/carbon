use juniper::GraphQLObject;


#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "RemoveLiquidityByRange")]
pub struct RemoveLiquidityByRangeGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub from_bin_id: i32,
        pub to_bin_id: i32,
        pub bps_to_remove: i32,
}

impl TryFrom<crate::instructions::postgres::RemoveLiquidityByRangeRow> for RemoveLiquidityByRangeGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::RemoveLiquidityByRangeRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            from_bin_id: row.from_bin_id,
            to_bin_id: row.to_bin_id,
            bps_to_remove: (*row.bps_to_remove) as i32,
        })
    }
}