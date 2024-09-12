use crate::collection::InstructionDecoderCollection;
use crate::instruction::DecodedInstruction;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone)]
pub enum SchemaNode<T: InstructionDecoderCollection> {
    Instruction(T::InstructionType),
    Any,
    AnyUnparsed,
    Sequence(Vec<SchemaNode<T>>),
    OneOf(Vec<SchemaNode<T>>),
    ZeroOrMore(Box<SchemaNode<T>>),
    OneOrMore(Box<SchemaNode<T>>),
    Optional(Box<SchemaNode<T>>),
    Nested(Box<SchemaNode<T>>),
}

#[derive(Debug)]
pub struct ParsedInstruction<T: InstructionDecoderCollection> {
    pub program_id: Pubkey,
    pub instruction: DecodedInstruction<T>,
    pub inner_instructions: Vec<ParsedInstruction<T>>,
}

impl<T: InstructionDecoderCollection> SchemaNode<T> {
    pub fn matches(&self, instruction: &ParsedInstruction<T>) -> bool {
        match self {
            SchemaNode::Instruction(expected_type) => {
                instruction.instruction.data.get_type() == *expected_type
            }
            SchemaNode::Any => true,
            SchemaNode::AnyUnparsed => false,
            SchemaNode::Sequence(nodes) => nodes.len() == 1 && nodes[0].matches(instruction),
            SchemaNode::OneOf(nodes) => nodes.iter().any(|node| node.matches(instruction)),
            SchemaNode::ZeroOrMore(_) => true,
            SchemaNode::OneOrMore(node) => node.matches(instruction),
            SchemaNode::Optional(_) => true,
            SchemaNode::Nested(node) => instruction
                .inner_instructions
                .iter()
                .any(|inner| node.matches(inner)),
        }
    }
}

#[derive(Debug)]
pub struct TransactionSchema<T: InstructionDecoderCollection> {
    pub root: SchemaNode<T>,
}

impl<T: InstructionDecoderCollection> TransactionSchema<T> {
    pub fn matches(&self, instructions: &[ParsedInstruction<T>]) -> bool {
        self.match_instructions(&self.root, instructions)
    }

    fn match_instructions(
        &self,
        node: &SchemaNode<T>,
        instructions: &[ParsedInstruction<T>],
    ) -> bool {
        match node {
            SchemaNode::Instruction(_) | SchemaNode::Any | SchemaNode::AnyUnparsed => {
                instructions.len() == 1 && node.matches(&instructions[0])
            }
            SchemaNode::Sequence(nodes) => {
                if nodes.len() != instructions.len() {
                    return false;
                }
                nodes
                    .iter()
                    .zip(instructions)
                    .all(|(node, instr)| self.match_instructions(node, std::slice::from_ref(instr)))
            }
            SchemaNode::OneOf(nodes) => nodes
                .iter()
                .any(|node| self.match_instructions(node, instructions)),
            SchemaNode::ZeroOrMore(inner_node) => {
                instructions.is_empty()
                    || instructions.iter().all(|instr| {
                        self.match_instructions(inner_node, std::slice::from_ref(instr))
                    })
            }
            SchemaNode::OneOrMore(inner_node) => {
                !instructions.is_empty()
                    && instructions.iter().all(|instr| {
                        self.match_instructions(inner_node, std::slice::from_ref(instr))
                    })
            }
            SchemaNode::Optional(inner_node) => {
                instructions.is_empty() || self.match_instructions(inner_node, instructions)
            }
            SchemaNode::Nested(inner_node) => instructions.iter().all(|instr| {
                instr.inner_instructions.is_empty()
                    || self.match_instructions(inner_node, &instr.inner_instructions)
            }),
        }
    }
}
