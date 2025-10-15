use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::U64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "AdminUpdateTokenIncentives")]
pub struct AdminUpdateTokenIncentivesGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
    pub start_time: I64,
    pub end_time: I64,
    pub seconds_in_a_day: I64,
    pub day_number: U64,
    pub token_supply_per_day: U64,
}

impl From<crate::instructions::postgres::AdminUpdateTokenIncentivesRow>
    for AdminUpdateTokenIncentivesGraphQL
{
    fn from(row: crate::instructions::postgres::AdminUpdateTokenIncentivesRow) -> Self {
        Self {
            metadata: row.metadata.into(),
            start_time: carbon_core::graphql::primitives::I64(row.start_time),
            end_time: carbon_core::graphql::primitives::I64(row.end_time),
            seconds_in_a_day: carbon_core::graphql::primitives::I64(row.seconds_in_a_day),
            day_number: carbon_core::graphql::primitives::U64(*row.day_number),
            token_supply_per_day: carbon_core::graphql::primitives::U64(*row.token_supply_per_day),
        }
    }
}
