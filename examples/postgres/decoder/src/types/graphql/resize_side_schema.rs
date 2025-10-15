use juniper::GraphQLEnum;


/// Side of resize, 0 for lower and 1 for upper
#[derive(Debug, Clone, GraphQLEnum)]
#[graphql(name = "ResizeSide")]
pub enum ResizeSideGraphQL {
        Lower,
        Upper,
}

impl From<crate::types::ResizeSide> for ResizeSideGraphQL {
    fn from(original: crate::types::ResizeSide) -> Self {
        match original {
            crate::types::ResizeSide::Lower => Self::Lower,
            crate::types::ResizeSide::Upper => Self::Upper,
        }
    }
}


