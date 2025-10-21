use crate::types::graphql::TokenGraphQL;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CreateTripleFarm")]
pub struct CreateTripleFarmGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
    pub supply_marco: TokenGraphQL,
    pub supply_project_first: TokenGraphQL,
    pub supply_project_second: TokenGraphQL,
    pub duration: U64,
    pub bump: U8,
}

impl TryFrom<crate::instructions::postgres::CreateTripleFarmRow> for CreateTripleFarmGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(
        row: crate::instructions::postgres::CreateTripleFarmRow,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            supply_marco: row.supply_marco.0.into(),
            supply_project_first: row.supply_project_first.0.into(),
            supply_project_second: row.supply_project_second.0.into(),
            duration: carbon_core::graphql::primitives::U64(*row.duration),
            bump: carbon_core::graphql::primitives::U8((*row.bump) as u8),
        })
    }
}
