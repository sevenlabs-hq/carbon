use crate::types::graphql::TokenGraphQL;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CreateFarm")]
pub struct CreateFarmGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
    pub supply: TokenGraphQL,
    pub duration: U64,
    pub bump: U8,
}

impl TryFrom<crate::instructions::postgres::CreateFarmRow> for CreateFarmGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::CreateFarmRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            supply: row.supply.0.into(),
            duration: carbon_core::graphql::primitives::U64(*row.duration),
            bump: carbon_core::graphql::primitives::U8((*row.bump) as u8),
        })
    }
}
