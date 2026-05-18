//! Borsh helpers shared by Codama-generated decoder crates.
//!
//! # Components
//!
//! - [`CarbonDeserialize`] — discriminator-prefixed borsh deserialization
//!   contract used by every generated instruction/account decoder.
//! - [`extract_discriminator`] — splits raw bytes into `(discriminator,
//!   payload)`.
//! - [`ArrangeAccounts`] — turns the positional `&[AccountMeta]` of an
//!   instruction into a typed accounts struct.
//! - [`PrefixString`] / [`U64PrefixString`] — newtypes for `String`s serialized
//!   with a length prefix wider than borsh's default.

use std::{
    io::{Error, ErrorKind, Read, Result},
    ops::Deref,
};
/// Discriminator-prefixed borsh deserialization contract.
///
/// Implementors define a static `DISCRIMINATOR` byte slice (typically
/// 8 bytes for Anchor-style programs) and `deserialize` is expected to
/// peel that prefix off before delegating to `BorshDeserialize`.
pub trait CarbonDeserialize
where
    Self: Sized + crate::borsh::BorshDeserialize,
{
    const DISCRIMINATOR: &'static [u8];

    fn deserialize(data: &[u8]) -> Option<Self>;
}

pub fn extract_discriminator(length: usize, data: &[u8]) -> Option<(&[u8], &[u8])> {
    if data.len() < length {
        return None;
    }

    Some((&data[..length], &data[length..]))
}

/// Turns the positional `&[AccountMeta]` of an instruction into a typed
/// accounts struct. Generated alongside instruction decoders; rarely
/// implemented by hand.
pub trait ArrangeAccounts {
    type ArrangedAccounts: Clone + Send + Sync + std::fmt::Debug;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts>;
}

/// `String` newtype with a 32-bit borsh length prefix (matches the
/// default `String` layout but exposed as a distinct type so generated
/// decoders can opt in explicitly).
#[derive(serde::Serialize, serde::Deserialize, Default, PartialEq, Eq, Clone)]
pub struct PrefixString(pub String);

impl Deref for PrefixString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PrefixString> for String {
    fn from(val: PrefixString) -> Self {
        val.0
    }
}

impl std::fmt::Debug for PrefixString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

impl crate::borsh::BorshDeserialize for PrefixString {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        // read the length of the String
        let mut buffer = vec![0u8; 4];
        reader.read_exact(&mut buffer)?;
        let length = u32::deserialize(&mut buffer.as_slice())?;
        let mut buffer = vec![0u8; length as usize];
        reader.read_exact(&mut buffer)?;

        Ok(Self(String::from_utf8(buffer).map_err(|_| {
            Error::new(ErrorKind::InvalidData, "invalid utf8")
        })?))
    }
}

/// `String` newtype with a 64-bit borsh length prefix. Used by programs
/// that serialise long strings outside `String`'s 4-byte default.
#[derive(serde::Serialize, Default, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct U64PrefixString(pub String);

impl Deref for U64PrefixString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<U64PrefixString> for String {
    fn from(val: U64PrefixString) -> Self {
        val.0
    }
}

impl std::fmt::Debug for U64PrefixString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

impl crate::borsh::BorshDeserialize for U64PrefixString {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        // read the length of the String
        let mut buffer = vec![0u8; 8];
        reader.read_exact(&mut buffer)?;
        let length = u64::deserialize(&mut buffer.as_slice())?;
        let mut buffer = vec![0u8; length as usize];
        reader.read_exact(&mut buffer)?;

        Ok(Self(String::from_utf8(buffer).map_err(|_| {
            Error::new(ErrorKind::InvalidData, "invalid utf8")
        })?))
    }
}
