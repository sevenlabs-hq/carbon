use {super::DynamicBondingCurveDecoder, crate::PROGRAM_ID};
pub mod claim_creator_trading_fee;
pub mod claim_protocol_fee;
pub mod claim_trading_fee;
pub mod close_claim_fee_operator;
pub mod create_claim_fee_operator;
pub mod create_config;
pub mod create_locker;
pub mod create_partner_metadata;
pub mod create_virtual_pool_metadata;
pub mod creator_withdraw_surplus;
pub mod evt_claim_creator_trading_fee_event;
pub mod evt_claim_protocol_fee_event;
pub mod evt_claim_trading_fee_event;
pub mod evt_close_claim_fee_operator_event;
pub mod evt_create_claim_fee_operator_event;
pub mod evt_create_config_event;
pub mod evt_create_config_v2_event;
pub mod evt_create_damm_v2_migration_metadata_event;
pub mod evt_create_meteora_migration_metadata_event;
pub mod evt_creator_withdraw_surplus_event;
pub mod evt_curve_complete_event;
pub mod evt_initialize_pool_event;
pub mod evt_partner_metadata_event;
pub mod evt_partner_withdraw_migration_fee_event;
pub mod evt_partner_withdraw_surplus_event;
pub mod evt_protocol_withdraw_surplus_event;
pub mod evt_swap2_event;
pub mod evt_swap_event;
pub mod evt_update_pool_creator_event;
pub mod evt_virtual_pool_metadata_event;
pub mod evt_withdraw_leftover_event;
pub mod evt_withdraw_migration_fee_event;
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
pub mod swap2;
pub mod transfer_pool_creator;
pub mod withdraw_leftover;
pub mod withdraw_migration_fee;

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
pub enum DynamicBondingCurveInstruction {
    ClaimCreatorTradingFee(claim_creator_trading_fee::ClaimCreatorTradingFee),
    ClaimProtocolFee(claim_protocol_fee::ClaimProtocolFee),
    ClaimTradingFee(claim_trading_fee::ClaimTradingFee),
    CloseClaimFeeOperator(close_claim_fee_operator::CloseClaimFeeOperator),
    CreateClaimFeeOperator(create_claim_fee_operator::CreateClaimFeeOperator),
    CreateConfig(create_config::CreateConfig),
    CreateLocker(create_locker::CreateLocker),
    CreatePartnerMetadata(create_partner_metadata::CreatePartnerMetadata),
    CreateVirtualPoolMetadata(create_virtual_pool_metadata::CreateVirtualPoolMetadata),
    CreatorWithdrawSurplus(creator_withdraw_surplus::CreatorWithdrawSurplus),
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
    Swap2(swap2::Swap2),
    TransferPoolCreator(transfer_pool_creator::TransferPoolCreator),
    WithdrawLeftover(withdraw_leftover::WithdrawLeftover),
    WithdrawMigrationFee(withdraw_migration_fee::WithdrawMigrationFee),
    EvtClaimCreatorTradingFeeEvent(
        evt_claim_creator_trading_fee_event::EvtClaimCreatorTradingFeeEvent,
    ),
    EvtClaimProtocolFeeEvent(evt_claim_protocol_fee_event::EvtClaimProtocolFeeEvent),
    EvtClaimTradingFeeEvent(evt_claim_trading_fee_event::EvtClaimTradingFeeEvent),
    EvtCloseClaimFeeOperatorEvent(
        evt_close_claim_fee_operator_event::EvtCloseClaimFeeOperatorEvent,
    ),
    EvtCreateClaimFeeOperatorEvent(
        evt_create_claim_fee_operator_event::EvtCreateClaimFeeOperatorEvent,
    ),
    EvtCreateConfigEvent(evt_create_config_event::EvtCreateConfigEvent),
    EvtCreateConfigV2Event(evt_create_config_v2_event::EvtCreateConfigV2Event),
    EvtCreateDammV2MigrationMetadataEvent(
        evt_create_damm_v2_migration_metadata_event::EvtCreateDammV2MigrationMetadataEvent,
    ),
    EvtCreateMeteoraMigrationMetadataEvent(
        evt_create_meteora_migration_metadata_event::EvtCreateMeteoraMigrationMetadataEvent,
    ),
    EvtCreatorWithdrawSurplusEvent(
        evt_creator_withdraw_surplus_event::EvtCreatorWithdrawSurplusEvent,
    ),
    EvtCurveCompleteEvent(evt_curve_complete_event::EvtCurveCompleteEvent),
    EvtInitializePoolEvent(evt_initialize_pool_event::EvtInitializePoolEvent),
    EvtPartnerMetadataEvent(evt_partner_metadata_event::EvtPartnerMetadataEvent),
    EvtPartnerWithdrawMigrationFeeEvent(
        evt_partner_withdraw_migration_fee_event::EvtPartnerWithdrawMigrationFeeEvent,
    ),
    EvtPartnerWithdrawSurplusEvent(
        evt_partner_withdraw_surplus_event::EvtPartnerWithdrawSurplusEvent,
    ),
    EvtProtocolWithdrawSurplusEvent(
        evt_protocol_withdraw_surplus_event::EvtProtocolWithdrawSurplusEvent,
    ),
    EvtSwapEvent(evt_swap_event::EvtSwapEvent),
    EvtSwap2Event(evt_swap2_event::EvtSwap2Event),
    EvtUpdatePoolCreatorEvent(evt_update_pool_creator_event::EvtUpdatePoolCreatorEvent),
    EvtVirtualPoolMetadataEvent(evt_virtual_pool_metadata_event::EvtVirtualPoolMetadataEvent),
    EvtWithdrawLeftoverEvent(evt_withdraw_leftover_event::EvtWithdrawLeftoverEvent),
    EvtWithdrawMigrationFeeEvent(evt_withdraw_migration_fee_event::EvtWithdrawMigrationFeeEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for DynamicBondingCurveDecoder {
    type InstructionType = DynamicBondingCurveInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            DynamicBondingCurveInstruction::ClaimCreatorTradingFee => claim_creator_trading_fee::ClaimCreatorTradingFee,
            DynamicBondingCurveInstruction::ClaimProtocolFee => claim_protocol_fee::ClaimProtocolFee,
            DynamicBondingCurveInstruction::ClaimTradingFee => claim_trading_fee::ClaimTradingFee,
            DynamicBondingCurveInstruction::CloseClaimFeeOperator => close_claim_fee_operator::CloseClaimFeeOperator,
            DynamicBondingCurveInstruction::CreateClaimFeeOperator => create_claim_fee_operator::CreateClaimFeeOperator,
            DynamicBondingCurveInstruction::CreateConfig => create_config::CreateConfig,
            DynamicBondingCurveInstruction::CreateLocker => create_locker::CreateLocker,
            DynamicBondingCurveInstruction::CreatePartnerMetadata => create_partner_metadata::CreatePartnerMetadata,
            DynamicBondingCurveInstruction::CreateVirtualPoolMetadata => create_virtual_pool_metadata::CreateVirtualPoolMetadata,
            DynamicBondingCurveInstruction::CreatorWithdrawSurplus => creator_withdraw_surplus::CreatorWithdrawSurplus,
            DynamicBondingCurveInstruction::InitializeVirtualPoolWithSplToken => initialize_virtual_pool_with_spl_token::InitializeVirtualPoolWithSplToken,
            DynamicBondingCurveInstruction::InitializeVirtualPoolWithToken2022 => initialize_virtual_pool_with_token2022::InitializeVirtualPoolWithToken2022,
            DynamicBondingCurveInstruction::MigrateMeteoraDamm => migrate_meteora_damm::MigrateMeteoraDamm,
            DynamicBondingCurveInstruction::MigrateMeteoraDammClaimLpToken => migrate_meteora_damm_claim_lp_token::MigrateMeteoraDammClaimLpToken,
            DynamicBondingCurveInstruction::MigrateMeteoraDammLockLpToken => migrate_meteora_damm_lock_lp_token::MigrateMeteoraDammLockLpToken,
            DynamicBondingCurveInstruction::MigrationDammV2 => migration_damm_v2::MigrationDammV2,
            DynamicBondingCurveInstruction::MigrationDammV2CreateMetadata => migration_damm_v2_create_metadata::MigrationDammV2CreateMetadata,
            DynamicBondingCurveInstruction::MigrationMeteoraDammCreateMetadata => migration_meteora_damm_create_metadata::MigrationMeteoraDammCreateMetadata,
            DynamicBondingCurveInstruction::PartnerWithdrawSurplus => partner_withdraw_surplus::PartnerWithdrawSurplus,
            DynamicBondingCurveInstruction::ProtocolWithdrawSurplus => protocol_withdraw_surplus::ProtocolWithdrawSurplus,
            DynamicBondingCurveInstruction::Swap => swap::Swap,
            DynamicBondingCurveInstruction::Swap2 => swap2::Swap2,
            DynamicBondingCurveInstruction::TransferPoolCreator => transfer_pool_creator::TransferPoolCreator,
            DynamicBondingCurveInstruction::WithdrawLeftover => withdraw_leftover::WithdrawLeftover,
            DynamicBondingCurveInstruction::WithdrawMigrationFee => withdraw_migration_fee::WithdrawMigrationFee,
            DynamicBondingCurveInstruction::EvtClaimCreatorTradingFeeEvent => evt_claim_creator_trading_fee_event::EvtClaimCreatorTradingFeeEvent,
            DynamicBondingCurveInstruction::EvtClaimProtocolFeeEvent => evt_claim_protocol_fee_event::EvtClaimProtocolFeeEvent,
            DynamicBondingCurveInstruction::EvtClaimTradingFeeEvent => evt_claim_trading_fee_event::EvtClaimTradingFeeEvent,
            DynamicBondingCurveInstruction::EvtCloseClaimFeeOperatorEvent => evt_close_claim_fee_operator_event::EvtCloseClaimFeeOperatorEvent,
            DynamicBondingCurveInstruction::EvtCreateClaimFeeOperatorEvent => evt_create_claim_fee_operator_event::EvtCreateClaimFeeOperatorEvent,
            DynamicBondingCurveInstruction::EvtCreateConfigEvent => evt_create_config_event::EvtCreateConfigEvent,
            DynamicBondingCurveInstruction::EvtCreateConfigV2Event => evt_create_config_v2_event::EvtCreateConfigV2Event,
            DynamicBondingCurveInstruction::EvtCreateDammV2MigrationMetadataEvent => evt_create_damm_v2_migration_metadata_event::EvtCreateDammV2MigrationMetadataEvent,
            DynamicBondingCurveInstruction::EvtCreateMeteoraMigrationMetadataEvent => evt_create_meteora_migration_metadata_event::EvtCreateMeteoraMigrationMetadataEvent,
            DynamicBondingCurveInstruction::EvtCreatorWithdrawSurplusEvent => evt_creator_withdraw_surplus_event::EvtCreatorWithdrawSurplusEvent,
            DynamicBondingCurveInstruction::EvtCurveCompleteEvent => evt_curve_complete_event::EvtCurveCompleteEvent,
            DynamicBondingCurveInstruction::EvtInitializePoolEvent => evt_initialize_pool_event::EvtInitializePoolEvent,
            DynamicBondingCurveInstruction::EvtPartnerMetadataEvent => evt_partner_metadata_event::EvtPartnerMetadataEvent,
            DynamicBondingCurveInstruction::EvtPartnerWithdrawMigrationFeeEvent => evt_partner_withdraw_migration_fee_event::EvtPartnerWithdrawMigrationFeeEvent,
            DynamicBondingCurveInstruction::EvtPartnerWithdrawSurplusEvent => evt_partner_withdraw_surplus_event::EvtPartnerWithdrawSurplusEvent,
            DynamicBondingCurveInstruction::EvtProtocolWithdrawSurplusEvent => evt_protocol_withdraw_surplus_event::EvtProtocolWithdrawSurplusEvent,
            DynamicBondingCurveInstruction::EvtSwapEvent => evt_swap_event::EvtSwapEvent,
            DynamicBondingCurveInstruction::EvtSwap2Event => evt_swap2_event::EvtSwap2Event,
            DynamicBondingCurveInstruction::EvtUpdatePoolCreatorEvent => evt_update_pool_creator_event::EvtUpdatePoolCreatorEvent,
            DynamicBondingCurveInstruction::EvtVirtualPoolMetadataEvent => evt_virtual_pool_metadata_event::EvtVirtualPoolMetadataEvent,
            DynamicBondingCurveInstruction::EvtWithdrawLeftoverEvent => evt_withdraw_leftover_event::EvtWithdrawLeftoverEvent,
            DynamicBondingCurveInstruction::EvtWithdrawMigrationFeeEvent => evt_withdraw_migration_fee_event::EvtWithdrawMigrationFeeEvent,
        )
    }
}
