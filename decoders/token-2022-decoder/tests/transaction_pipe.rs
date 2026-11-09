//! Token-2022's generated instruction enum in a transaction pipe. Some of its
//! instructions carry floats, so the enum can never be `Eq` or `Hash`, and a
//! collection that required them could not wrap it.
#![cfg(feature = "serde")]

use {
    carbon_core::{
        collection::InstructionDecoderCollection,
        error::CarbonResult,
        instruction::{InstructionDecoder, InstructionMetadata},
        instruction_decoder_collection,
        pipeline::Pipeline,
        processor::Processor,
        transaction::{
            parse_instructions_flat, TransactionMetadata, TransactionProcessorInputType,
        },
    },
    carbon_token_2022_decoder::{
        instructions::{Token2022Instruction, Token2022InstructionType},
        Token2022Decoder, PROGRAM_ID,
    },
    solana_pubkey::Pubkey,
    spl_token_2022::extension::scaled_ui_amount::instruction::update_multiplier,
    std::sync::Arc,
};

instruction_decoder_collection!(
    Instructions, InstructionTypes, Programs,
    Token2022 => PROGRAM_ID => Token2022Decoder => Token2022Instruction,
);

struct Ignore;

impl Processor<TransactionProcessorInputType<'_, Instructions>> for Ignore {
    async fn process(
        &mut self,
        _data: &TransactionProcessorInputType<'_, Instructions>,
    ) -> CarbonResult<()> {
        Ok(())
    }
}

fn key(seed: u8) -> Pubkey {
    Pubkey::new_from_array([seed; 32])
}

#[test]
fn a_transaction_pipe_accepts_the_generated_enum() {
    let _ = Pipeline::builder().transaction::<Instructions, _>(Ignore);
}

#[test]
fn the_collection_decodes_an_instruction_that_carries_a_float() {
    let instruction = update_multiplier(
        &spl_token_2022::ID,
        &key(1),
        &key(2),
        &[],
        1.5,
        1_800_000_000,
    )
    .unwrap();
    let metadata = InstructionMetadata {
        transaction_metadata: Arc::new(TransactionMetadata::default()),
        stack_height: 1,
        index: 0,
        absolute_path: vec![0],
    };

    let decoded = parse_instructions_flat::<Instructions>(&[(metadata, instruction)]);

    let [(_, parsed)] = decoded.as_slice() else {
        panic!("expected one decoded instruction, got {}", decoded.len());
    };
    let Instructions::Token2022(Token2022Instruction::UpdateMultiplierScaledUiMint {
        data, ..
    }) = parsed
    else {
        panic!("decoded as another instruction: {parsed:?}");
    };
    assert_eq!(data.multiplier, 1.5);
    assert_eq!(
        parsed.get_type(),
        InstructionTypes::Token2022(Token2022InstructionType::UpdateMultiplierScaledUiMint)
    );
}
