use juniper::GraphQLObject;


#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "Disable")]
pub struct DisableGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub disable_create_pool: bool,
        pub disable_deposit: bool,
        pub disable_withdraw: bool,
        pub disable_buy: bool,
        pub disable_sell: bool,
}

impl From<crate::instructions::postgres::DisableRow> for DisableGraphQL {
    fn from(row: crate::instructions::postgres::DisableRow) -> Self {
        Self {
            metadata: row.metadata.into(),
            disable_create_pool: row.disable_create_pool,
            disable_deposit: row.disable_deposit,
            disable_withdraw: row.disable_withdraw,
            disable_buy: row.disable_buy,
            disable_sell: row.disable_sell,
        }
    }
}