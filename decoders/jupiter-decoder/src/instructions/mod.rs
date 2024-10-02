
use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;


use crate::JupiterDecoder;
pub mod route;
pub mod route_with_token_ledger;
pub mod exact_out_route;
pub mod shared_accounts_route;
pub mod shared_accounts_route_with_token_ledger;
pub mod shared_accounts_exact_out_route;
pub mod set_token_ledger;
pub mod create_open_orders;
pub mod create_token_account;
pub mod create_program_open_orders;
pub mod claim;
pub mod claim_token;
pub mod create_token_ledger;
pub mod mercurial_swap;
pub mod cykura_swap;
pub mod serum_swap;
pub mod saber_swap;
pub mod saber_add_decimals;
pub mod token_swap;
pub mod token_swap_v2;
pub mod sencha_swap;
pub mod step_swap;
pub mod cropper_swap;
pub mod raydium_swap;
pub mod crema_swap;
pub mod lifinity_swap;
pub mod marinade_deposit;
pub mod marinade_unstake;
pub mod aldrin_swap;
pub mod aldrin_v2_swap;
pub mod whirlpool_swap;
pub mod whirlpool_swap_v2;
pub mod invariant_swap;
pub mod meteora_swap;
pub mod goosefx_swap;
pub mod deltafi_swap;
pub mod balansol_swap;
pub mod marco_polo_swap;
pub mod dradex_swap;
pub mod lifinity_v2_swap;
pub mod raydium_clmm_swap;
pub mod raydium_clmm_swap_v2;
pub mod phoenix_swap;
pub mod symmetry_swap;
pub mod helium_treasury_management_redeem_v0;
pub mod goosefx_v2_swap;
pub mod perps_swap;
pub mod perps_add_liquidity;
pub mod perps_remove_liquidity;
pub mod meteora_dlmm_swap;
pub mod open_book_v2_swap;
pub mod clone_swap;
pub mod raydium_cp_swap;
pub mod one_intro_swap;
pub mod pumpdotfun_wrapped_buy;
pub mod pumpdotfun_wrapped_sell;
pub mod perps_v2_swap;
pub mod perps_v2_add_liquidity;
pub mod perps_v2_remove_liquidity;
pub mod moonshot_wrapped_buy;
pub mod moonshot_wrapped_sell;
pub mod stabble_stable_swap;
pub mod stabble_weighted_swap;
pub mod obric_swap;
pub mod swap_event;
pub mod fee_event;

#[derive(carbon_proc_macros::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum JupiterInstruction {
    Route(route::Route),
    RouteWithTokenLedger(route_with_token_ledger::RouteWithTokenLedger),
    ExactOutRoute(exact_out_route::ExactOutRoute),
    SharedAccountsRoute(shared_accounts_route::SharedAccountsRoute),
    SharedAccountsRouteWithTokenLedger(shared_accounts_route_with_token_ledger::SharedAccountsRouteWithTokenLedger),
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
    HeliumTreasuryManagementRedeemV0(helium_treasury_management_redeem_v0::HeliumTreasuryManagementRedeemV0),
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

impl InstructionDecoder for JupiterDecoder {
    type InstructionType = JupiterInstruction;

