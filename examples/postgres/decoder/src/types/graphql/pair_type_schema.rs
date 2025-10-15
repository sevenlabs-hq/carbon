use juniper::GraphQLEnum;


/// Type of the Pair. 0 = Permissionless, 1 = Permission, 2 = CustomizablePermissionless. Putting 0 as permissionless for backward compatibility.
#[derive(Debug, Clone, GraphQLEnum)]
#[graphql(name = "PairType")]
pub enum PairTypeGraphQL {
        Permissionless,
        Permission,
        CustomizablePermissionless,
        PermissionlessV2,
}

impl From<crate::types::PairType> for PairTypeGraphQL {
    fn from(original: crate::types::PairType) -> Self {
        match original {
            crate::types::PairType::Permissionless => Self::Permissionless,
            crate::types::PairType::Permission => Self::Permission,
            crate::types::PairType::CustomizablePermissionless => Self::CustomizablePermissionless,
            crate::types::PairType::PermissionlessV2 => Self::PermissionlessV2,
        }
    }
}


