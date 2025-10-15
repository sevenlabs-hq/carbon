use juniper::GraphQLObject;
use crate::types::graphql::PositionBinDataGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "DummyZcAccount")]
pub struct DummyZcAccountGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
        pub position_bin_data: PositionBinDataGraphQL,
}

impl TryFrom<crate::accounts::postgres::DummyZcAccountRow> for DummyZcAccountGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::accounts::postgres::DummyZcAccountRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            position_bin_data: row.position_bin_data.0.into(),
        })
    }
}