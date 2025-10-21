use juniper::GraphQLEnum;

#[derive(Debug, Clone, GraphQLEnum)]
#[graphql(name = "FarmType")]
pub enum FarmTypeGraphQL {
    Single,
    Dual,
    Triple,
}

impl From<crate::types::FarmType> for FarmTypeGraphQL {
    fn from(original: crate::types::FarmType) -> Self {
        match original {
            crate::types::FarmType::Single => Self::Single,
            crate::types::FarmType::Dual => Self::Dual,
            crate::types::FarmType::Triple => Self::Triple,
        }
    }
}
