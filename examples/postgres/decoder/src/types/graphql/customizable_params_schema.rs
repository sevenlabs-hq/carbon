use juniper::GraphQLObject;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "CustomizableParams")]
pub struct CustomizableParamsGraphQL {
            /// Pool price
            pub active_id: i32,
            /// Bin step
            pub bin_step: i32,
            /// Base factor
            pub base_factor: i32,
            /// Activation type. 0 = Slot, 1 = Time. Check ActivationType enum
            pub activation_type: U8,
            /// Whether the pool has an alpha vault
            pub has_alpha_vault: bool,
            /// Decide when does the pool start trade. None = Now
            pub activation_point: Option<U64>,
            /// Pool creator have permission to enable/disable pool with restricted program validation. Only applicable for customizable permissionless pool.
            pub creator_pool_on_off_control: bool,
            /// Base fee power factor
            pub base_fee_power_factor: U8,
            /// Padding, for future use
            pub padding: Vec<U8>,
}

impl From<crate::types::CustomizableParams> for CustomizableParamsGraphQL {
    fn from(original: crate::types::CustomizableParams) -> Self {
        Self {
            active_id: original.active_id,
            bin_step: original.bin_step as i32,
            base_factor: original.base_factor as i32,
            activation_type: carbon_core::graphql::primitives::U8(original.activation_type),
            has_alpha_vault: original.has_alpha_vault,
            activation_point: original.activation_point.map(|v| carbon_core::graphql::primitives::U64(v)),
            creator_pool_on_off_control: original.creator_pool_on_off_control,
            base_fee_power_factor: carbon_core::graphql::primitives::U8(original.base_fee_power_factor),
            padding: original.padding.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
        }
    }
}