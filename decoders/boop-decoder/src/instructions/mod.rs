use crate::PROGRAM_ID;

use super::BoopDecoder;
pub mod add_operators;
pub mod authority_transfer_cancelled_event;
pub mod authority_transfer_completed_event;
pub mod authority_transfer_initiated_event;
pub mod bonding_curve_deployed_event;
pub mod bonding_curve_deployed_fallback_event;
pub mod bonding_curve_vault_closed_event;
pub mod buy_token;
pub mod cancel_authority_transfer;
pub mod close_bonding_curve_vault;
pub mod collect_meteora_trading_fees;
pub mod collect_trading_fees;
pub mod complete_authority_transfer;
pub mod config_updated_event;
pub mod create_meteora_pool;
pub mod create_raydium_pool;
pub mod create_raydium_random_pool;
pub mod create_token;
pub mod create_token_fallback;
pub mod deploy_bonding_curve;
pub mod deploy_bonding_curve_fallback;
pub mod deposit_into_raydium;
pub mod graduate;
pub mod initialize;
pub mod initiate_authority_transfer;
pub mod liquidity_deposited_into_raydium_event;
pub mod lock_raydium_liquidity;
pub mod operators_added_event;
pub mod operators_removed_event;
pub mod paused_toggled_event;
pub mod raydium_liquidity_locked_event;
pub mod raydium_pool_created_event;
pub mod raydium_random_pool_created_event;
pub mod remove_operators;
pub mod sell_token;
pub mod split_trading_fees;
pub mod swap_sol_for_tokens_on_raydium;
pub mod swap_sol_for_tokens_on_raydium_event;
pub mod swap_tokens_for_sol_on_raydium;
pub mod swap_tokens_for_sol_on_raydium_event;
pub mod toggle_paused;
pub mod token_bought_event;
pub mod token_created_event;
pub mod token_created_fallback_event;
pub mod token_graduated_event;
pub mod token_sold_event;
pub mod trading_fees_collected_event;
pub mod trading_fees_split_event;
pub mod update_config;

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
pub enum BoopInstruction {
    AddOperators(add_operators::AddOperators),
    BuyToken(buy_token::BuyToken),
    CancelAuthorityTransfer(cancel_authority_transfer::CancelAuthorityTransfer),
    CloseBondingCurveVault(close_bonding_curve_vault::CloseBondingCurveVault),
    CollectMeteoraTradingFees(collect_meteora_trading_fees::CollectMeteoraTradingFees),
    CollectTradingFees(collect_trading_fees::CollectTradingFees),
    CompleteAuthorityTransfer(complete_authority_transfer::CompleteAuthorityTransfer),
    CreateMeteoraPool(create_meteora_pool::CreateMeteoraPool),
    CreateRaydiumPool(create_raydium_pool::CreateRaydiumPool),
    CreateRaydiumRandomPool(create_raydium_random_pool::CreateRaydiumRandomPool),
    CreateToken(create_token::CreateToken),
    CreateTokenFallback(create_token_fallback::CreateTokenFallback),
    DeployBondingCurve(deploy_bonding_curve::DeployBondingCurve),
    DeployBondingCurveFallback(deploy_bonding_curve_fallback::DeployBondingCurveFallback),
    DepositIntoRaydium(deposit_into_raydium::DepositIntoRaydium),
    Graduate(graduate::Graduate),
    Initialize(initialize::Initialize),
    InitiateAuthorityTransfer(initiate_authority_transfer::InitiateAuthorityTransfer),
    LockRaydiumLiquidity(lock_raydium_liquidity::LockRaydiumLiquidity),
    RemoveOperators(remove_operators::RemoveOperators),
    SellToken(sell_token::SellToken),
    SplitTradingFees(split_trading_fees::SplitTradingFees),
    SwapSolForTokensOnRaydium(swap_sol_for_tokens_on_raydium::SwapSolForTokensOnRaydium),
    SwapTokensForSolOnRaydium(swap_tokens_for_sol_on_raydium::SwapTokensForSolOnRaydium),
    TogglePaused(toggle_paused::TogglePaused),
    UpdateConfig(update_config::UpdateConfig),
    AuthorityTransferCancelledEvent(
        authority_transfer_cancelled_event::AuthorityTransferCancelledEvent,
    ),
    AuthorityTransferCompletedEvent(
        authority_transfer_completed_event::AuthorityTransferCompletedEvent,
    ),
    AuthorityTransferInitiatedEvent(
        authority_transfer_initiated_event::AuthorityTransferInitiatedEvent,
    ),
    BondingCurveDeployedEvent(bonding_curve_deployed_event::BondingCurveDeployedEvent),
    BondingCurveDeployedFallbackEvent(
        bonding_curve_deployed_fallback_event::BondingCurveDeployedFallbackEvent,
    ),
    BondingCurveVaultClosedEvent(bonding_curve_vault_closed_event::BondingCurveVaultClosedEvent),
    ConfigUpdatedEvent(config_updated_event::ConfigUpdatedEvent),
    LiquidityDepositedIntoRaydiumEvent(
        liquidity_deposited_into_raydium_event::LiquidityDepositedIntoRaydiumEvent,
    ),
    OperatorsAddedEvent(operators_added_event::OperatorsAddedEvent),
    OperatorsRemovedEvent(operators_removed_event::OperatorsRemovedEvent),
    PausedToggledEvent(paused_toggled_event::PausedToggledEvent),
    RaydiumLiquidityLockedEvent(raydium_liquidity_locked_event::RaydiumLiquidityLockedEvent),
    RaydiumPoolCreatedEvent(raydium_pool_created_event::RaydiumPoolCreatedEvent),
    RaydiumRandomPoolCreatedEvent(raydium_random_pool_created_event::RaydiumRandomPoolCreatedEvent),
    SwapSolForTokensOnRaydiumEvent(
        swap_sol_for_tokens_on_raydium_event::SwapSolForTokensOnRaydiumEvent,
    ),
    SwapTokensForSolOnRaydiumEvent(
        swap_tokens_for_sol_on_raydium_event::SwapTokensForSolOnRaydiumEvent,
    ),
    TokenBoughtEvent(token_bought_event::TokenBoughtEvent),
    TokenCreatedEvent(token_created_event::TokenCreatedEvent),
    TokenCreatedFallbackEvent(token_created_fallback_event::TokenCreatedFallbackEvent),
    TokenGraduatedEvent(token_graduated_event::TokenGraduatedEvent),
    TokenSoldEvent(token_sold_event::TokenSoldEvent),
    TradingFeesCollectedEvent(trading_fees_collected_event::TradingFeesCollectedEvent),
    TradingFeesSplitEvent(trading_fees_split_event::TradingFeesSplitEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for BoopDecoder {
    type InstructionType = BoopInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            BoopInstruction::AddOperators => add_operators::AddOperators,
            BoopInstruction::BuyToken => buy_token::BuyToken,
            BoopInstruction::CancelAuthorityTransfer => cancel_authority_transfer::CancelAuthorityTransfer,
            BoopInstruction::CloseBondingCurveVault => close_bonding_curve_vault::CloseBondingCurveVault,
            BoopInstruction::CollectMeteoraTradingFees => collect_meteora_trading_fees::CollectMeteoraTradingFees,
            BoopInstruction::CollectTradingFees => collect_trading_fees::CollectTradingFees,
            BoopInstruction::CompleteAuthorityTransfer => complete_authority_transfer::CompleteAuthorityTransfer,
            BoopInstruction::CreateMeteoraPool => create_meteora_pool::CreateMeteoraPool,
            BoopInstruction::CreateRaydiumPool => create_raydium_pool::CreateRaydiumPool,
            BoopInstruction::CreateRaydiumRandomPool => create_raydium_random_pool::CreateRaydiumRandomPool,
            BoopInstruction::CreateToken => create_token::CreateToken,
            BoopInstruction::CreateTokenFallback => create_token_fallback::CreateTokenFallback,
            BoopInstruction::DeployBondingCurve => deploy_bonding_curve::DeployBondingCurve,
            BoopInstruction::DeployBondingCurveFallback => deploy_bonding_curve_fallback::DeployBondingCurveFallback,
            BoopInstruction::DepositIntoRaydium => deposit_into_raydium::DepositIntoRaydium,
            BoopInstruction::Graduate => graduate::Graduate,
            BoopInstruction::Initialize => initialize::Initialize,
            BoopInstruction::InitiateAuthorityTransfer => initiate_authority_transfer::InitiateAuthorityTransfer,
            BoopInstruction::LockRaydiumLiquidity => lock_raydium_liquidity::LockRaydiumLiquidity,
            BoopInstruction::RemoveOperators => remove_operators::RemoveOperators,
            BoopInstruction::SellToken => sell_token::SellToken,
            BoopInstruction::SplitTradingFees => split_trading_fees::SplitTradingFees,
            BoopInstruction::SwapSolForTokensOnRaydium => swap_sol_for_tokens_on_raydium::SwapSolForTokensOnRaydium,
            BoopInstruction::SwapTokensForSolOnRaydium => swap_tokens_for_sol_on_raydium::SwapTokensForSolOnRaydium,
            BoopInstruction::TogglePaused => toggle_paused::TogglePaused,
            BoopInstruction::UpdateConfig => update_config::UpdateConfig,
            BoopInstruction::AuthorityTransferCancelledEvent => authority_transfer_cancelled_event::AuthorityTransferCancelledEvent,
            BoopInstruction::AuthorityTransferCompletedEvent => authority_transfer_completed_event::AuthorityTransferCompletedEvent,
            BoopInstruction::AuthorityTransferInitiatedEvent => authority_transfer_initiated_event::AuthorityTransferInitiatedEvent,
            BoopInstruction::BondingCurveDeployedEvent => bonding_curve_deployed_event::BondingCurveDeployedEvent,
            BoopInstruction::BondingCurveDeployedFallbackEvent => bonding_curve_deployed_fallback_event::BondingCurveDeployedFallbackEvent,
            BoopInstruction::BondingCurveVaultClosedEvent => bonding_curve_vault_closed_event::BondingCurveVaultClosedEvent,
            BoopInstruction::ConfigUpdatedEvent => config_updated_event::ConfigUpdatedEvent,
            BoopInstruction::LiquidityDepositedIntoRaydiumEvent => liquidity_deposited_into_raydium_event::LiquidityDepositedIntoRaydiumEvent,
            BoopInstruction::OperatorsAddedEvent => operators_added_event::OperatorsAddedEvent,
            BoopInstruction::OperatorsRemovedEvent => operators_removed_event::OperatorsRemovedEvent,
            BoopInstruction::PausedToggledEvent => paused_toggled_event::PausedToggledEvent,
            BoopInstruction::RaydiumLiquidityLockedEvent => raydium_liquidity_locked_event::RaydiumLiquidityLockedEvent,
            BoopInstruction::RaydiumPoolCreatedEvent => raydium_pool_created_event::RaydiumPoolCreatedEvent,
            BoopInstruction::RaydiumRandomPoolCreatedEvent => raydium_random_pool_created_event::RaydiumRandomPoolCreatedEvent,
            BoopInstruction::SwapSolForTokensOnRaydiumEvent => swap_sol_for_tokens_on_raydium_event::SwapSolForTokensOnRaydiumEvent,
            BoopInstruction::SwapTokensForSolOnRaydiumEvent => swap_tokens_for_sol_on_raydium_event::SwapTokensForSolOnRaydiumEvent,
            BoopInstruction::TokenBoughtEvent => token_bought_event::TokenBoughtEvent,
            BoopInstruction::TokenCreatedEvent => token_created_event::TokenCreatedEvent,
            BoopInstruction::TokenCreatedFallbackEvent => token_created_fallback_event::TokenCreatedFallbackEvent,
            BoopInstruction::TokenGraduatedEvent => token_graduated_event::TokenGraduatedEvent,
            BoopInstruction::TokenSoldEvent => token_sold_event::TokenSoldEvent,
            BoopInstruction::TradingFeesCollectedEvent => trading_fees_collected_event::TradingFeesCollectedEvent,
            BoopInstruction::TradingFeesSplitEvent => trading_fees_split_event::TradingFeesSplitEvent,
        )
    }
}
