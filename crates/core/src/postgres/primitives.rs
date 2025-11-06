//! Provides PostgreSQL-specific primitive types and their SQLx implementations
//! for the `carbon-core` framework.
//!
//! This module defines custom primitive types that are designed to work seamlessly
//! with PostgreSQL databases through the `sqlx` crate. These primitives handle
//! Solana-specific data types like public keys and various numeric types that
//! may exceed the standard PostgreSQL integer ranges.
//!
//! # Overview
//!
//! The primitives module includes:
//! - **Pubkey**: A wrapper around Solana's `Pubkey` type with PostgreSQL
//!   encoding/decoding support using `BYTEA` storage.
//! - **Unsigned Integer Types**: `U8`, `U16`, and `U32` wrappers that map
//!   unsigned integers to PostgreSQL's signed integer types with range validation.
//! - **Large Integer Types**: `U64`, `U128`, and `I128` types that use
//!   PostgreSQL's `NUMERIC` type to handle values beyond standard integer ranges.
//!
//! # Key Components
//!
//! ## Pubkey Type
//!
//! The `Pubkey` type wraps Solana's public key with PostgreSQL compatibility:
//! - Stores as `BYTEA` in PostgreSQL
//! - Implements `Deref` for transparent access to the underlying `Pubkey`
//! - Provides `From` conversions for easy type conversion
//!
//! ## Unsigned Integer Types
//!
//! Small unsigned integers (`U8`, `U16`, `U32`) are mapped to PostgreSQL's
//! signed integer types with range validation:
//! - `U8` → `SMALLINT` (i16)
//! - `U16` → `INTEGER` (i32)
//! - `U32` → `BIGINT` (i64)
//!
//! ## Large Integer Types
//!
//! Large integers use PostgreSQL's `NUMERIC` type to handle values beyond
//! standard integer ranges:
//! - `U64` → `NUMERIC(20)`
//! - `U128` → `NUMERIC(39)`
//! - `I128` → `NUMERIC`
//!
//! # Usage Examples
//!
//! ## Using Pubkey in SQLx queries
//!
//! ```ignore
//! use carbon_core::postgres::primitives::Pubkey;
//! use sqlx::PgPool;
//!
//! async fn get_account(pool: &PgPool, pubkey: Pubkey) -> Result<Option<Account>, sqlx::Error> {
//!     sqlx::query_as!(
//!         Account,
//!         "SELECT * FROM accounts WHERE pubkey = $1",
//!         pubkey
//!     )
//!     .fetch_optional(pool)
//!     .await
//! }
//! ```
//!
//! ## Using unsigned integers
//!
//! ```ignore
//! use carbon_core::postgres::primitives::{U8, U16, U32};
//!
//! // Automatic conversion from standard types
//! let slot: U64 = 12345u64.into();
//! let index: U32 = 5u32.into();
//!
//! // Range validation on conversion back
//! match u64::try_from(slot) {
//!     Ok(value) => println!("Slot: {}", value),
//!     Err(_) => println!("Value out of range"),
//! }
//! ```
//!
//! ## Using large integers
//!
//! ```ignore
//! use carbon_core::postgres::primitives::{U64, U128, I128};
//!
//! // Large values that exceed standard integer ranges
//! let lamports: U64 = 1_000_000_000_000u64.into();
//! let amount: U128 = 1_000_000_000_000_000_000u128.into();
//! let balance: I128 = (-1_000_000_000_000i128).into();
//! ```
//!
//! # Notes
//!
//! - All types implement `Deref` for transparent access to their underlying values
//! - Range validation is performed when converting from PostgreSQL types back to
//!   standard Rust types
//! - The `NUMERIC` types support arbitrary precision but have performance overhead
//!   compared to native integer types
//! - Array types are supported for all primitives (e.g., `_BYTEA` for `Pubkey`)
//! - Error handling uses the crate's `CarbonResult` type for consistency

use bigdecimal::BigDecimal;
use borsh::BorshDeserialize;
use num_traits::cast::{FromPrimitive, ToPrimitive};
use sqlx::postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef};
use sqlx::types::Decimal;
use sqlx::{Decode, Encode, Postgres, Type};
use std::str::FromStr;
use std::{convert::TryFrom, ops::Deref};

