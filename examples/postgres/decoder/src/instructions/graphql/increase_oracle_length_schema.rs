use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "IncreaseOracleLength")]
pub struct IncreaseOracleLengthGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub length_to_add: U64,
}

impl TryFrom<crate::instructions::postgres::IncreaseOracleLengthRow> for IncreaseOracleLengthGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::IncreaseOracleLengthRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            length_to_add: carbon_core::graphql::primitives::U64(*row.length_to_add),
        })
    }
}