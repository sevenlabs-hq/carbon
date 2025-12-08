use solana_pubkey::Pubkey;

pub struct PumpSwapDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    Pubkey::from_str_const("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA");

#[cfg(test)]
mod tests {

    use crate::instructions::PumpSwapInstruction;

    use super::PumpSwapDecoder;
    use carbon_core::instruction::InstructionDecoder;

    #[test]
    fn test_buy_deserialization_without_track_volume() {
        let decoder = PumpSwapDecoder;
        let ix =
            carbon_test_utils::read_instruction("tests/fixtures/buy_with_track_volume_false.json")
                .expect("read fixture");

        let maybe_decoded = decoder.decode_instruction(&ix);
        let decoded = maybe_decoded.expect("Invalid instruction");
        match decoded.data {
            PumpSwapInstruction::Buy(buy) => {
                assert!(!buy.track_volume.0);
            }
            other => {
                panic!("Expected Buy, got {other:?}");
            }
        }
    }

    #[test]
    fn test_buy_deserialization_with_track_volume() {
        let decoder = PumpSwapDecoder;
        let ix =
            carbon_test_utils::read_instruction("tests/fixtures/buy_with_track_volume_true.json")
                .expect("read fixture");

        let decoded = decoder
            .decode_instruction(&ix)
            .expect("Invalid instruction");
        match decoded.data {
            PumpSwapInstruction::Buy(buy) => {
                assert!(buy.track_volume.0);
            }
            other => {
                panic!("Expected Buy, got {other:?}");
            }
        }
    }
}
