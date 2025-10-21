use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U8;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "State")]
pub struct StateGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
    pub admin: Pubkey,
    pub program_authority: Pubkey,
    pub bump: U8,
    pub nonce: U8,
}

impl TryFrom<crate::accounts::postgres::StateRow> for StateGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::accounts::postgres::StateRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            admin: carbon_core::graphql::primitives::Pubkey(*row.admin),
            program_authority: carbon_core::graphql::primitives::Pubkey(*row.program_authority),
            bump: carbon_core::graphql::primitives::U8((*row.bump) as u8),
            nonce: carbon_core::graphql::primitives::U8((*row.nonce) as u8),
        })
    }
}
