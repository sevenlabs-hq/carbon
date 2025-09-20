// Local wrapper for legacy Solana Pubkey types used across the workspace.
// This file provides a thin, well-typed bridge between the "modern"
// solana_program::pubkey::Pubkey and the legacy crate that some external
// dependencies expect (declared in Cargo.toml as `solana-pubkey-legacy`).

use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// Wrapper around the legacy pubkey type (crate dependency alias
/// `solana-pubkey-legacy` -> crate name `solana_pubkey_legacy`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Pubkey(pub legacy_solana_pubkey::Pubkey);

pub fn to_modern(pk: &legacy_solana_pubkey::Pubkey) -> solana_program::pubkey::Pubkey {
    solana_program::pubkey::Pubkey::new_from_array(pk.to_bytes())
}

impl Pubkey {
    /// Access the inner legacy pubkey.
    pub fn inner(&self) -> &legacy_solana_pubkey::Pubkey {
        &self.0
    }
}

impl From<solana_program::pubkey::Pubkey> for Pubkey {
    /// Convert a `solana_program::pubkey::Pubkey` into the legacy wrapper by
    /// transferring the raw bytes. This conversion is infallible.
    fn from(pk: solana_program::pubkey::Pubkey) -> Self {
        // Use the canonical bytes representation and construct the legacy
        // Pubkey from that array. Both sides expose byte-array based APIs
        // so this is safe and stable across versions.
        let bytes = pk.to_bytes();
        let legacy = legacy_solana_pubkey::Pubkey::new_from_array(bytes);
        Self(legacy)
    }
}

impl From<spl_token::solana_program::pubkey::Pubkey> for Pubkey {
    /// Convert a `spl_token::solana_program::pubkey::Pubkey` into the legacy wrapper by
    /// transferring the raw bytes. This conversion is infallible.
    fn from(pk: spl_token::solana_program::pubkey::Pubkey) -> Self {
        // Use the canonical bytes representation and construct the legacy
        // Pubkey from that array. Both sides expose byte-array based APIs
        // so this is safe and stable across versions.
        let bytes = pk.to_bytes();
        let legacy = legacy_solana_pubkey::Pubkey::new_from_array(bytes);
        Self(legacy)
    }
}

impl From<Pubkey> for solana_program::pubkey::Pubkey {
    /// Convert back to the `solana_program` Pubkey using raw bytes.
    fn from(lp: Pubkey) -> Self {
        solana_program::pubkey::Pubkey::new_from_array(lp.0.to_bytes())
    }
}

impl AsRef<legacy_solana_pubkey::Pubkey> for Pubkey {
    fn as_ref(&self) -> &legacy_solana_pubkey::Pubkey {
        &self.0
    }
}

impl Deref for Pubkey {
    type Target = legacy_solana_pubkey::Pubkey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
