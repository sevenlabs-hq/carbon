use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;
use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "UpdateFeeConfig")]
pub struct UpdateFeeConfigGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
    pub lp_fee_basis_points: U64,
    pub protocol_fee_basis_points: U64,
    pub protocol_fee_recipients: Vec<Pubkey>,
    pub coin_creator_fee_basis_points: U64,
    pub admin_set_coin_creator_authority: Pubkey,
}

impl From<crate::instructions::postgres::UpdateFeeConfigRow> for UpdateFeeConfigGraphQL {
    fn from(row: crate::instructions::postgres::UpdateFeeConfigRow) -> Self {
        Self {
            metadata: row.metadata.into(),
            lp_fee_basis_points: carbon_core::graphql::primitives::U64(*row.lp_fee_basis_points),
            protocol_fee_basis_points: carbon_core::graphql::primitives::U64(
                *row.protocol_fee_basis_points,
            ),
            protocol_fee_recipients: row
                .protocol_fee_recipients
                .into_iter()
                .map(|item| carbon_core::graphql::primitives::Pubkey(*item))
                .collect(),
            coin_creator_fee_basis_points: carbon_core::graphql::primitives::U64(
                *row.coin_creator_fee_basis_points,
            ),
            admin_set_coin_creator_authority: carbon_core::graphql::primitives::Pubkey(
                *row.admin_set_coin_creator_authority,
            ),
        }
    }
}
