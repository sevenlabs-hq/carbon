use crate::types::graphql::TokenGraphQL;
use carbon_core::graphql::primitives::U64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AddSupply")]
pub struct AddSupplyGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
    pub supply_marco: TokenGraphQL,
    pub supply_project_first: TokenGraphQL,
    pub supply_project_second: TokenGraphQL,
    pub duration: U64,
}

impl TryFrom<crate::instructions::postgres::AddSupplyRow> for AddSupplyGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::AddSupplyRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            supply_marco: row.supply_marco.0.into(),
            supply_project_first: row.supply_project_first.0.into(),
            supply_project_second: row.supply_project_second.0.into(),
            duration: carbon_core::graphql::primitives::U64(*row.duration),
        })
    }
}
