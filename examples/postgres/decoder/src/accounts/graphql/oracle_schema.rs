use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Oracle")]
pub struct OracleGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
            /// Index of latest observation
            pub idx: U64,
            /// Size of active sample. Active sample is initialized observation.
            pub active_size: U64,
            /// Number of observations
            pub length: U64,
}

impl TryFrom<crate::accounts::postgres::OracleRow> for OracleGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::accounts::postgres::OracleRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            idx: carbon_core::graphql::primitives::U64(*row.idx),
            active_size: carbon_core::graphql::primitives::U64(*row.active_size),
            length: carbon_core::graphql::primitives::U64(*row.length),
        })
    }
}