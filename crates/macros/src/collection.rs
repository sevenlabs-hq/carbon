// #[macro_export]
// macro_rules! instruction_decoder_collection {
//     (
//         $instructions:ident, $instruction_types:ident, $programs:ident,
//         $(
//             $program:ident => $decoder:expr => $instruction:ty => $instruction_type:ty
//         ),+ $(,)?
//     ) => {
//         #[derive(Debug, Clone, std::hash::Hash, serde::Serialize, PartialEq, Eq)]
//         pub enum $instructions {
//             $(
//                 $program($instruction),
//             )+
//         }

//         #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
//         pub enum $instruction_types {
//             $(
//                 $program($instruction_type),
//             )+
//         }

//         #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
//         pub enum $programs {
//             $(
//                 $program,
//             )+
//         }

//         impl InstructionDecoderCollection for $instructions {
//             type InstructionType = $instruction_types;

//             fn parse_instruction(
//                 instruction: solana_sdk::instruction::Instruction
//             ) -> Option<DecodedInstruction<Self>> {
//                 $(
//                     if let Some(decoded_instruction) = $decoder.decode(instruction.clone()) {
//                         return Some(DecodedInstruction {
//                             program_id: instruction.program_id,
//                             data: $instructions::$program(decoded_instruction.data),
//                         });
//                     }
//                 )+
//                 None
//             }

//             fn get_type(&self) -> Self::InstructionType {
//                 self.get_instruction_type()
//             }
//         }

//         impl $instructions {
//             pub fn get_instruction_type(&self) -> $instruction_types {
//                 match self {
//                     $(
//                         Self::$program(instruction) => {
//                             $instruction_types::$program(instruction.get_instruction_type())
//                         }
//                     ),+
//                 }
//             }
//         }
//     };
// }
