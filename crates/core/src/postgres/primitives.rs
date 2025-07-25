#![cfg(feature = "postgres")]

use borsh::BorshDeserialize;
use num_traits::cast::{FromPrimitive, ToPrimitive};
use sqlx::postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef};
use sqlx::types::Decimal;
use sqlx::{Decode, Encode, Postgres, Type};
use std::{convert::TryFrom, ops::Deref};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
        Ok(Self(solana_pubkey::Pubkey::try_from_slice(&bytes)?))
    }
}

impl Type<Postgres> for Pubkey {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("BYTEA")
    }
}

macro_rules! unsigned_small {
    ($name:ident, $src:ty, $inner:ty) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug, sqlx::Encode, sqlx::Decode)]
        pub struct $name(pub $inner);

        /* constructors & conversions */
        impl From<$src> for $name {
            fn from(v: $src) -> Self {
                Self(v as $inner)
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
    };
}

unsigned_small!(U8, u8, i16);
unsigned_small!(U16, u16, i32);
unsigned_small!(U32, u32, i64);

macro_rules! big_unsigned {
    ($name:ident, $src:ty, $prec:literal) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
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
        impl<'q> Encode<'q, Postgres> for $name {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync + 'static>>
            {
                let bd = Decimal::from_u128(self.0 as u128)
                    .ok_or(crate::error::Error::Custom("out of range".to_string()))?;
                bd.encode_by_ref(buf)
            }
        }
        impl<'r> Decode<'r, Postgres> for $name {
            fn decode(
                value: sqlx::postgres::PgValueRef<'r>,
            ) -> Result<Self, sqlx::error::BoxDynError> {
                let bd = Decimal::decode(value)?;
                match bd.to_u128() {
                    Some(v) => Ok(Self(v as $src)),
                    None => Err(Box::new(crate::error::Error::Custom(
                        "out of range".to_string(),
                    ))),
                }
            }
        }
    };
}

big_unsigned!(U64, u64, 20);
big_unsigned!(U128, u128, 39);

#[derive(Clone, Debug, PartialEq, Eq)]
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
impl<'q> Encode<'q, Postgres> for I128 {
    fn encode_by_ref(
        &self,
        buf: &mut sqlx::postgres::PgArgumentBuffer,
    ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + Send + Sync + 'static>> {
        let bd = Decimal::from_i128(self.0)
            .ok_or(crate::error::Error::Custom("out of range".to_string()))?;
        bd.encode_by_ref(buf)
    }
}
impl<'r> Decode<'r, Postgres> for I128 {
    fn decode(value: sqlx::postgres::PgValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let bd = Decimal::decode(value)?;
        match bd.to_i128() {
            Some(v) => Ok(Self(v)),
            None => Err(Box::new(crate::error::Error::Custom(
                "out of range".to_string(),
            ))),
        }
    }
}