    fn decode_instruction(
        &self,
        instruction: solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if let Some(decoded_instruction) = route::Route::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::Route(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = route_with_token_ledger::RouteWithTokenLedger::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::RouteWithTokenLedger(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = exact_out_route::ExactOutRoute::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::ExactOutRoute(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = shared_accounts_route::SharedAccountsRoute::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::SharedAccountsRoute(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = shared_accounts_route_with_token_ledger::SharedAccountsRouteWithTokenLedger::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::SharedAccountsRouteWithTokenLedger(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = shared_accounts_exact_out_route::SharedAccountsExactOutRoute::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::SharedAccountsExactOutRoute(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = set_token_ledger::SetTokenLedger::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::SetTokenLedger(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = create_open_orders::CreateOpenOrders::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::CreateOpenOrders(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = create_token_account::CreateTokenAccount::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::CreateTokenAccount(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = create_program_open_orders::CreateProgramOpenOrders::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::CreateProgramOpenOrders(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = claim::Claim::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::Claim(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = claim_token::ClaimToken::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::ClaimToken(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = create_token_ledger::CreateTokenLedger::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::CreateTokenLedger(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = mercurial_swap::MercurialSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::MercurialSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = cykura_swap::CykuraSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::CykuraSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = serum_swap::SerumSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::SerumSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = saber_swap::SaberSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::SaberSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = saber_add_decimals::SaberAddDecimals::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::SaberAddDecimals(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = token_swap::TokenSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::TokenSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = token_swap_v2::TokenSwapV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::TokenSwapV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = sencha_swap::SenchaSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::SenchaSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = step_swap::StepSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::StepSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = cropper_swap::CropperSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::CropperSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = raydium_swap::RaydiumSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::RaydiumSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = crema_swap::CremaSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::CremaSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = lifinity_swap::LifinitySwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::LifinitySwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = marinade_deposit::MarinadeDeposit::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::MarinadeDeposit(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = marinade_unstake::MarinadeUnstake::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::MarinadeUnstake(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = aldrin_swap::AldrinSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::AldrinSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = aldrin_v2_swap::AldrinV2Swap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::AldrinV2Swap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = whirlpool_swap::WhirlpoolSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::WhirlpoolSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = whirlpool_swap_v2::WhirlpoolSwapV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::WhirlpoolSwapV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = invariant_swap::InvariantSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::InvariantSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = meteora_swap::MeteoraSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::MeteoraSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = goosefx_swap::GoosefxSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::GoosefxSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = deltafi_swap::DeltafiSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::DeltafiSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = balansol_swap::BalansolSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::BalansolSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = marco_polo_swap::MarcoPoloSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::MarcoPoloSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = dradex_swap::DradexSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::DradexSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = lifinity_v2_swap::LifinityV2Swap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::LifinityV2Swap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = raydium_clmm_swap::RaydiumClmmSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::RaydiumClmmSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = raydium_clmm_swap_v2::RaydiumClmmSwapV2::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::RaydiumClmmSwapV2(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = phoenix_swap::PhoenixSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::PhoenixSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = symmetry_swap::SymmetrySwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::SymmetrySwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = helium_treasury_management_redeem_v0::HeliumTreasuryManagementRedeemV0::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::HeliumTreasuryManagementRedeemV0(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = goosefx_v2_swap::GoosefxV2Swap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::GoosefxV2Swap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = perps_swap::PerpsSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::PerpsSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = perps_add_liquidity::PerpsAddLiquidity::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::PerpsAddLiquidity(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = perps_remove_liquidity::PerpsRemoveLiquidity::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::PerpsRemoveLiquidity(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = meteora_dlmm_swap::MeteoraDlmmSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::MeteoraDlmmSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = open_book_v2_swap::OpenBookV2Swap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::OpenBookV2Swap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = clone_swap::CloneSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::CloneSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = raydium_cp_swap::RaydiumCpSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::RaydiumCpSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = one_intro_swap::OneIntroSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::OneIntroSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = pumpdotfun_wrapped_buy::PumpdotfunWrappedBuy::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::PumpdotfunWrappedBuy(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = pumpdotfun_wrapped_sell::PumpdotfunWrappedSell::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::PumpdotfunWrappedSell(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = perps_v2_swap::PerpsV2Swap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::PerpsV2Swap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = perps_v2_add_liquidity::PerpsV2AddLiquidity::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::PerpsV2AddLiquidity(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = perps_v2_remove_liquidity::PerpsV2RemoveLiquidity::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::PerpsV2RemoveLiquidity(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = moonshot_wrapped_buy::MoonshotWrappedBuy::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::MoonshotWrappedBuy(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = moonshot_wrapped_sell::MoonshotWrappedSell::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::MoonshotWrappedSell(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = stabble_stable_swap::StabbleStableSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::StabbleStableSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = stabble_weighted_swap::StabbleWeightedSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::StabbleWeightedSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = obric_swap::ObricSwap::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::ObricSwap(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = swap_event::SwapEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::SwapEvent(decoded_instruction),
            });
        }
        if let Some(decoded_instruction) = fee_event::FeeEvent::deserialize(instruction.data.as_slice()) {
            return Some(carbon_core::instruction::DecodedInstruction {
                program_id: instruction.program_id,
                data: JupiterInstruction::FeeEvent(decoded_instruction),
            });
        }

        None
    }
}