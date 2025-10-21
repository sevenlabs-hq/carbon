use crate::types::graphql::FixedPointGraphQL;
use crate::types::graphql::TokenGraphQL;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Swap")]
pub struct SwapGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
    pub delta_in: TokenGraphQL,
    pub price_limit: FixedPointGraphQL,
    pub x_to_y: bool,
}

impl TryFrom<crate::instructions::postgres::SwapRow> for SwapGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::SwapRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            delta_in: row.delta_in.0.into(),
            price_limit: row.price_limit.0.into(),
            x_to_y: row.x_to_y,
        })
    }
}
