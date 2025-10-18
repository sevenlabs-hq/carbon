use crate::PROGRAM_ID;

use super::RaydiumLaunchpadDecoder;
pub mod buy_exact_in;
pub mod buy_exact_out;
pub mod claim_creator_fee;
pub mod claim_platform_fee;
pub mod claim_platform_fee_from_vault;
pub mod claim_vested_event;
pub mod claim_vested_token;
pub mod collect_fee;
pub mod collect_migrate_fee;
pub mod create_config;
pub mod create_platform_config;
pub mod create_vesting_account;
pub mod create_vesting_event;
pub mod initialize;
pub mod initialize_v2;
pub mod initialize_with_token_2022;
pub mod migrate_to_amm;
pub mod migrate_to_cpswap;
pub mod pool_create_event;
pub mod remove_platform_curve_param;
pub mod sell_exact_in;
pub mod sell_exact_out;
pub mod trade_event;
pub mod update_config;
pub mod update_platform_config;
pub mod update_platform_curve_param;

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
pub enum RaydiumLaunchpadInstruction {
    BuyExactIn(buy_exact_in::BuyExactIn),
    BuyExactOut(buy_exact_out::BuyExactOut),
    ClaimCreatorFee(claim_creator_fee::ClaimCreatorFee),
    ClaimPlatformFee(claim_platform_fee::ClaimPlatformFee),
    ClaimPlatformFeeFromVault(claim_platform_fee_from_vault::ClaimPlatformFeeFromVault),
    ClaimVestedToken(claim_vested_token::ClaimVestedToken),
    CollectFee(collect_fee::CollectFee),
    CollectMigrateFee(collect_migrate_fee::CollectMigrateFee),
    CreateConfig(create_config::CreateConfig),
    CreatePlatformConfig(create_platform_config::CreatePlatformConfig),
    CreateVestingAccount(create_vesting_account::CreateVestingAccount),
    Initialize(initialize::Initialize),
    InitializeV2(initialize_v2::InitializeV2),
    InitializeWithToken2022(initialize_with_token_2022::InitializeWithToken2022),
    MigrateToAmm(migrate_to_amm::MigrateToAmm),
    MigrateToCpswap(migrate_to_cpswap::MigrateToCpswap),
    RemovePlatformCurveParam(remove_platform_curve_param::RemovePlatformCurveParam),
    SellExactIn(sell_exact_in::SellExactIn),
    SellExactOut(sell_exact_out::SellExactOut),
    UpdateConfig(update_config::UpdateConfig),
    UpdatePlatformConfig(update_platform_config::UpdatePlatformConfig),
    UpdatePlatformCurveParam(update_platform_curve_param::UpdatePlatformCurveParam),
    ClaimVestedEvent(claim_vested_event::ClaimVestedEvent),
    CreateVestingEvent(create_vesting_event::CreateVestingEvent),
    PoolCreateEvent(pool_create_event::PoolCreateEvent),
    TradeEvent(trade_event::TradeEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for RaydiumLaunchpadDecoder {
    type InstructionType = RaydiumLaunchpadInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            RaydiumLaunchpadInstruction::BuyExactIn => buy_exact_in::BuyExactIn,
            RaydiumLaunchpadInstruction::BuyExactOut => buy_exact_out::BuyExactOut,
            RaydiumLaunchpadInstruction::ClaimCreatorFee => claim_creator_fee::ClaimCreatorFee,
            RaydiumLaunchpadInstruction::ClaimPlatformFee => claim_platform_fee::ClaimPlatformFee,
            RaydiumLaunchpadInstruction::ClaimPlatformFeeFromVault => claim_platform_fee_from_vault::ClaimPlatformFeeFromVault,
            RaydiumLaunchpadInstruction::ClaimVestedToken => claim_vested_token::ClaimVestedToken,
            RaydiumLaunchpadInstruction::CollectFee => collect_fee::CollectFee,
            RaydiumLaunchpadInstruction::CollectMigrateFee => collect_migrate_fee::CollectMigrateFee,
            RaydiumLaunchpadInstruction::CreateConfig => create_config::CreateConfig,
            RaydiumLaunchpadInstruction::CreatePlatformConfig => create_platform_config::CreatePlatformConfig,
            RaydiumLaunchpadInstruction::CreateVestingAccount => create_vesting_account::CreateVestingAccount,
            RaydiumLaunchpadInstruction::Initialize => initialize::Initialize,
            RaydiumLaunchpadInstruction::InitializeV2 => initialize_v2::InitializeV2,
            RaydiumLaunchpadInstruction::InitializeWithToken2022 => initialize_with_token_2022::InitializeWithToken2022,
            RaydiumLaunchpadInstruction::MigrateToAmm => migrate_to_amm::MigrateToAmm,
            RaydiumLaunchpadInstruction::MigrateToCpswap => migrate_to_cpswap::MigrateToCpswap,
            RaydiumLaunchpadInstruction::RemovePlatformCurveParam => remove_platform_curve_param::RemovePlatformCurveParam,
            RaydiumLaunchpadInstruction::SellExactIn => sell_exact_in::SellExactIn,
            RaydiumLaunchpadInstruction::SellExactOut => sell_exact_out::SellExactOut,
            RaydiumLaunchpadInstruction::UpdateConfig => update_config::UpdateConfig,
            RaydiumLaunchpadInstruction::UpdatePlatformConfig => update_platform_config::UpdatePlatformConfig,
            RaydiumLaunchpadInstruction::UpdatePlatformCurveParam => update_platform_curve_param::UpdatePlatformCurveParam,
            RaydiumLaunchpadInstruction::ClaimVestedEvent => claim_vested_event::ClaimVestedEvent,
            RaydiumLaunchpadInstruction::CreateVestingEvent => create_vesting_event::CreateVestingEvent,
            RaydiumLaunchpadInstruction::PoolCreateEvent => pool_create_event::PoolCreateEvent,
            RaydiumLaunchpadInstruction::TradeEvent => trade_event::TradeEvent,
        )
    }
}
