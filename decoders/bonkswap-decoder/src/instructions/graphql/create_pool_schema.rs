use crate::types::graphql::FixedPointGraphQL;
use crate::types::graphql::TokenGraphQL;
use carbon_core::graphql::primitives::U8;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CreatePool")]
pub struct CreatePoolGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
    pub lp_fee: FixedPointGraphQL,
    pub buyback_fee: FixedPointGraphQL,
    pub project_fee: FixedPointGraphQL,
    pub mercanti_fee: FixedPointGraphQL,
    pub initial_token_x: TokenGraphQL,
    pub initial_token_y: TokenGraphQL,
    pub bump: U8,
}

impl TryFrom<crate::instructions::postgres::CreatePoolRow> for CreatePoolGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::CreatePoolRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            lp_fee: row.lp_fee.0.into(),
            buyback_fee: row.buyback_fee.0.into(),
            project_fee: row.project_fee.0.into(),
            mercanti_fee: row.mercanti_fee.0.into(),
            initial_token_x: row.initial_token_x.0.into(),
            initial_token_y: row.initial_token_y.0.into(),
            bump: carbon_core::graphql::primitives::U8((*row.bump) as u8),
        })
    }
}
