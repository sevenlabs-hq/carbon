use juniper::GraphQLEnum;


#[derive(Debug, Clone, GraphQLEnum)]
#[graphql(name = "TokenProgramFlags")]
pub enum TokenProgramFlagsGraphQL {
        TokenProgram,
        TokenProgram2022,
}

impl From<crate::types::TokenProgramFlags> for TokenProgramFlagsGraphQL {
    fn from(original: crate::types::TokenProgramFlags) -> Self {
        match original {
            crate::types::TokenProgramFlags::TokenProgram => Self::TokenProgram,
            crate::types::TokenProgramFlags::TokenProgram2022 => Self::TokenProgram2022,
        }
    }
}


