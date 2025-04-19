use crate::PROGRAM_ID;

use super::VirtualCurveDecoder;
pub mod claim_protocol_fee;
pub mod claim_trading_fee;
pub mod close_claim_fee_operator;
pub mod create_claim_fee_operator;
pub mod create_config;
pub mod create_locker;
pub mod create_partner_metadata;
pub mod create_virtual_pool_metadata;
pub mod evt_claim_protocol_fee_event;
pub mod evt_claim_trading_fee_event;
pub mod evt_close_claim_fee_operator_event;
pub mod evt_create_claim_fee_operator_event;
pub mod evt_create_config_event;
pub mod evt_create_damm_v2_migration_metadata_event;
pub mod evt_create_meteora_migration_metadata_event;
pub mod evt_curve_complete_event;
pub mod evt_initialize_pool_event;
pub mod evt_partner_metadata_event;
pub mod evt_partner_withdraw_surplus_event;
pub mod evt_protocol_withdraw_surplus_event;
pub mod evt_swap_event;
pub mod evt_virtual_pool_metadata_event;
pub mod initialize_virtual_pool_with_spl_token;
pub mod initialize_virtual_pool_with_token2022;
pub mod migrate_meteora_damm;
pub mod migrate_meteora_damm_claim_lp_token;
pub mod migrate_meteora_damm_lock_lp_token;
pub mod migration_damm_v2;
pub mod migration_damm_v2_create_metadata;
pub mod migration_meteora_damm_create_metadata;
pub mod partner_withdraw_surplus;
pub mod protocol_withdraw_surplus;
pub mod swap;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum VirtualCurveInstruction {
    ClaimProtocolFee(claim_protocol_fee::ClaimProtocolFee),
    ClaimTradingFee(claim_trading_fee::ClaimTradingFee),
    CloseClaimFeeOperator(close_claim_fee_operator::CloseClaimFeeOperator),
    CreateClaimFeeOperator(create_claim_fee_operator::CreateClaimFeeOperator),
    CreateConfig(create_config::CreateConfig),
    CreateLocker(create_locker::CreateLocker),
    CreatePartnerMetadata(create_partner_metadata::CreatePartnerMetadata),
    CreateVirtualPoolMetadata(create_virtual_pool_metadata::CreateVirtualPoolMetadata),
    InitializeVirtualPoolWithSplToken(
        initialize_virtual_pool_with_spl_token::InitializeVirtualPoolWithSplToken,
    ),
    InitializeVirtualPoolWithToken2022(
        initialize_virtual_pool_with_token2022::InitializeVirtualPoolWithToken2022,
    ),
    MigrateMeteoraDamm(migrate_meteora_damm::MigrateMeteoraDamm),
    MigrateMeteoraDammClaimLpToken(
        migrate_meteora_damm_claim_lp_token::MigrateMeteoraDammClaimLpToken,
    ),
    MigrateMeteoraDammLockLpToken(
        migrate_meteora_damm_lock_lp_token::MigrateMeteoraDammLockLpToken,
    ),
    MigrationDammV2(migration_damm_v2::MigrationDammV2),
    MigrationDammV2CreateMetadata(migration_damm_v2_create_metadata::MigrationDammV2CreateMetadata),
    MigrationMeteoraDammCreateMetadata(
        migration_meteora_damm_create_metadata::MigrationMeteoraDammCreateMetadata,
    ),
    PartnerWithdrawSurplus(partner_withdraw_surplus::PartnerWithdrawSurplus),
    ProtocolWithdrawSurplus(protocol_withdraw_surplus::ProtocolWithdrawSurplus),
    Swap(swap::Swap),
    EvtClaimProtocolFeeEvent(evt_claim_protocol_fee_event::EvtClaimProtocolFeeEvent),
    EvtClaimTradingFeeEvent(evt_claim_trading_fee_event::EvtClaimTradingFeeEvent),
    EvtCloseClaimFeeOperatorEvent(
        evt_close_claim_fee_operator_event::EvtCloseClaimFeeOperatorEvent,
    ),
    EvtCreateClaimFeeOperatorEvent(
        evt_create_claim_fee_operator_event::EvtCreateClaimFeeOperatorEvent,
    ),
    EvtCreateConfigEvent(evt_create_config_event::EvtCreateConfigEvent),
    EvtCreateDammV2MigrationMetadataEvent(
        evt_create_damm_v2_migration_metadata_event::EvtCreateDammV2MigrationMetadataEvent,
    ),
    EvtCreateMeteoraMigrationMetadataEvent(
        evt_create_meteora_migration_metadata_event::EvtCreateMeteoraMigrationMetadataEvent,
    ),
    EvtCurveCompleteEvent(evt_curve_complete_event::EvtCurveCompleteEvent),
    EvtInitializePoolEvent(evt_initialize_pool_event::EvtInitializePoolEvent),
    EvtPartnerMetadataEvent(evt_partner_metadata_event::EvtPartnerMetadataEvent),
    EvtPartnerWithdrawSurplusEvent(
        evt_partner_withdraw_surplus_event::EvtPartnerWithdrawSurplusEvent,
    ),
    EvtProtocolWithdrawSurplusEvent(
        evt_protocol_withdraw_surplus_event::EvtProtocolWithdrawSurplusEvent,
    ),
    EvtSwapEvent(evt_swap_event::EvtSwapEvent),
    EvtVirtualPoolMetadataEvent(evt_virtual_pool_metadata_event::EvtVirtualPoolMetadataEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for VirtualCurveDecoder {
    type InstructionType = VirtualCurveInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }
        carbon_core::try_decode_instructions!(instruction,
            VirtualCurveInstruction::ClaimProtocolFee => claim_protocol_fee::ClaimProtocolFee,
            VirtualCurveInstruction::ClaimTradingFee => claim_trading_fee::ClaimTradingFee,
            VirtualCurveInstruction::CloseClaimFeeOperator => close_claim_fee_operator::CloseClaimFeeOperator,
            VirtualCurveInstruction::CreateClaimFeeOperator => create_claim_fee_operator::CreateClaimFeeOperator,
            VirtualCurveInstruction::CreateConfig => create_config::CreateConfig,
            VirtualCurveInstruction::CreateLocker => create_locker::CreateLocker,
            VirtualCurveInstruction::CreatePartnerMetadata => create_partner_metadata::CreatePartnerMetadata,
            VirtualCurveInstruction::CreateVirtualPoolMetadata => create_virtual_pool_metadata::CreateVirtualPoolMetadata,
            VirtualCurveInstruction::InitializeVirtualPoolWithSplToken => initialize_virtual_pool_with_spl_token::InitializeVirtualPoolWithSplToken,
            VirtualCurveInstruction::InitializeVirtualPoolWithToken2022 => initialize_virtual_pool_with_token2022::InitializeVirtualPoolWithToken2022,
            VirtualCurveInstruction::MigrateMeteoraDamm => migrate_meteora_damm::MigrateMeteoraDamm,
            VirtualCurveInstruction::MigrateMeteoraDammClaimLpToken => migrate_meteora_damm_claim_lp_token::MigrateMeteoraDammClaimLpToken,
            VirtualCurveInstruction::MigrateMeteoraDammLockLpToken => migrate_meteora_damm_lock_lp_token::MigrateMeteoraDammLockLpToken,
            VirtualCurveInstruction::MigrationDammV2 => migration_damm_v2::MigrationDammV2,
            VirtualCurveInstruction::MigrationDammV2CreateMetadata => migration_damm_v2_create_metadata::MigrationDammV2CreateMetadata,
            VirtualCurveInstruction::MigrationMeteoraDammCreateMetadata => migration_meteora_damm_create_metadata::MigrationMeteoraDammCreateMetadata,
            VirtualCurveInstruction::PartnerWithdrawSurplus => partner_withdraw_surplus::PartnerWithdrawSurplus,
            VirtualCurveInstruction::ProtocolWithdrawSurplus => protocol_withdraw_surplus::ProtocolWithdrawSurplus,
            VirtualCurveInstruction::Swap => swap::Swap,
            VirtualCurveInstruction::EvtClaimProtocolFeeEvent => evt_claim_protocol_fee_event::EvtClaimProtocolFeeEvent,
            VirtualCurveInstruction::EvtClaimTradingFeeEvent => evt_claim_trading_fee_event::EvtClaimTradingFeeEvent,
            VirtualCurveInstruction::EvtCloseClaimFeeOperatorEvent => evt_close_claim_fee_operator_event::EvtCloseClaimFeeOperatorEvent,
            VirtualCurveInstruction::EvtCreateClaimFeeOperatorEvent => evt_create_claim_fee_operator_event::EvtCreateClaimFeeOperatorEvent,
            VirtualCurveInstruction::EvtCreateConfigEvent => evt_create_config_event::EvtCreateConfigEvent,
            VirtualCurveInstruction::EvtCreateDammV2MigrationMetadataEvent => evt_create_damm_v2_migration_metadata_event::EvtCreateDammV2MigrationMetadataEvent,
            VirtualCurveInstruction::EvtCreateMeteoraMigrationMetadataEvent => evt_create_meteora_migration_metadata_event::EvtCreateMeteoraMigrationMetadataEvent,
            VirtualCurveInstruction::EvtCurveCompleteEvent => evt_curve_complete_event::EvtCurveCompleteEvent,
            VirtualCurveInstruction::EvtInitializePoolEvent => evt_initialize_pool_event::EvtInitializePoolEvent,
            VirtualCurveInstruction::EvtPartnerMetadataEvent => evt_partner_metadata_event::EvtPartnerMetadataEvent,
            VirtualCurveInstruction::EvtPartnerWithdrawSurplusEvent => evt_partner_withdraw_surplus_event::EvtPartnerWithdrawSurplusEvent,
            VirtualCurveInstruction::EvtProtocolWithdrawSurplusEvent => evt_protocol_withdraw_surplus_event::EvtProtocolWithdrawSurplusEvent,
            VirtualCurveInstruction::EvtSwapEvent => evt_swap_event::EvtSwapEvent,
            VirtualCurveInstruction::EvtVirtualPoolMetadataEvent => evt_virtual_pool_metadata_event::EvtVirtualPoolMetadataEvent,
        )
    }
}
