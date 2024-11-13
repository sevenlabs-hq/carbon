use super::JupiterSwapDecoder;
pub mod aldrin_swap;
pub mod aldrin_v2_swap;
pub mod balansol_swap;
pub mod claim;
pub mod claim_token;
pub mod clone_swap;
pub mod create_open_orders;
pub mod create_program_open_orders;
pub mod create_token_account;
pub mod create_token_ledger;
pub mod crema_swap;
pub mod cropper_swap;
pub mod cykura_swap;
pub mod deltafi_swap;
pub mod dradex_swap;
pub mod exact_out_route;
pub mod fee_event;
pub mod goosefx_swap;
pub mod goosefx_v2_swap;
pub mod helium_treasury_management_redeem_v0;
pub mod invariant_swap;
pub mod lifinity_swap;
pub mod lifinity_v2_swap;
pub mod marco_polo_swap;
pub mod marinade_deposit;
pub mod marinade_unstake;
pub mod mercurial_swap;
pub mod meteora_dlmm_swap;
pub mod meteora_swap;
pub mod moonshot_wrapped_buy;
pub mod moonshot_wrapped_sell;
pub mod obric_swap;
pub mod one_intro_swap;
pub mod open_book_v2_swap;
pub mod perps_add_liquidity;
pub mod perps_remove_liquidity;
pub mod perps_swap;
pub mod perps_v2_add_liquidity;
pub mod perps_v2_remove_liquidity;
pub mod perps_v2_swap;
pub mod phoenix_swap;
pub mod pumpdotfun_wrapped_buy;
pub mod pumpdotfun_wrapped_sell;
pub mod raydium_clmm_swap;
pub mod raydium_clmm_swap_v2;
pub mod raydium_cp_swap;
pub mod raydium_swap;
pub mod route;
pub mod route_with_token_ledger;
pub mod saber_add_decimals;
pub mod saber_swap;
pub mod sencha_swap;
pub mod serum_swap;
pub mod set_token_ledger;
pub mod shared_accounts_exact_out_route;
pub mod shared_accounts_route;
pub mod shared_accounts_route_with_token_ledger;
pub mod stabble_stable_swap;
pub mod stabble_weighted_swap;
pub mod step_swap;
pub mod swap_event;
pub mod symmetry_swap;
pub mod token_swap;
pub mod token_swap_v2;
pub mod whirlpool_swap;
pub mod whirlpool_swap_v2;

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
pub enum JupiterSwapInstruction {
    Route(route::Route),
    RouteWithTokenLedger(route_with_token_ledger::RouteWithTokenLedger),
    ExactOutRoute(exact_out_route::ExactOutRoute),
    SharedAccountsRoute(shared_accounts_route::SharedAccountsRoute),
    SharedAccountsRouteWithTokenLedger(
        shared_accounts_route_with_token_ledger::SharedAccountsRouteWithTokenLedger,
    ),
    SharedAccountsExactOutRoute(shared_accounts_exact_out_route::SharedAccountsExactOutRoute),
    SetTokenLedger(set_token_ledger::SetTokenLedger),
    CreateOpenOrders(create_open_orders::CreateOpenOrders),
    CreateTokenAccount(create_token_account::CreateTokenAccount),
    CreateProgramOpenOrders(create_program_open_orders::CreateProgramOpenOrders),
    Claim(claim::Claim),
    ClaimToken(claim_token::ClaimToken),
    CreateTokenLedger(create_token_ledger::CreateTokenLedger),
    MercurialSwap(mercurial_swap::MercurialSwap),
    CykuraSwap(cykura_swap::CykuraSwap),
    SerumSwap(serum_swap::SerumSwap),
    SaberSwap(saber_swap::SaberSwap),
    SaberAddDecimals(saber_add_decimals::SaberAddDecimals),
    TokenSwap(token_swap::TokenSwap),
    TokenSwapV2(token_swap_v2::TokenSwapV2),
    SenchaSwap(sencha_swap::SenchaSwap),
    StepSwap(step_swap::StepSwap),
    CropperSwap(cropper_swap::CropperSwap),
    RaydiumSwap(raydium_swap::RaydiumSwap),
    CremaSwap(crema_swap::CremaSwap),
    LifinitySwap(lifinity_swap::LifinitySwap),
    MarinadeDeposit(marinade_deposit::MarinadeDeposit),
    MarinadeUnstake(marinade_unstake::MarinadeUnstake),
    AldrinSwap(aldrin_swap::AldrinSwap),
    AldrinV2Swap(aldrin_v2_swap::AldrinV2Swap),
    WhirlpoolSwap(whirlpool_swap::WhirlpoolSwap),
    WhirlpoolSwapV2(whirlpool_swap_v2::WhirlpoolSwapV2),
    InvariantSwap(invariant_swap::InvariantSwap),
    MeteoraSwap(meteora_swap::MeteoraSwap),
    GoosefxSwap(goosefx_swap::GoosefxSwap),
    DeltafiSwap(deltafi_swap::DeltafiSwap),
    BalansolSwap(balansol_swap::BalansolSwap),
    MarcoPoloSwap(marco_polo_swap::MarcoPoloSwap),
    DradexSwap(dradex_swap::DradexSwap),
    LifinityV2Swap(lifinity_v2_swap::LifinityV2Swap),
    RaydiumClmmSwap(raydium_clmm_swap::RaydiumClmmSwap),
    RaydiumClmmSwapV2(raydium_clmm_swap_v2::RaydiumClmmSwapV2),
    PhoenixSwap(phoenix_swap::PhoenixSwap),
    SymmetrySwap(symmetry_swap::SymmetrySwap),
    HeliumTreasuryManagementRedeemV0(
        helium_treasury_management_redeem_v0::HeliumTreasuryManagementRedeemV0,
    ),
    GoosefxV2Swap(goosefx_v2_swap::GoosefxV2Swap),
    PerpsSwap(perps_swap::PerpsSwap),
    PerpsAddLiquidity(perps_add_liquidity::PerpsAddLiquidity),
    PerpsRemoveLiquidity(perps_remove_liquidity::PerpsRemoveLiquidity),
    MeteoraDlmmSwap(meteora_dlmm_swap::MeteoraDlmmSwap),
    OpenBookV2Swap(open_book_v2_swap::OpenBookV2Swap),
    CloneSwap(clone_swap::CloneSwap),
    RaydiumCpSwap(raydium_cp_swap::RaydiumCpSwap),
    OneIntroSwap(one_intro_swap::OneIntroSwap),
    PumpdotfunWrappedBuy(pumpdotfun_wrapped_buy::PumpdotfunWrappedBuy),
    PumpdotfunWrappedSell(pumpdotfun_wrapped_sell::PumpdotfunWrappedSell),
    PerpsV2Swap(perps_v2_swap::PerpsV2Swap),
    PerpsV2AddLiquidity(perps_v2_add_liquidity::PerpsV2AddLiquidity),
    PerpsV2RemoveLiquidity(perps_v2_remove_liquidity::PerpsV2RemoveLiquidity),
    MoonshotWrappedBuy(moonshot_wrapped_buy::MoonshotWrappedBuy),
    MoonshotWrappedSell(moonshot_wrapped_sell::MoonshotWrappedSell),
    StabbleStableSwap(stabble_stable_swap::StabbleStableSwap),
    StabbleWeightedSwap(stabble_weighted_swap::StabbleWeightedSwap),
    ObricSwap(obric_swap::ObricSwap),
    SwapEvent(swap_event::SwapEvent),
    FeeEvent(fee_event::FeeEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for JupiterSwapDecoder {
    type InstructionType = JupiterSwapInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            JupiterSwapInstruction::Route => route::Route,
            JupiterSwapInstruction::RouteWithTokenLedger => route_with_token_ledger::RouteWithTokenLedger,
            JupiterSwapInstruction::ExactOutRoute => exact_out_route::ExactOutRoute,
            JupiterSwapInstruction::SharedAccountsRoute => shared_accounts_route::SharedAccountsRoute,
            JupiterSwapInstruction::SharedAccountsRouteWithTokenLedger => shared_accounts_route_with_token_ledger::SharedAccountsRouteWithTokenLedger,
            JupiterSwapInstruction::SharedAccountsExactOutRoute => shared_accounts_exact_out_route::SharedAccountsExactOutRoute,
            JupiterSwapInstruction::SetTokenLedger => set_token_ledger::SetTokenLedger,
            JupiterSwapInstruction::CreateOpenOrders => create_open_orders::CreateOpenOrders,
            JupiterSwapInstruction::CreateTokenAccount => create_token_account::CreateTokenAccount,
            JupiterSwapInstruction::CreateProgramOpenOrders => create_program_open_orders::CreateProgramOpenOrders,
            JupiterSwapInstruction::Claim => claim::Claim,
            JupiterSwapInstruction::ClaimToken => claim_token::ClaimToken,
            JupiterSwapInstruction::CreateTokenLedger => create_token_ledger::CreateTokenLedger,
            JupiterSwapInstruction::MercurialSwap => mercurial_swap::MercurialSwap,
            JupiterSwapInstruction::CykuraSwap => cykura_swap::CykuraSwap,
            JupiterSwapInstruction::SerumSwap => serum_swap::SerumSwap,
            JupiterSwapInstruction::SaberSwap => saber_swap::SaberSwap,
            JupiterSwapInstruction::SaberAddDecimals => saber_add_decimals::SaberAddDecimals,
            JupiterSwapInstruction::TokenSwap => token_swap::TokenSwap,
            JupiterSwapInstruction::TokenSwapV2 => token_swap_v2::TokenSwapV2,
            JupiterSwapInstruction::SenchaSwap => sencha_swap::SenchaSwap,
            JupiterSwapInstruction::StepSwap => step_swap::StepSwap,
            JupiterSwapInstruction::CropperSwap => cropper_swap::CropperSwap,
            JupiterSwapInstruction::RaydiumSwap => raydium_swap::RaydiumSwap,
            JupiterSwapInstruction::CremaSwap => crema_swap::CremaSwap,
            JupiterSwapInstruction::LifinitySwap => lifinity_swap::LifinitySwap,
            JupiterSwapInstruction::MarinadeDeposit => marinade_deposit::MarinadeDeposit,
            JupiterSwapInstruction::MarinadeUnstake => marinade_unstake::MarinadeUnstake,
            JupiterSwapInstruction::AldrinSwap => aldrin_swap::AldrinSwap,
            JupiterSwapInstruction::AldrinV2Swap => aldrin_v2_swap::AldrinV2Swap,
            JupiterSwapInstruction::WhirlpoolSwap => whirlpool_swap::WhirlpoolSwap,
            JupiterSwapInstruction::WhirlpoolSwapV2 => whirlpool_swap_v2::WhirlpoolSwapV2,
            JupiterSwapInstruction::InvariantSwap => invariant_swap::InvariantSwap,
            JupiterSwapInstruction::MeteoraSwap => meteora_swap::MeteoraSwap,
            JupiterSwapInstruction::GoosefxSwap => goosefx_swap::GoosefxSwap,
            JupiterSwapInstruction::DeltafiSwap => deltafi_swap::DeltafiSwap,
            JupiterSwapInstruction::BalansolSwap => balansol_swap::BalansolSwap,
            JupiterSwapInstruction::MarcoPoloSwap => marco_polo_swap::MarcoPoloSwap,
            JupiterSwapInstruction::DradexSwap => dradex_swap::DradexSwap,
            JupiterSwapInstruction::LifinityV2Swap => lifinity_v2_swap::LifinityV2Swap,
            JupiterSwapInstruction::RaydiumClmmSwap => raydium_clmm_swap::RaydiumClmmSwap,
            JupiterSwapInstruction::RaydiumClmmSwapV2 => raydium_clmm_swap_v2::RaydiumClmmSwapV2,
            JupiterSwapInstruction::PhoenixSwap => phoenix_swap::PhoenixSwap,
            JupiterSwapInstruction::SymmetrySwap => symmetry_swap::SymmetrySwap,
            JupiterSwapInstruction::HeliumTreasuryManagementRedeemV0 => helium_treasury_management_redeem_v0::HeliumTreasuryManagementRedeemV0,
            JupiterSwapInstruction::GoosefxV2Swap => goosefx_v2_swap::GoosefxV2Swap,
            JupiterSwapInstruction::PerpsSwap => perps_swap::PerpsSwap,
            JupiterSwapInstruction::PerpsAddLiquidity => perps_add_liquidity::PerpsAddLiquidity,
            JupiterSwapInstruction::PerpsRemoveLiquidity => perps_remove_liquidity::PerpsRemoveLiquidity,
            JupiterSwapInstruction::MeteoraDlmmSwap => meteora_dlmm_swap::MeteoraDlmmSwap,
            JupiterSwapInstruction::OpenBookV2Swap => open_book_v2_swap::OpenBookV2Swap,
            JupiterSwapInstruction::CloneSwap => clone_swap::CloneSwap,
            JupiterSwapInstruction::RaydiumCpSwap => raydium_cp_swap::RaydiumCpSwap,
            JupiterSwapInstruction::OneIntroSwap => one_intro_swap::OneIntroSwap,
            JupiterSwapInstruction::PumpdotfunWrappedBuy => pumpdotfun_wrapped_buy::PumpdotfunWrappedBuy,
            JupiterSwapInstruction::PumpdotfunWrappedSell => pumpdotfun_wrapped_sell::PumpdotfunWrappedSell,
            JupiterSwapInstruction::PerpsV2Swap => perps_v2_swap::PerpsV2Swap,
            JupiterSwapInstruction::PerpsV2AddLiquidity => perps_v2_add_liquidity::PerpsV2AddLiquidity,
            JupiterSwapInstruction::PerpsV2RemoveLiquidity => perps_v2_remove_liquidity::PerpsV2RemoveLiquidity,
            JupiterSwapInstruction::MoonshotWrappedBuy => moonshot_wrapped_buy::MoonshotWrappedBuy,
            JupiterSwapInstruction::MoonshotWrappedSell => moonshot_wrapped_sell::MoonshotWrappedSell,
            JupiterSwapInstruction::StabbleStableSwap => stabble_stable_swap::StabbleStableSwap,
            JupiterSwapInstruction::StabbleWeightedSwap => stabble_weighted_swap::StabbleWeightedSwap,
            JupiterSwapInstruction::ObricSwap => obric_swap::ObricSwap,
            JupiterSwapInstruction::SwapEvent => swap_event::SwapEvent,
            JupiterSwapInstruction::FeeEvent => fee_event::FeeEvent,
        )
    }
}
