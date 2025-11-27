use carbon_core::{borsh, CarbonDeserialize};
use serde::ser::SerializeTuple;

#[derive(CarbonDeserialize, Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub enum DynamicTick {
    Uninitialized,
    Initialized(DynamicTickData),
}

impl serde::Serialize for DynamicTick {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            DynamicTick::Uninitialized => serializer.serialize_u8(0x00),
            DynamicTick::Initialized(data) => {
                let mut tuple = serializer.serialize_tuple(2)?;
                tuple.serialize_element(&0x01u8)?;
                tuple.serialize_element(data)?;
                tuple.end()
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for DynamicTick {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        struct StreamOutputFormatVisitor;

        impl<'de> serde::de::Visitor<'de> for StreamOutputFormatVisitor {
            type Value = DynamicTick;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "a tuple of size 2 consisting of a u8 discriminant and a value"
                )
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let discriminant: u8 = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::invalid_length(0, &self))?;
                match discriminant {
                    0x00 => Ok(DynamicTick::Uninitialized),
                    0x01 => {
                        let y = seq
                            .next_element()?
                            .ok_or_else(|| A::Error::invalid_length(1, &self))?;
                        Ok(DynamicTick::Initialized(y))
                    }
                    d => Err(A::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(d as u64),
                        &"0x00 or 0x01",
                    )),
                }
            }
        }

        deserializer.deserialize_tuple(2, StreamOutputFormatVisitor)
    }
}

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash, Copy,
)]
pub struct DynamicTickData {
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_a: u128,
    pub fee_growth_outside_b: u128,
    pub reward_growths_outside: [u128; 3],
}
