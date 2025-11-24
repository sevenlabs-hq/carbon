use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "MigrateToWalletAddressV1")]
pub struct MigrateToWalletAddressV1GraphQL {
    pub instruction_metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
}

impl TryFrom<crate::instructions::postgres::migrate_to_wallet_address_v1_row::MigrateToWalletAddressV1Row> for MigrateToWalletAddressV1GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::migrate_to_wallet_address_v1_row::MigrateToWalletAddressV1Row) -> Result<Self, Self::Error> {
        Ok(Self {
            instruction_metadata: row.instruction_metadata.into(),
        })
    }
}
