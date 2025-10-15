use juniper::GraphQLObject;
use carbon_core::graphql::primitives::I64;
use carbon_core::graphql::primitives::Pubkey;
use carbon_core::graphql::primitives::U64;
use carbon_core::graphql::primitives::U8;
use crate::types::graphql::ProtocolFeeGraphQL;
use crate::types::graphql::RewardInfoGraphQL;
use crate::types::graphql::StaticParametersGraphQL;
use crate::types::graphql::VariableParametersGraphQL;

#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "LbPair")]
pub struct LbPairGraphQL {
    pub metadata: crate::accounts::graphql::AccountMetadataGraphQL,
        pub parameters: StaticParametersGraphQL,
        pub v_parameters: VariableParametersGraphQL,
        pub bump_seed: Vec<U8>,
            /// Bin step signer seed
            pub bin_step_seed: Vec<U8>,
            /// Type of the pair
            pub pair_type: U8,
            /// Active bin id
            pub active_id: i32,
            /// Bin step. Represent the price increment / decrement.
            pub bin_step: i32,
            /// Status of the pair. Check PairStatus enum.
            pub status: U8,
            /// Require base factor seed
            pub require_base_factor_seed: U8,
            /// Base factor seed
            pub base_factor_seed: Vec<U8>,
            /// Activation type
            pub activation_type: U8,
            /// Allow pool creator to enable/disable pool with restricted validation. Only applicable for customizable permissionless pair type.
            pub creator_pool_on_off_control: U8,
            /// Token X mint
            pub token_x_mint: Pubkey,
            /// Token Y mint
            pub token_y_mint: Pubkey,
            /// LB token X vault
            pub reserve_x: Pubkey,
            /// LB token Y vault
            pub reserve_y: Pubkey,
            /// Uncollected protocol fee
            pub protocol_fee: ProtocolFeeGraphQL,
            /// _padding_1, previous Fee owner, BE CAREFUL FOR TOMBSTONE WHEN REUSE !!
            pub padding1: Vec<U8>,
            /// Farming reward information
            pub reward_infos: Vec<RewardInfoGraphQL>,
            /// Oracle pubkey
            pub oracle: Pubkey,
            /// Packed initialized bin array state
            pub bin_array_bitmap: Vec<U64>,
            /// Last time the pool fee parameter was updated
            pub last_updated_at: I64,
            /// _padding_2, previous whitelisted_wallet, BE CAREFUL FOR TOMBSTONE WHEN REUSE !!
            pub padding2: Vec<U8>,
            /// Address allowed to swap when the current point is greater than or equal to the pre-activation point. The pre-activation point is calculated as `activation_point - pre_activation_duration`.
            pub pre_activation_swap_address: Pubkey,
            /// Base keypair. Only required for permission pair
            pub base_key: Pubkey,
            /// Time point to enable the pair. Only applicable for permission pair.
            pub activation_point: U64,
            /// Duration before activation activation_point. Used to calculate pre-activation time point for pre_activation_swap_address
            pub pre_activation_duration: U64,
            /// _padding 3 is reclaimed free space from swap_cap_deactivate_point and swap_cap_amount before, BE CAREFUL FOR TOMBSTONE WHEN REUSE !!
            pub padding3: Vec<U8>,
            /// _padding_4, previous lock_duration, BE CAREFUL FOR TOMBSTONE WHEN REUSE !!
            pub padding4: U64,
            /// Pool creator
            pub creator: Pubkey,
            /// token_mint_x_program_flag
            pub token_mint_x_program_flag: U8,
            /// token_mint_y_program_flag
            pub token_mint_y_program_flag: U8,
            /// Reserved space for future use
            pub reserved: Vec<U8>,
}

impl TryFrom<crate::accounts::postgres::LbPairRow> for LbPairGraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::accounts::postgres::LbPairRow) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: row.metadata.into(),
            parameters: row.parameters.0.into(),
            v_parameters: row.v_parameters.0.into(),
            bump_seed: row.bump_seed.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
            bin_step_seed: row.bin_step_seed.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
            pair_type: carbon_core::graphql::primitives::U8((*row.pair_type) as u8),
            active_id: row.active_id,
            bin_step: (*row.bin_step) as i32,
            status: carbon_core::graphql::primitives::U8((*row.status) as u8),
            require_base_factor_seed: carbon_core::graphql::primitives::U8((*row.require_base_factor_seed) as u8),
            base_factor_seed: row.base_factor_seed.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
            activation_type: carbon_core::graphql::primitives::U8((*row.activation_type) as u8),
            creator_pool_on_off_control: carbon_core::graphql::primitives::U8((*row.creator_pool_on_off_control) as u8),
            token_x_mint: carbon_core::graphql::primitives::Pubkey(*row.token_x_mint),
            token_y_mint: carbon_core::graphql::primitives::Pubkey(*row.token_y_mint),
            reserve_x: carbon_core::graphql::primitives::Pubkey(*row.reserve_x),
            reserve_y: carbon_core::graphql::primitives::Pubkey(*row.reserve_y),
            protocol_fee: row.protocol_fee.0.into(),
            padding1: row.padding1.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
            reward_infos: row.reward_infos.0.into_iter().map(|item| item.into()).collect(),
            oracle: carbon_core::graphql::primitives::Pubkey(*row.oracle),
            bin_array_bitmap: row.bin_array_bitmap.into_iter().map(|item| carbon_core::graphql::primitives::U64(*item)).collect(),
            last_updated_at: carbon_core::graphql::primitives::I64(row.last_updated_at),
            padding2: row.padding2.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
            pre_activation_swap_address: carbon_core::graphql::primitives::Pubkey(*row.pre_activation_swap_address),
            base_key: carbon_core::graphql::primitives::Pubkey(*row.base_key),
            activation_point: carbon_core::graphql::primitives::U64(*row.activation_point),
            pre_activation_duration: carbon_core::graphql::primitives::U64(*row.pre_activation_duration),
            padding3: row.padding3.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
            padding4: carbon_core::graphql::primitives::U64(*row.padding4),
            creator: carbon_core::graphql::primitives::Pubkey(*row.creator),
            token_mint_x_program_flag: carbon_core::graphql::primitives::U8((*row.token_mint_x_program_flag) as u8),
            token_mint_y_program_flag: carbon_core::graphql::primitives::U8((*row.token_mint_y_program_flag) as u8),
            reserved: row.reserved.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect(),
        })
    }
}