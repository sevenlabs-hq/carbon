//! EOF-tolerant boolean for Solana program instructions.
//!
//! Many Solana programs (PumpSwap, Pump) define an `OptionBool` IDL type as a
//! trailing instruction argument that callers may omit.  The on-chain program
//! defaults to `false` when the byte is absent.
//!
//! Codama mis-maps this as `type OptionBool = bool;`, which requires the byte
//! to be present.  This newtype preserves the optional semantics: borsh
//! deserialization treats EOF as `false` instead of returning an error.
//!
//! Reference: official PumpSwap IDL at
//! <https://github.com/pump-fun/pump-public-docs/blob/main/idl/pump_amm.json>
//! defines `OptionBool` as `{ "kind": "struct", "fields": ["bool"] }`.

/// A boolean that defaults to `false` when the byte stream ends early.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, borsh::BorshSerialize)]
pub struct OptionBool(pub bool);

impl borsh::BorshDeserialize for OptionBool {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut buf = [0u8; 1];
        match reader.read_exact(&mut buf) {
            Ok(()) => Ok(OptionBool(buf[0] != 0)),
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(OptionBool(false)),
            Err(e) => Err(e),
        }
    }
}

impl From<OptionBool> for bool {
    fn from(v: OptionBool) -> bool {
        v.0
    }
}

impl From<bool> for OptionBool {
    fn from(v: bool) -> Self {
        OptionBool(v)
    }
}