/// A PostgreSQL-compatible wrapper around Solana's `Pubkey` type.
///
/// This type provides seamless integration between Solana public keys and
/// PostgreSQL databases. It stores public keys as `BYTEA` in PostgreSQL,
/// allowing for efficient storage and retrieval while maintaining type safety.
///
/// # Features
///
/// - **PostgreSQL Integration**: Implements `sqlx` traits for direct use in
///   database queries
/// - **Transparent Access**: Implements `Deref` for direct access to the
///   underlying `Pubkey`
/// - **Type Conversion**: Provides `From` implementations for easy conversion
///   from standard `Pubkey` types
/// - **Array Support**: Supports PostgreSQL array types (`_BYTEA`)
///
/// # Example
///
/// ```ignore
/// use carbon_core::postgres::primitives::Pubkey;
/// use solana_pubkey::Pubkey as SolanaPubkey;
///
/// // Create from a Solana pubkey
/// let solana_pubkey = SolanaPubkey::new_unique();
/// let pg_pubkey: Pubkey = solana_pubkey.into();
///
/// // Use directly in SQLx queries
/// sqlx::query!("SELECT * FROM accounts WHERE pubkey = $1", pg_pubkey)
///     .fetch_optional(pool)
///     .await?;
/// ```
#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Pubkey(pub solana_pubkey::Pubkey);

impl Deref for Pubkey {
    type Target = solana_pubkey::Pubkey;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<solana_pubkey::Pubkey> for Pubkey {
    fn from(pubkey: solana_pubkey::Pubkey) -> Self {
        Self(pubkey)
    }
}

// Ergonomic conversions from raw bytes (DEFAULT on invalid length/format) for debugging paths
impl From<Vec<u8>> for Pubkey {
    fn from(bytes: Vec<u8>) -> Self {
        Self(solana_pubkey::Pubkey::try_from_slice(&bytes).unwrap_or_default())
    }
}
impl From<&[u8]> for Pubkey {
    fn from(bytes: &[u8]) -> Self {
        Self(solana_pubkey::Pubkey::try_from_slice(bytes).unwrap_or_default())
    }
}

impl Encode<'_, Postgres> for Pubkey {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync + 'static>> {
        self.0.to_bytes().encode_by_ref(buf)
    }
}

impl Decode<'_, Postgres> for Pubkey {
    fn decode(
        value: PgValueRef<'_>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let bytes = value.as_bytes()?;
        Ok(Self(solana_pubkey::Pubkey::try_from_slice(bytes)?))
    }
}

impl Type<Postgres> for Pubkey {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("BYTEA")
    }
}

impl PgHasArrayType for Pubkey {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_BYTEA")
    }
}

