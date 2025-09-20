//! Compatibility helpers for converting inner instructions / UiInstruction variants
//! from different client versions into the workspace's expected types.

use serde_json::Value;
use solana_message::compiled_instruction::CompiledInstruction;
use solana_transaction_status::{InnerInstruction, InnerInstructions};

/// Convert an optional, versioned inner instructions structure (from an external
/// crate version) into the modern `Vec<InnerInstructions>` used by the
/// workspace. We accept any structure that serializes to the expected JSON
/// shape and attempt to extract "Compiled" instruction variants. Unknown or
/// unsupported instruction types are converted to an empty CompiledInstruction
/// placeholder while preserving stack height when available.
pub fn convert_inner_instructions<T: serde::Serialize>(
    source: Option<Vec<T>>,
) -> Vec<InnerInstructions> {
    let json = match serde_json::to_value(&source) {
        Ok(v) => v,
        Err(_) => return vec![],
    };

    let arr = match json {
        Value::Array(a) => a,
        // If serialization produced something else (including null), bail out.
        _ => return vec![],
    };

    let mut out = Vec::with_capacity(arr.len());

    for group in arr.iter() {
        let index = group.get("index").and_then(|v| v.as_u64()).unwrap_or(0) as u8;

        let instructions_json = group.get("instructions").and_then(|v| v.as_array());

        let mut instrs: Vec<InnerInstruction> = Vec::new();

        if let Some(instructions_json) = instructions_json {
            for instr in instructions_json.iter() {
                // The UiInstruction::Compiled variant can be serialized in a
                // few different shapes depending on the source crate's
                // Serialize impl. Try a couple of common representations.
                let compiled_obj = if let Some(compiled) = instr.get("Compiled") {
                    Some(compiled)
                } else if instr.get("program_id_index").is_some() {
                    Some(instr)
                } else {
                    None
                };

                if let Some(obj) = compiled_obj {
                    let program_id_index = obj
                        .get("program_id_index")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0) as u8;

                    let accounts = obj
                        .get("accounts")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|x| x.as_u64().map(|n| n as u8))
                                .collect::<Vec<u8>>()
                        })
                        .unwrap_or_else(std::vec::Vec::new);

                    let data_b58 = obj.get("data").and_then(|v| v.as_str()).unwrap_or("");
                    let data = bs58::decode(data_b58).into_vec().unwrap_or_default();

                    let stack_height = obj
                        .get("stack_height")
                        .and_then(|v| v.as_u64())
                        .map(|n| n as u32);

                    let inner = InnerInstruction {
                        instruction: CompiledInstruction {
                            program_id_index,
                            accounts,
                            data,
                        },
                        stack_height: stack_height,
                    };

                    instrs.push(inner);
                } else {
                    // Unsupported/unknown instruction shape — emit placeholder
                    instrs.push(InnerInstruction {
                        instruction: CompiledInstruction {
                            program_id_index: 0,
                            accounts: vec![],
                            data: vec![],
                        },
                        stack_height: None,
                    });
                }
            }
        }

        out.push(InnerInstructions {
            index,
            instructions: instrs,
        });
    }

    out
}
