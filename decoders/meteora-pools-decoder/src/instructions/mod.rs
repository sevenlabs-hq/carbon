use {super::MeteoraPoolsDecoder, crate::PROGRAM_ID};
pub mod add_balance_liquidity;
pub mod add_imbalance_liquidity;
pub mod add_liquidity_event;
pub mod bootstrap_liquidity;
pub mod bootstrap_liquidity_event;
pub mod claim_fee;
pub mod claim_fee_event;
pub mod close_config;
pub mod close_config_event;
pub mod create_config;
pub mod create_config_event;
pub mod create_lock_escrow;
pub mod create_lock_escrow_event;
pub mod create_mint_metadata;
pub mod enable_or_disable_pool;
pub mod get_pool_info;
pub mod initialize_customizable_permissionless_constant_product_pool;
pub mod initialize_permissioned_pool;
pub mod initialize_permissionless_constant_product_pool_with_config;
pub mod initialize_permissionless_constant_product_pool_with_config2;
pub mod initialize_permissionless_pool;
pub mod initialize_permissionless_pool_with_fee_tier;
pub mod lock;
pub mod lock_event;
pub mod migrate_fee_account_event;
pub mod override_curve_param;
pub mod override_curve_param_event;
pub mod partner_claim_fee;
pub mod partner_claim_fees_event;
pub mod pool_created_event;
pub mod pool_enabled_event;
pub mod pool_info_event;
pub mod remove_balance_liquidity;
pub mod remove_liquidity_event;
pub mod remove_liquidity_single_side;
pub mod set_pool_fees;
pub mod set_pool_fees_event;
pub mod set_whitelisted_vault;
pub mod swap;
pub mod swap_event;
pub mod transfer_admin_event;
pub mod update_activation_point;
pub mod withdraw_protocol_fees;
pub mod withdraw_protocol_fees_event;

