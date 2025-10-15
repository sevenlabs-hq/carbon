pub mod add_liquidity_schema;
pub mod add_liquidity2_schema;
pub mod add_liquidity_by_strategy_schema;
pub mod add_liquidity_by_strategy2_schema;
pub mod add_liquidity_by_strategy_one_side_schema;
pub mod add_liquidity_by_weight_schema;
pub mod add_liquidity_one_side_schema;
pub mod add_liquidity_one_side_precise_schema;
pub mod add_liquidity_one_side_precise2_schema;
pub mod claim_fee2_schema;
pub mod claim_reward_schema;
pub mod claim_reward2_schema;
pub mod decrease_position_length_schema;
pub mod for_idl_type_generation_do_not_call_schema;
pub mod fund_reward_schema;
pub mod go_to_a_bin_schema;
pub mod increase_oracle_length_schema;
pub mod increase_position_length_schema;
pub mod initialize_bin_array_schema;
pub mod initialize_customizable_permissionless_lb_pair_schema;
pub mod initialize_customizable_permissionless_lb_pair2_schema;
pub mod initialize_lb_pair_schema;
pub mod initialize_lb_pair2_schema;
pub mod initialize_permission_lb_pair_schema;
pub mod initialize_position_schema;
pub mod initialize_position_by_operator_schema;
pub mod initialize_position_pda_schema;
pub mod initialize_preset_parameter_schema;
pub mod initialize_preset_parameter2_schema;
pub mod initialize_reward_schema;
pub mod rebalance_liquidity_schema;
pub mod remove_liquidity_schema;
pub mod remove_liquidity2_schema;
pub mod remove_liquidity_by_range_schema;
pub mod remove_liquidity_by_range2_schema;
pub mod set_activation_point_schema;
pub mod set_pair_status_schema;
pub mod set_pair_status_permissionless_schema;
pub mod set_pre_activation_duration_schema;
pub mod set_pre_activation_swap_address_schema;
pub mod swap_schema;
pub mod swap2_schema;
pub mod swap_exact_out_schema;
pub mod swap_exact_out2_schema;
pub mod swap_with_price_impact_schema;
pub mod swap_with_price_impact2_schema;
pub mod update_base_fee_parameters_schema;
pub mod update_dynamic_fee_parameters_schema;
pub mod update_fees_and_reward2_schema;
pub mod update_position_operator_schema;
pub mod update_reward_duration_schema;
pub mod update_reward_funder_schema;
pub mod withdraw_ineligible_reward_schema;
pub mod withdraw_protocol_fee_schema;
pub mod cpi_event_schema;

pub use add_liquidity_schema::*;
pub use add_liquidity2_schema::*;
pub use add_liquidity_by_strategy_schema::*;
pub use add_liquidity_by_strategy2_schema::*;
pub use add_liquidity_by_strategy_one_side_schema::*;
pub use add_liquidity_by_weight_schema::*;
pub use add_liquidity_one_side_schema::*;
pub use add_liquidity_one_side_precise_schema::*;
pub use add_liquidity_one_side_precise2_schema::*;
pub use claim_fee2_schema::*;
pub use claim_reward_schema::*;
pub use claim_reward2_schema::*;
pub use decrease_position_length_schema::*;
pub use for_idl_type_generation_do_not_call_schema::*;
pub use fund_reward_schema::*;
pub use go_to_a_bin_schema::*;
pub use increase_oracle_length_schema::*;
pub use increase_position_length_schema::*;
pub use initialize_bin_array_schema::*;
pub use initialize_customizable_permissionless_lb_pair_schema::*;
pub use initialize_customizable_permissionless_lb_pair2_schema::*;
pub use initialize_lb_pair_schema::*;
pub use initialize_lb_pair2_schema::*;
pub use initialize_permission_lb_pair_schema::*;
pub use initialize_position_schema::*;
pub use initialize_position_by_operator_schema::*;
pub use initialize_position_pda_schema::*;
pub use initialize_preset_parameter_schema::*;
pub use initialize_preset_parameter2_schema::*;
pub use initialize_reward_schema::*;
pub use rebalance_liquidity_schema::*;
pub use remove_liquidity_schema::*;
pub use remove_liquidity2_schema::*;
pub use remove_liquidity_by_range_schema::*;
pub use remove_liquidity_by_range2_schema::*;
pub use set_activation_point_schema::*;
pub use set_pair_status_schema::*;
pub use set_pair_status_permissionless_schema::*;
pub use set_pre_activation_duration_schema::*;
pub use set_pre_activation_swap_address_schema::*;
pub use swap_schema::*;
pub use swap2_schema::*;
pub use swap_exact_out_schema::*;
pub use swap_exact_out2_schema::*;
pub use swap_with_price_impact_schema::*;
pub use swap_with_price_impact2_schema::*;
pub use update_base_fee_parameters_schema::*;
pub use update_dynamic_fee_parameters_schema::*;
pub use update_fees_and_reward2_schema::*;
pub use update_position_operator_schema::*;
pub use update_reward_duration_schema::*;
pub use update_reward_funder_schema::*;
pub use withdraw_ineligible_reward_schema::*;
pub use withdraw_protocol_fee_schema::*;
pub use cpi_event_schema::*;

use juniper::GraphQLObject;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "InstructionMetadata")]
pub struct InstructionMetadataGraphQL {
    pub signature: String,
    pub instruction_index: carbon_core::graphql::primitives::U32,
    pub stack_height: carbon_core::graphql::primitives::U32,
    pub slot: Option<carbon_core::graphql::primitives::U64>,
}

impl From<carbon_core::postgres::metadata::InstructionRowMetadata> for InstructionMetadataGraphQL {
    fn from(metadata: carbon_core::postgres::metadata::InstructionRowMetadata) -> Self {
        Self {
            signature: metadata.signature,
            instruction_index: carbon_core::graphql::primitives::U32((*metadata.instruction_index) as u32),
            stack_height: carbon_core::graphql::primitives::U32((*metadata.stack_height) as u32),
            slot: metadata
                .slot
                .map(|v| carbon_core::graphql::primitives::U64(*v)),
        }
    }
}