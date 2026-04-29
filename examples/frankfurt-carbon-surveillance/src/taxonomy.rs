//! Activity taxonomy for surveillance events.
//!
//! Each transaction touching a watched wallet maps to one or more activity
//! categories below. The per-tx coordinator (see `coordinator.rs`) collects
//! candidates from every per-program processor, then emits ONE event per
//! (sig, wallet, category) — never one per instruction. Tip-PDA noise,
//! Anchor event-log echoes, and inner-CPI splits all fall away because
//! lower-precedence candidates lose to higher-precedence ones on the same
//! tx.
//!
//! `as_event_type` is the value written to `surveillance_events.event_type`
//! (and the parity table). Existing frontend rendering expects `swap_buy`,
//! `swap_sell`, `transfer_in`, `transfer_out`; new categories below extend
//! the schema additively without DDL changes.
//!
//! Precedence (higher number wins when categories collide on the same
//! (sig, wallet, family) — see `coordinator::ActivityFamily`).

/// Several variants are taxonomy slots reserved for future per-program
/// processors (LimitOrder/Vault/Stake/Dca/Nft/etc.). They compile but are
/// constructed only by code we haven't shipped yet.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Activity {
    SwapBuy,
    SwapSell,
    MintCreate,
    LiquidityAdd,
    LiquidityRemove,
    LimitOrderCreate,
    LimitOrderFill,
    VaultDeposit,
    VaultWithdraw,
    Stake,
    Unstake,
    DcaCreate,
    DcaClose,
    NftMint,
    NftSale,
    TransferIn,
    TransferOut,
    /// Fallback when a registered decoder fired on an unrecognized variant.
    /// Carries the program label so the frontend can show "wallet X used
    /// program Y" rows for activity we haven't mapped yet.
    ProgramActivity,
}

impl Activity {
    pub fn as_event_type(self) -> &'static str {
        match self {
            Activity::SwapBuy => "swap_buy",
            Activity::SwapSell => "swap_sell",
            Activity::MintCreate => "mint_create",
            Activity::LiquidityAdd => "liquidity_add",
            Activity::LiquidityRemove => "liquidity_remove",
            Activity::LimitOrderCreate => "limit_order_create",
            Activity::LimitOrderFill => "limit_order_fill",
            Activity::VaultDeposit => "vault_deposit",
            Activity::VaultWithdraw => "vault_withdraw",
            Activity::Stake => "stake",
            Activity::Unstake => "unstake",
            Activity::DcaCreate => "dca_create",
            Activity::DcaClose => "dca_close",
            Activity::NftMint => "nft_mint",
            Activity::NftSale => "nft_sale",
            Activity::TransferIn => "transfer_in",
            Activity::TransferOut => "transfer_out",
            Activity::ProgramActivity => "program_activity",
        }
    }

    /// Higher = wins when two candidates land in the same (sig, wallet,
    /// family) bucket. Within a family, richer data beats coarser data.
    /// Across families (e.g. SwapBuy and MintCreate on a pump.fun
    /// create_and_buy), both can co-emit — see `family()`.
    pub fn precedence(self) -> u8 {
        match self {
            Activity::SwapBuy
            | Activity::SwapSell
            | Activity::MintCreate
            | Activity::LiquidityAdd
            | Activity::LiquidityRemove
            | Activity::LimitOrderCreate
            | Activity::LimitOrderFill
            | Activity::VaultDeposit
            | Activity::VaultWithdraw
            | Activity::Stake
            | Activity::Unstake
            | Activity::DcaCreate
            | Activity::DcaClose
            | Activity::NftMint
            | Activity::NftSale => 100,
            Activity::TransferIn | Activity::TransferOut => 50,
            Activity::ProgramActivity => 10,
        }
    }

    /// Activities in the same family compete (only highest-precedence wins
    /// per-tx-per-wallet); activities in different families can co-emit.
    /// Example: pump.fun `create_and_buy` triggers both `MintCreate` and
    /// `SwapBuy` — different families, both rows persist.
    pub fn family(self) -> ActivityFamily {
        match self {
            Activity::SwapBuy | Activity::SwapSell => ActivityFamily::Swap,
            Activity::MintCreate => ActivityFamily::Mint,
            Activity::LiquidityAdd | Activity::LiquidityRemove => ActivityFamily::Liquidity,
            Activity::LimitOrderCreate | Activity::LimitOrderFill => ActivityFamily::LimitOrder,
            Activity::VaultDeposit | Activity::VaultWithdraw => ActivityFamily::Vault,
            Activity::Stake | Activity::Unstake => ActivityFamily::Staking,
            Activity::DcaCreate | Activity::DcaClose => ActivityFamily::Dca,
            Activity::NftMint | Activity::NftSale => ActivityFamily::Nft,
            Activity::TransferIn | Activity::TransferOut => ActivityFamily::Transfer,
            Activity::ProgramActivity => ActivityFamily::Fallback,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ActivityFamily {
    Swap,
    Mint,
    Liquidity,
    LimitOrder,
    Vault,
    Staking,
    Dca,
    Nft,
    Transfer,
    Fallback,
}
