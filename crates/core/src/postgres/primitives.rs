use bigdecimal::BigDecimal;
use borsh::BorshDeserialize;
use num_traits::cast::{FromPrimitive, ToPrimitive};
use sqlx::postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef};
use sqlx::types::Decimal;
use sqlx::{Decode, Encode, Postgres, Type};
use std::str::FromStr;
use std::{convert::TryFrom, ops::Deref};

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
