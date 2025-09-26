pub use legacy_solana_account::Account;

/// Convert a legacy solana_account representation into the modern solana_account::Account
///
/// This helper centralizes the conversion logic so callers can decode a legacy
/// account (e.g. coming from Helius) and then convert it into the modern
/// `solana_account::Account` used throughout the workspace.
pub fn to_modern_account(legacy: Account) -> solana_account::Account {
    solana_account::Account {
        lamports: legacy.lamports,
        data: legacy.data,
        owner: super::pubkey::to_modern(&legacy.owner),
        executable: legacy.executable,
        rent_epoch: legacy.rent_epoch,
    }
}