/// Macro for creating PostgreSQL-compatible unsigned integer types.
///
/// This macro generates a wrapper type that maps unsigned integers to
/// PostgreSQL's signed integer types with range validation. The generated
/// type provides seamless conversion between Rust unsigned types and
/// PostgreSQL signed types while ensuring data integrity.
///
/// # Parameters
///
/// - `$name`: The name of the generated type
/// - `$src`: The source unsigned integer type (e.g., `u8`, `u16`, `u32`)
/// - `$inner`: The PostgreSQL integer type (e.g., `i16`, `i32`, `i64`)
/// - `$type`: The PostgreSQL type name (e.g., `"SMALLINT"`, `"INTEGER"`)
/// - `$array_type`: The PostgreSQL array type name (e.g., `"_SMALLINT"`)
///
/// # Generated Features
///
/// - **Type Safety**: Range validation when converting back to source types
/// - **PostgreSQL Integration**: Full `sqlx` trait implementations
/// - **Transparent Access**: `Deref` implementation for direct value access
/// - **Conversion Support**: `From` and `TryFrom` implementations
/// - **Array Support**: PostgreSQL array type support
///
/// # Example
///
/// ```ignore
/// // Generated type: U8(u8) -> SMALLINT(i16)
/// let value: U8 = 255u8.into();
///
/// // Range validation on conversion back
/// match u8::try_from(value) {
///     Ok(v) => println!("Value: {}", v),
///     Err(_) => println!("Value out of range"),
/// }
/// ```
macro_rules! unsigned_small {
    ($name:ident, $src:ty, $inner:ty, $type:literal, $array_type:literal) => {
        #[derive(
            Clone,
            Copy,
            PartialEq,
            Eq,
            Debug,
            sqlx::Encode,
            sqlx::Decode,
            serde::Serialize,
            serde::Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(pub $inner);

        /* constructors & conversions */
        impl From<$src> for $name {
            fn from(v: $src) -> Self {
                Self(v as $inner)
            }
        }
        impl From<$inner> for $name {
            fn from(v: $inner) -> Self {
                Self(v)
            }
        }
        impl From<&$inner> for $name {
            fn from(v: &$inner) -> Self {
                Self(*v)
            }
        }
        impl TryFrom<$name> for $src {
            type Error = crate::error::Error;
            fn try_from(v: $name) -> Result<Self, Self::Error> {
                <$src>::try_from(v.0)
                    .map_err(|_| crate::error::Error::Custom("out of range".to_string()))
            }
        }

        impl Deref for $name {
            type Target = $inner;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl Type<Postgres> for $name {
            fn type_info() -> PgTypeInfo {
                PgTypeInfo::with_name($type)
            }
        }
        impl PgHasArrayType for $name {
            fn array_type_info() -> PgTypeInfo {
                PgTypeInfo::with_name($array_type)
            }
        }
    };
}

unsigned_small!(U8, u8, i16, "INT2", "_INT2");
unsigned_small!(U16, u16, i32, "INT4", "_INT4");
unsigned_small!(U32, u32, i64, "INT8", "_INT8");

/// Macro for creating PostgreSQL-compatible large unsigned integer types.
///
/// This macro generates a wrapper type that uses PostgreSQL's `NUMERIC` type
/// to handle unsigned integers that exceed the range of standard PostgreSQL
/// integer types. The generated type provides arbitrary precision support
/// for large numeric values.
///
/// # Parameters
///
/// - `$name`: The name of the generated type
/// - `$src`: The source unsigned integer type (e.g., `u64`, `u128`)
/// - `$prec`: The precision specification for the NUMERIC type
///
/// # Generated Features
///
/// - **Arbitrary Precision**: Uses PostgreSQL `NUMERIC` type for large values
/// - **Range Safety**: Handles values beyond standard integer ranges
/// - **PostgreSQL Integration**: Full `sqlx` trait implementations
/// - **Transparent Access**: `Deref` implementation for direct value access
/// - **Conversion Support**: `From` implementations for easy type conversion
/// - **Array Support**: PostgreSQL array type support (`_NUMERIC`)
///
/// # Performance Considerations
///
/// - `NUMERIC` types have higher performance overhead compared to native
///   integer types
/// - Use for values that exceed standard integer ranges only
/// - Consider using smaller types when possible for better performance
///
/// # Example
///
/// ```ignore
/// // Generated type: U64(u64) -> NUMERIC(20)
/// let lamports: U64 = 1_000_000_000_000u64.into();
/// let amount: U128 = 1_000_000_000_000_000_000u128.into();
///
/// // Direct access to underlying value
/// println!("Lamports: {}", *lamports);
/// ```
macro_rules! big_unsigned {
    ($name:ident, $src:ty, $prec:literal) => {
        #[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub $src);

        impl From<$src> for $name {
            fn from(v: $src) -> Self {
                Self(v)
            }
        }
        impl Deref for $name {
            type Target = $src;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl TryFrom<$name> for $src {
            type Error = crate::error::Error;
            fn try_from(v: $name) -> Result<Self, Self::Error> {
                Ok(v.0)
            }
        }

        /* NUMERIC <=-> BigDecimal bridge */
        impl Type<Postgres> for $name {
            fn type_info() -> PgTypeInfo {
                PgTypeInfo::with_name("NUMERIC")
            }
        }
        impl PgHasArrayType for $name {
            fn array_type_info() -> PgTypeInfo {
                PgTypeInfo::with_name("_NUMERIC")
            }
        }
        impl Encode<'_, Postgres> for $name {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync + 'static>>
            {
                // Encode via BigDecimal -> PgNumeric (true NUMERIC binary)
                let s = self.0.to_string();
                let bd = BigDecimal::from_str(&s).map_err(|e| {
                    Box::new(crate::error::Error::Custom(e.to_string()))
                        as Box<dyn std::error::Error + Send + Sync + 'static>
                })?;
                <BigDecimal as Encode<'_, Postgres>>::encode_by_ref(&bd, buf)
            }
        }
        impl<'r> Decode<'r, Postgres> for $name {
            fn decode(
                value: sqlx::postgres::PgValueRef<'r>,
            ) -> Result<Self, sqlx::error::BoxDynError> {
                // Decode NUMERIC via BigDecimal, expect integer (dscale == 0)
                let bd: BigDecimal = <BigDecimal as Decode<'r, Postgres>>::decode(value)?;
                let s = bd.to_string();
                if s.contains('.') || s.contains('-') {
                    return Err(Box::new(crate::error::Error::Custom(
                        "invalid sign/scale for unsigned integer".to_string(),
                    )));
                }
                match s.parse::<u128>() {
                    Ok(v) if v <= (<$src>::MAX as u128) => Ok(Self(v as $src)),
                    Ok(_) => Err(Box::new(crate::error::Error::Custom(
                        "value exceeds maximum for target type".to_string(),
                    ))),
                    Err(_) => Err(Box::new(crate::error::Error::Custom(
                        "invalid numeric string".to_string(),
                    ))),
                }
            }
        }

        impl From<$name> for Decimal {
            fn from(v: $name) -> Self {
                // safe: $src is an unsigned integer; representable as u128
                Decimal::from_u128(v.0 as u128).unwrap_or_default()
            }
        }

        /* Total conversions (panic on out-of-range) for ergonomic `.into()` */
        impl From<Decimal> for $name {
            fn from(d: Decimal) -> Self {
                match d.to_u128() {
                    Some(v) if v <= (<$src>::MAX as u128) => Self(v as $src),
                    _ => Self(0),
                }
            }
        }
        impl From<&Decimal> for $name {
            fn from(d: &Decimal) -> Self {
                match d.to_u128() {
                    Some(v) if v <= (<$src>::MAX as u128) => Self(v as $src),
                    _ => Self(0),
                }
            }
        }
    };
}

big_unsigned!(U64, u64, 20);
big_unsigned!(U128, u128, 39);

/// A PostgreSQL-compatible wrapper for 128-bit signed integers.
///
/// This type uses PostgreSQL's `NUMERIC` type to handle 128-bit signed integers
/// that exceed the range of standard PostgreSQL integer types. It provides
/// arbitrary precision support for large signed numeric values.
///
/// # Features
///
/// - **Arbitrary Precision**: Uses PostgreSQL `NUMERIC` type for large values
/// - **Range Safety**: Handles values beyond standard integer ranges
/// - **PostgreSQL Integration**: Full `sqlx` trait implementations
/// - **Transparent Access**: `Deref` implementation for direct value access
/// - **Conversion Support**: `From` implementations for easy type conversion
/// - **Array Support**: PostgreSQL array type support (`_NUMERIC`)
///
/// # Performance Considerations
///
/// - `NUMERIC` types have higher performance overhead compared to native
///   integer types
/// - Use for values that exceed standard integer ranges only
/// - Consider using smaller types when possible for better performance
///
/// # Example
///
/// ```ignore
/// use carbon_core::postgres::primitives::I128;
///
/// // Large signed values
/// let balance: I128 = (-1_000_000_000_000i128).into();
/// let amount: I128 = 1_000_000_000_000_000_000i128.into();
///
/// // Direct access to underlying value
/// println!("Balance: {}", *balance);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct I128(pub i128);

impl From<i128> for I128 {
    fn from(v: i128) -> Self {
        Self(v)
    }
}
impl Deref for I128 {
    type Target = i128;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Type<Postgres> for I128 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("NUMERIC")
    }
}
impl PgHasArrayType for I128 {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_NUMERIC")
    }
}
impl Encode<'_, Postgres> for I128 {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync + 'static>> {
        // Encode via BigDecimal -> PgNumeric to support full i128 range
        let s = self.0.to_string();
        let bd = BigDecimal::from_str(&s).map_err(|e| {
            Box::new(crate::error::Error::Custom(e.to_string()))
                as Box<dyn std::error::Error + Send + Sync + 'static>
        })?;
        <BigDecimal as Encode<'_, Postgres>>::encode_by_ref(&bd, buf)
    }
}
impl Decode<'_, Postgres> for I128 {
    fn decode(value: sqlx::postgres::PgValueRef<'_>) -> Result<Self, sqlx::error::BoxDynError> {
        // Decode via BigDecimal and parse as i128 (require integer scale)
        let bd: BigDecimal = <BigDecimal as Decode<'_, Postgres>>::decode(value)?;
        let s = bd.to_string();
        if s.contains('.') {
            return Err(Box::new(crate::error::Error::Custom(
                "invalid scale for i128".to_string(),
            )));
        }
        match s.parse::<i128>() {
            Ok(v) => Ok(Self(v)),
            Err(_) => Err(Box::new(crate::error::Error::Custom(
                "invalid numeric string".to_string(),
            ))),
        }
    }
}
