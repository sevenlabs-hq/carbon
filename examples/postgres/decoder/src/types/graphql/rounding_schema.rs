use juniper::GraphQLEnum;


#[derive(Debug, Clone, GraphQLEnum)]
#[graphql(name = "Rounding")]
pub enum RoundingGraphQL {
        Up,
        Down,
}

impl From<crate::types::Rounding> for RoundingGraphQL {
    fn from(original: crate::types::Rounding) -> Self {
        match original {
            crate::types::Rounding::Up => Self::Up,
            crate::types::Rounding::Down => Self::Down,
        }
    }
}


