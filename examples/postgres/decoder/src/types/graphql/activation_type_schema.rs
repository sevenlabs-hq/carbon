use juniper::GraphQLEnum;


/// Type of the activation
#[derive(Debug, Clone, GraphQLEnum)]
#[graphql(name = "ActivationType")]
pub enum ActivationTypeGraphQL {
        Slot,
        Timestamp,
}

impl From<crate::types::ActivationType> for ActivationTypeGraphQL {
    fn from(original: crate::types::ActivationType) -> Self {
        match original {
            crate::types::ActivationType::Slot => Self::Slot,
            crate::types::ActivationType::Timestamp => Self::Timestamp,
        }
    }
}


