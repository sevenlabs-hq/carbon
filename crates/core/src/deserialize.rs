//! Borsh helpers shared by Codama-generated decoder crates.
//!
//! # Components
//!
//! - [`extract_discriminator`] — splits raw bytes into `(discriminator,
//!   payload)`.
//! - [`ArrangeAccounts`] — turns the positional `&[AccountMeta]` of an
//!   instruction into a typed accounts struct.
//! - Newtypes for layouts `#[derive(BorshDeserialize)]` reads at the wrong
//!   width, each consuming exactly the bytes its field occupies:
//!   [`PrefixString`], [`U64PrefixString`], [`U64PrefixBytes`],
//!   [`U8PrefixBytes`], [`RemainderVec`] and [`ZeroableOptionPubkey`].

use std::{
    io::{Error, ErrorKind, Read, Result, Write},
    ops::Deref,
};

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

/// Declares a newtype that borsh reads by hand, plus the conversions generated
/// decoders and their Postgres rows need in both directions.
macro_rules! borsh_newtype {
    ($(#[$docs:meta])* $name:ident($inner:ty)) => {
        $(#[$docs])*
        #[derive(serde::Serialize, serde::Deserialize, Default, PartialEq, Eq, Clone, Hash)]
        pub struct $name(pub $inner);

        impl Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$name> for $inner {
            fn from(val: $name) -> Self {
                val.0
            }
        }

        impl From<$inner> for $name {
            fn from(val: $inner) -> Self {
                Self(val)
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{:?}", self.0))
            }
        }
    };
}

/// Reads `$width` bytes of length, then that many bytes of payload.
macro_rules! prefix_impls {
    ($name:ident, $width:ty, $bytes:expr, $read:expr) => {
        impl crate::borsh::BorshDeserialize for $name {
            #[inline]
            fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
                let mut length = [0u8; $bytes];
                reader.read_exact(&mut length)?;
                let mut buffer = vec![0u8; <$width>::from_le_bytes(length) as usize];
                reader.read_exact(&mut buffer)?;

                $read(buffer).map(Self)
            }
        }

        impl crate::borsh::BorshSerialize for $name {
            #[inline]
            fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
                let length = <$width>::try_from(self.0.len())
                    .map_err(|_| Error::new(ErrorKind::InvalidData, "length exceeds prefix width"))?;
                writer.write_all(&length.to_le_bytes())?;
                writer.write_all(self.0.as_ref())
            }
        }
    };
}

fn utf8(buffer: Vec<u8>) -> Result<String> {
    String::from_utf8(buffer).map_err(|_| Error::new(ErrorKind::InvalidData, "invalid utf8"))
}

borsh_newtype! {
    /// `String` with a 32-bit length prefix. Matches borsh's own `String`
    /// layout, exposed as a distinct type so generated decoders can say so.
    PrefixString(String)
}
prefix_impls!(PrefixString, u32, 4, utf8);

borsh_newtype! {
    /// `String` with a 64-bit length prefix, as bincode writes it.
    U64PrefixString(String)
}
prefix_impls!(U64PrefixString, u64, 8, utf8);

borsh_newtype! {
    /// `Vec<u8>` with a 64-bit length prefix, as bincode writes it.
    U64PrefixBytes(Vec<u8>)
}
prefix_impls!(U64PrefixBytes, u64, 8, Ok);

borsh_newtype! {
    /// `Vec<u8>` with an 8-bit length prefix.
    U8PrefixBytes(Vec<u8>)
}
prefix_impls!(U8PrefixBytes, u8, 1, Ok);

borsh_newtype! {
    /// `Option<Pubkey>` held in full with no option tag, where all-zero bytes
    /// mean unset. Reads 32 bytes either way, so the fields after it stay put.
    ZeroableOptionPubkey(Option<solana_pubkey::Pubkey>)
}

impl crate::borsh::BorshDeserialize for ZeroableOptionPubkey {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0u8; 32];
        reader.read_exact(&mut buffer)?;

        Ok(Self((buffer != [0u8; 32]).then(|| solana_pubkey::Pubkey::new_from_array(buffer))))
    }
}

impl crate::borsh::BorshSerialize for ZeroableOptionPubkey {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self.0.unwrap_or_default().to_bytes())
    }
}

/// `Vec<T>` with no count: items are read until the data runs out, so this is
/// only valid as the last field.
#[derive(serde::Serialize, serde::Deserialize, Default, PartialEq, Eq, Clone, Hash)]
pub struct RemainderVec<T>(pub Vec<T>);

impl<T> Deref for RemainderVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<RemainderVec<T>> for Vec<T> {
    fn from(val: RemainderVec<T>) -> Self {
        val.0
    }
}

impl<T> From<Vec<T>> for RemainderVec<T> {
    fn from(val: Vec<T>) -> Self {
        Self(val)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for RemainderVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

impl<T: crate::borsh::BorshDeserialize> crate::borsh::BorshDeserialize for RemainderVec<T> {
    #[inline]
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut rest = Vec::new();
        reader.read_to_end(&mut rest)?;

        let mut remaining = rest.as_slice();
        let mut items = Vec::new();
        while !remaining.is_empty() {
            items.push(T::deserialize(&mut remaining)?);
        }

        Ok(Self(items))
    }
}

impl<T: crate::borsh::BorshSerialize> crate::borsh::BorshSerialize for RemainderVec<T> {
    #[inline]
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        for item in &self.0 {
            item.serialize(writer)?;
        }
        Ok(())
    }
}
