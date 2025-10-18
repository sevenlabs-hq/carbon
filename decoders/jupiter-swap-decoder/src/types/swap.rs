use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum Swap {
    Saber,
    SaberAddDecimalsDeposit,
    SaberAddDecimalsWithdraw,
    TokenSwap,
    Sencha,
    Step,
    Cropper,
    Raydium,
    Crema {
        a_to_b: bool,
    },
    Lifinity,
    Mercurial,
    Cykura,
    Serum {
        side: Side,
    },
    MarinadeDeposit,
    MarinadeUnstake,
    Aldrin {
        side: Side,
    },
    AldrinV2 {
        side: Side,
    },
    Whirlpool {
        a_to_b: bool,
    },
    Invariant {
        x_to_y: bool,
    },
    Meteora,
    GooseFX,
    DeltaFi {
        stable: bool,
    },
    Balansol,
    MarcoPolo {
        x_to_y: bool,
    },
    Dradex {
        side: Side,
    },
    LifinityV2,
    RaydiumClmm,
    Openbook {
        side: Side,
    },
    Phoenix {
        side: Side,
    },
    Symmetry {
        from_token_id: u64,
        to_token_id: u64,
    },
    TokenSwapV2,
    HeliumTreasuryManagementRedeemV0,
    StakeDexStakeWrappedSol,
    StakeDexSwapViaStake {
        bridge_stake_seed: u32,
    },
    GooseFXV2,
    Perps,
    PerpsAddLiquidity,
    PerpsRemoveLiquidity,
    MeteoraDlmm,
    OpenBookV2 {
        side: Side,
    },
    RaydiumClmmV2,
    StakeDexPrefundWithdrawStakeAndDepositStake {
        bridge_stake_seed: u32,
    },
    Clone {
        pool_index: u8,
        quantity_is_input: bool,
        quantity_is_collateral: bool,
    },
    SanctumS {
        src_lst_value_calc_accs: u8,
        dst_lst_value_calc_accs: u8,
        src_lst_index: u32,
        dst_lst_index: u32,
    },
    SanctumSAddLiquidity {
        lst_value_calc_accs: u8,
        lst_index: u32,
    },
    SanctumSRemoveLiquidity {
        lst_value_calc_accs: u8,
        lst_index: u32,
    },
    RaydiumCP,
    WhirlpoolSwapV2 {
        a_to_b: bool,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    },
    OneIntro,
    PumpWrappedBuy,
    PumpWrappedSell,
    PerpsV2,
    PerpsV2AddLiquidity,
    PerpsV2RemoveLiquidity,
    MoonshotWrappedBuy,
    MoonshotWrappedSell,
    StabbleStableSwap,
    StabbleWeightedSwap,
    Obric {
        x_to_y: bool,
    },
    FoxBuyFromEstimatedCost,
    FoxClaimPartial {
        is_y: bool,
    },
    SolFi {
        is_quote_to_base: bool,
    },
    SolayerDelegateNoInit,
    SolayerUndelegateNoInit,
    TokenMill {
        side: Side,
    },
    DaosFunBuy,
    DaosFunSell,
    ZeroFi,
    StakeDexWithdrawWrappedSol,
    VirtualsBuy,
    VirtualsSell,
    Perena {
        in_index: u8,
        out_index: u8,
    },
    PumpSwapBuy,
    PumpSwapSell,
    Gamma,
    MeteoraDlmmSwapV2 {
        remaining_accounts_info: RemainingAccountsInfo,
    },
    Woofi,
    MeteoraDammV2,
    MeteoraDynamicBondingCurveSwap,
    StabbleStableSwapV2,
    StabbleWeightedSwapV2,
    RaydiumLaunchlabBuy {
        share_fee_rate: u64,
    },
    RaydiumLaunchlabSell {
        share_fee_rate: u64,
    },
    BoopdotfunWrappedBuy,
    BoopdotfunWrappedSell,
    Plasma {
        side: Side,
    },
    GoonFi {
        is_bid: bool,
        blacklist_bump: u8,
    },
    HumidiFi {
        swap_id: u64,
        is_base_to_quote: bool,
    },
    MeteoraDynamicBondingCurveSwapWithRemainingAccounts,
    TesseraV {
        side: Side,
    },
    PumpWrappedBuyV2,
    PumpWrappedSellV2,
    PumpSwapBuyV2,
    PumpSwapSellV2,
    Heaven {
        a_to_b: bool,
    },
    SolFiV2 {
        is_quote_to_base: bool,
    },
    Aquifer,
    PumpWrappedBuyV3,
    PumpWrappedSellV3,
    PumpSwapBuyV3,
    PumpSwapSellV3,
    JupiterLendDeposit,
    JupiterLendRedeem,
    DefiTuna {
        a_to_b: bool,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    },
    AlphaQ {
        a_to_b: bool,
    },
    RaydiumV2,
}
