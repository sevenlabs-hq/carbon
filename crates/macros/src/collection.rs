use carbon_core::{collection::InstructionDecoderCollection, instruction::DecodedInstruction};
use paste::paste;
use solana_sdk::pubkey::Pubkey;

#[macro_export]
macro_rules! instruction_decoder_collection {
    ($name:ident, $name_type:ident, $($variant:ident => $parser:expr => $origin:ty),+ $(,)?) => {
        #[derive(Debug, Clone)]
        pub enum $name {
            $($variant($origin)),+
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub enum $name_type {
            $($variant),+
        }

        impl InstructionDecoderCollection for $name {
            type InstructionType = $name_type;

            fn parse_instruction<'a>(instruction: &'a solana_sdk::instruction::Instruction) -> Option<DecodedInstruction<Self>> {
                $(
                    if let Some(decoded_instruction) = $parser.decode(instruction.clone()) {
                        return Some(DecodedInstruction {
                            program_id: instruction.program_id,
                            data: $name::$variant(decoded_instruction.data),
                        });
                    }
                )+
                None
            }

            fn get_type(&self) -> Self::InstructionType {
                match self {
                    $(Self::$variant(_) => $name_type::$variant),+
                }
            }
        }
    };
}
