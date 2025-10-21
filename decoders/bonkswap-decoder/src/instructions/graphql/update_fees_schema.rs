use crate::types::graphql::FixedPointGraphQL;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdateFees")]
pub struct UpdateFeesGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
    pub new_buyback_fee: FixedPointGraphQL,
    pub new_project_fee: FixedPointGraphQL,
    pub new_provider_fee: FixedPointGraphQL,
    pub new_mercanti_fee: FixedPointGraphQL,
}

impl TryFrom<crate::instructions::postgres::UpdateFeesRow> for UpdateFeesGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::UpdateFeesRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            new_buyback_fee: row.new_buyback_fee.0.into(),
            new_project_fee: row.new_project_fee.0.into(),
            new_provider_fee: row.new_provider_fee.0.into(),
            new_mercanti_fee: row.new_mercanti_fee.0.into(),
        })
    }
}
