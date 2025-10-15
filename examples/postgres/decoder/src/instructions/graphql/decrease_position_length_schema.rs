use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "DecreasePositionLength")]
pub struct DecreasePositionLengthGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub length_to_remove: i32,
        pub side: U8,
}

impl TryFrom<crate::instructions::postgres::DecreasePositionLengthRow> for DecreasePositionLengthGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::DecreasePositionLengthRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            length_to_remove: (*row.length_to_remove) as i32,
            side: carbon_core::graphql::primitives::U8((*row.side) as u8),
        })
    }
}