#[derive(
    carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone,
)]
pub enum MeteoraPoolsProgramInstruction {
    InitializePermissionedPool(initialize_permissioned_pool::InitializePermissionedPool),
    InitializePermissionlessPool(initialize_permissionless_pool::InitializePermissionlessPool),
    InitializePermissionlessPoolWithFeeTier(initialize_permissionless_pool_with_fee_tier::InitializePermissionlessPoolWithFeeTier),
    EnableOrDisablePool(enable_or_disable_pool::EnableOrDisablePool),
    Swap(swap::Swap),
    RemoveLiquiditySingleSide(remove_liquidity_single_side::RemoveLiquiditySingleSide),
    AddImbalanceLiquidity(add_imbalance_liquidity::AddImbalanceLiquidity),
    RemoveBalanceLiquidity(remove_balance_liquidity::RemoveBalanceLiquidity),
    AddBalanceLiquidity(add_balance_liquidity::AddBalanceLiquidity),
    SetPoolFees(set_pool_fees::SetPoolFees),
    OverrideCurveParam(override_curve_param::OverrideCurveParam),
    GetPoolInfo(get_pool_info::GetPoolInfo),
    BootstrapLiquidity(bootstrap_liquidity::BootstrapLiquidity),
    CreateMintMetadata(create_mint_metadata::CreateMintMetadata),
    CreateLockEscrow(create_lock_escrow::CreateLockEscrow),
    Lock(lock::Lock),
    ClaimFee(claim_fee::ClaimFee),
    CreateConfig(create_config::CreateConfig),
    CloseConfig(close_config::CloseConfig),
    InitializePermissionlessConstantProductPoolWithConfig(initialize_permissionless_constant_product_pool_with_config::InitializePermissionlessConstantProductPoolWithConfig),
    InitializePermissionlessConstantProductPoolWithConfig2(initialize_permissionless_constant_product_pool_with_config2::InitializePermissionlessConstantProductPoolWithConfig2),
    InitializeCustomizablePermissionlessConstantProductPool(initialize_customizable_permissionless_constant_product_pool::InitializeCustomizablePermissionlessConstantProductPool),
    UpdateActivationPoint(update_activation_point::UpdateActivationPoint),
    WithdrawProtocolFees(withdraw_protocol_fees::WithdrawProtocolFees),
    SetWhitelistedVault(set_whitelisted_vault::SetWhitelistedVault),
    PartnerClaimFee(partner_claim_fee::PartnerClaimFee),
    AddLiquidityEvent(add_liquidity_event::AddLiquidityEvent),
    RemoveLiquidityEvent(remove_liquidity_event::RemoveLiquidityEvent),
    BootstrapLiquidityEvent(bootstrap_liquidity_event::BootstrapLiquidityEvent),
    SwapEvent(swap_event::SwapEvent),
    SetPoolFeesEvent(set_pool_fees_event::SetPoolFeesEvent),
    PoolInfoEvent(pool_info_event::PoolInfoEvent),
    TransferAdminEvent(transfer_admin_event::TransferAdminEvent),
    OverrideCurveParamEvent(override_curve_param_event::OverrideCurveParamEvent),
    PoolCreatedEvent(pool_created_event::PoolCreatedEvent),
    PoolEnabledEvent(pool_enabled_event::PoolEnabledEvent),
    MigrateFeeAccountEvent(migrate_fee_account_event::MigrateFeeAccountEvent),
    CreateLockEscrowEvent(create_lock_escrow_event::CreateLockEscrowEvent),
    LockEvent(lock_event::LockEvent),
    ClaimFeeEvent(claim_fee_event::ClaimFeeEvent),
    CreateConfigEvent(create_config_event::CreateConfigEvent),
    CloseConfigEvent(close_config_event::CloseConfigEvent),
    WithdrawProtocolFeesEvent(withdraw_protocol_fees_event::WithdrawProtocolFeesEvent),
    PartnerClaimFeesEvent(partner_claim_fees_event::PartnerClaimFeesEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for MeteoraPoolsDecoder {
    type InstructionType = MeteoraPoolsProgramInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            MeteoraPoolsProgramInstruction::InitializePermissionedPool => initialize_permissioned_pool::InitializePermissionedPool,
            MeteoraPoolsProgramInstruction::InitializePermissionlessPool => initialize_permissionless_pool::InitializePermissionlessPool,
            MeteoraPoolsProgramInstruction::InitializePermissionlessPoolWithFeeTier => initialize_permissionless_pool_with_fee_tier::InitializePermissionlessPoolWithFeeTier,
            MeteoraPoolsProgramInstruction::EnableOrDisablePool => enable_or_disable_pool::EnableOrDisablePool,
            MeteoraPoolsProgramInstruction::Swap => swap::Swap,
            MeteoraPoolsProgramInstruction::RemoveLiquiditySingleSide => remove_liquidity_single_side::RemoveLiquiditySingleSide,
            MeteoraPoolsProgramInstruction::AddImbalanceLiquidity => add_imbalance_liquidity::AddImbalanceLiquidity,
            MeteoraPoolsProgramInstruction::RemoveBalanceLiquidity => remove_balance_liquidity::RemoveBalanceLiquidity,
            MeteoraPoolsProgramInstruction::AddBalanceLiquidity => add_balance_liquidity::AddBalanceLiquidity,
            MeteoraPoolsProgramInstruction::SetPoolFees => set_pool_fees::SetPoolFees,
            MeteoraPoolsProgramInstruction::OverrideCurveParam => override_curve_param::OverrideCurveParam,
            MeteoraPoolsProgramInstruction::GetPoolInfo => get_pool_info::GetPoolInfo,
            MeteoraPoolsProgramInstruction::BootstrapLiquidity => bootstrap_liquidity::BootstrapLiquidity,
            MeteoraPoolsProgramInstruction::CreateMintMetadata => create_mint_metadata::CreateMintMetadata,
            MeteoraPoolsProgramInstruction::CreateLockEscrow => create_lock_escrow::CreateLockEscrow,
            MeteoraPoolsProgramInstruction::Lock => lock::Lock,
            MeteoraPoolsProgramInstruction::ClaimFee => claim_fee::ClaimFee,
            MeteoraPoolsProgramInstruction::CreateConfig => create_config::CreateConfig,
            MeteoraPoolsProgramInstruction::CloseConfig => close_config::CloseConfig,
            MeteoraPoolsProgramInstruction::InitializePermissionlessConstantProductPoolWithConfig => initialize_permissionless_constant_product_pool_with_config::InitializePermissionlessConstantProductPoolWithConfig,
            MeteoraPoolsProgramInstruction::InitializePermissionlessConstantProductPoolWithConfig2 => initialize_permissionless_constant_product_pool_with_config2::InitializePermissionlessConstantProductPoolWithConfig2,
            MeteoraPoolsProgramInstruction::InitializeCustomizablePermissionlessConstantProductPool => initialize_customizable_permissionless_constant_product_pool::InitializeCustomizablePermissionlessConstantProductPool,
            MeteoraPoolsProgramInstruction::UpdateActivationPoint => update_activation_point::UpdateActivationPoint,
            MeteoraPoolsProgramInstruction::WithdrawProtocolFees => withdraw_protocol_fees::WithdrawProtocolFees,
            MeteoraPoolsProgramInstruction::SetWhitelistedVault => set_whitelisted_vault::SetWhitelistedVault,
            MeteoraPoolsProgramInstruction::PartnerClaimFee => partner_claim_fee::PartnerClaimFee,
            MeteoraPoolsProgramInstruction::AddLiquidityEvent => add_liquidity_event::AddLiquidityEvent,
            MeteoraPoolsProgramInstruction::RemoveLiquidityEvent => remove_liquidity_event::RemoveLiquidityEvent,
            MeteoraPoolsProgramInstruction::BootstrapLiquidityEvent => bootstrap_liquidity_event::BootstrapLiquidityEvent,
            MeteoraPoolsProgramInstruction::SwapEvent => swap_event::SwapEvent,
            MeteoraPoolsProgramInstruction::SetPoolFeesEvent => set_pool_fees_event::SetPoolFeesEvent,
            MeteoraPoolsProgramInstruction::PoolInfoEvent => pool_info_event::PoolInfoEvent,
            MeteoraPoolsProgramInstruction::TransferAdminEvent => transfer_admin_event::TransferAdminEvent,
            MeteoraPoolsProgramInstruction::OverrideCurveParamEvent => override_curve_param_event::OverrideCurveParamEvent,
            MeteoraPoolsProgramInstruction::PoolCreatedEvent => pool_created_event::PoolCreatedEvent,
            MeteoraPoolsProgramInstruction::PoolEnabledEvent => pool_enabled_event::PoolEnabledEvent,
            MeteoraPoolsProgramInstruction::MigrateFeeAccountEvent => migrate_fee_account_event::MigrateFeeAccountEvent,
            MeteoraPoolsProgramInstruction::CreateLockEscrowEvent => create_lock_escrow_event::CreateLockEscrowEvent,
            MeteoraPoolsProgramInstruction::LockEvent => lock_event::LockEvent,
            MeteoraPoolsProgramInstruction::ClaimFeeEvent => claim_fee_event::ClaimFeeEvent,
            MeteoraPoolsProgramInstruction::CreateConfigEvent => create_config_event::CreateConfigEvent,
            MeteoraPoolsProgramInstruction::CloseConfigEvent => close_config_event::CloseConfigEvent,
            MeteoraPoolsProgramInstruction::WithdrawProtocolFeesEvent => withdraw_protocol_fees_event::WithdrawProtocolFeesEvent,
            MeteoraPoolsProgramInstruction::PartnerClaimFeesEvent => partner_claim_fees_event::PartnerClaimFeesEvent,
        )
    }
}
