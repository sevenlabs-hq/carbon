use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "IncreasePositionLength")]
pub struct IncreasePositionLengthGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub length_to_add: i32,
        pub side: U8,
}

impl TryFrom<crate::instructions::postgres::IncreasePositionLengthRow> for IncreasePositionLengthGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::IncreasePositionLengthRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            length_to_add: (*row.length_to_add) as i32,
            side: carbon_core::graphql::primitives::U8((*row.side) as u8),
        })
    }
}