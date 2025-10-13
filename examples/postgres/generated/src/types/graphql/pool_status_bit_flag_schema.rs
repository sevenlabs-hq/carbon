use juniper::GraphQLEnum;


#[derive(Debug, Clone, GraphQLEnum)]
#[graphql(name = "PoolStatusBitFlag")]
pub enum PoolStatusBitFlagGraphQL {
        Enable,
        Disable,
}

impl From<crate::types::PoolStatusBitFlag> for PoolStatusBitFlagGraphQL {
    fn from(original: crate::types::PoolStatusBitFlag) -> Self {
        match original {
            crate::types::PoolStatusBitFlag::Enable => Self::Enable,
            crate::types::PoolStatusBitFlag::Disable => Self::Disable,
        }
    }
}


