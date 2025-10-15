use juniper::GraphQLEnum;


/// Pair status. 0 = Enabled, 1 = Disabled. Putting 0 as enabled for backward compatibility.
#[derive(Debug, Clone, GraphQLEnum)]
#[graphql(name = "PairStatus")]
pub enum PairStatusGraphQL {
        Enabled,
        Disabled,
}

impl From<crate::types::PairStatus> for PairStatusGraphQL {
    fn from(original: crate::types::PairStatus) -> Self {
        match original {
            crate::types::PairStatus::Enabled => Self::Enabled,
            crate::types::PairStatus::Disabled => Self::Disabled,
        }
    }
}


