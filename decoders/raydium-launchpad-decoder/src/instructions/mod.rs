use {super::RaydiumLaunchpadDecoder, crate::PROGRAM_ID};
pub mod buy_exact_in;
pub mod buy_exact_out;
pub mod claim_platform_fee;
pub mod claim_vested_event;
pub mod claim_vested_token;
pub mod collect_fee;
pub mod collect_migrate_fee;
pub mod create_config;
pub mod create_platform_config;
pub mod create_vesting_account;
pub mod create_vesting_event;
pub mod initialize;
pub mod migrate_to_amm;
pub mod migrate_to_cpswap;
pub mod pool_create_event;
pub mod sell_exact_in;
pub mod sell_exact_out;
pub mod trade_event;
pub mod update_config;
pub mod update_platform_config;

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
    ClaimPlatformFee(claim_platform_fee::ClaimPlatformFee),
    ClaimVestedToken(claim_vested_token::ClaimVestedToken),
    CollectFee(collect_fee::CollectFee),
    CollectMigrateFee(collect_migrate_fee::CollectMigrateFee),
    CreateConfig(create_config::CreateConfig),
    CreatePlatformConfig(create_platform_config::CreatePlatformConfig),
    CreateVestingAccount(create_vesting_account::CreateVestingAccount),
    Initialize(initialize::Initialize),
    MigrateToAmm(migrate_to_amm::MigrateToAmm),
    MigrateToCpswap(migrate_to_cpswap::MigrateToCpswap),
    SellExactIn(sell_exact_in::SellExactIn),
    SellExactOut(sell_exact_out::SellExactOut),
    UpdateConfig(update_config::UpdateConfig),
    UpdatePlatformConfig(update_platform_config::UpdatePlatformConfig),
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
            RaydiumLaunchpadInstruction::ClaimPlatformFee => claim_platform_fee::ClaimPlatformFee,
            RaydiumLaunchpadInstruction::ClaimVestedToken => claim_vested_token::ClaimVestedToken,
            RaydiumLaunchpadInstruction::CollectFee => collect_fee::CollectFee,
            RaydiumLaunchpadInstruction::CollectMigrateFee => collect_migrate_fee::CollectMigrateFee,
            RaydiumLaunchpadInstruction::CreateConfig => create_config::CreateConfig,
            RaydiumLaunchpadInstruction::CreatePlatformConfig => create_platform_config::CreatePlatformConfig,
            RaydiumLaunchpadInstruction::CreateVestingAccount => create_vesting_account::CreateVestingAccount,
            RaydiumLaunchpadInstruction::Initialize => initialize::Initialize,
            RaydiumLaunchpadInstruction::MigrateToAmm => migrate_to_amm::MigrateToAmm,
            RaydiumLaunchpadInstruction::MigrateToCpswap => migrate_to_cpswap::MigrateToCpswap,
            RaydiumLaunchpadInstruction::SellExactIn => sell_exact_in::SellExactIn,
            RaydiumLaunchpadInstruction::SellExactOut => sell_exact_out::SellExactOut,
            RaydiumLaunchpadInstruction::UpdateConfig => update_config::UpdateConfig,
            RaydiumLaunchpadInstruction::UpdatePlatformConfig => update_platform_config::UpdatePlatformConfig,
            RaydiumLaunchpadInstruction::ClaimVestedEvent => claim_vested_event::ClaimVestedEvent,
            RaydiumLaunchpadInstruction::CreateVestingEvent => create_vesting_event::CreateVestingEvent,
            RaydiumLaunchpadInstruction::PoolCreateEvent => pool_create_event::PoolCreateEvent,
            RaydiumLaunchpadInstruction::TradeEvent => trade_event::TradeEvent,
        )
    }
}
