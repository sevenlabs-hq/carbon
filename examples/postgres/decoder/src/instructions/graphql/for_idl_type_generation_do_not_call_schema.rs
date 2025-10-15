use juniper::GraphQLObject;
use crate::types::graphql::ActivationTypeGraphQL;
use crate::types::graphql::PairStatusGraphQL;
use crate::types::graphql::PairTypeGraphQL;
use crate::types::graphql::ResizeSideGraphQL;
use crate::types::graphql::RoundingGraphQL;
use crate::types::graphql::TokenProgramFlagsGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "ForIdlTypeGenerationDoNotCall")]
pub struct ForIdlTypeGenerationDoNotCallGraphQL {
    pub metadata: crate::instructions::graphql::InstructionMetadataGraphQL,
        pub pair_status: PairStatusGraphQL,
        pub pair_type: PairTypeGraphQL,
        pub activation_type: ActivationTypeGraphQL,
        pub token_program_flag: TokenProgramFlagsGraphQL,
        pub resize_side: ResizeSideGraphQL,
        pub rounding: RoundingGraphQL,
}

impl TryFrom<crate::instructions::postgres::ForIdlTypeGenerationDoNotCallRow> for ForIdlTypeGenerationDoNotCallGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::instructions::postgres::ForIdlTypeGenerationDoNotCallRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            pair_status: row.pair_status.0.into(),
            pair_type: row.pair_type.0.into(),
            activation_type: row.activation_type.0.into(),
            token_program_flag: row.token_program_flag.0.into(),
            resize_side: row.resize_side.0.into(),
            rounding: row.rounding.0.into(),
        })
    }
}