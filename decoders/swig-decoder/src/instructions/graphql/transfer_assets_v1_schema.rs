use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "TransferAssetsV1")]
pub struct TransferAssetsV1GraphQL {
    pub instruction_metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
}

impl TryFrom<crate::instructions::postgres::transfer_assets_v1_row::TransferAssetsV1Row>
    for TransferAssetsV1GraphQL
{
    type Error = carbon_core::error::Error;
    fn try_from(
        row: crate::instructions::postgres::transfer_assets_v1_row::TransferAssetsV1Row,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            instruction_metadata: row.instruction_metadata.into(),
        })
    }
}
