use carbon_core::{
    deserialization::{Discriminator, InstructionAccounts, InstructionData},
    error::{CarbonResult, Error},
};
use meteora_swap::{MeteoraSwapInstructionAccounts, MeteoraSwapInstructionData};

pub mod meteora_swap;

#[derive(Debug, Clone, Eq, Hash, PartialEq, serde::Serialize)]
pub enum MeteoraInstruction {
    Swap {
        data: MeteoraSwapInstructionData,
        accounts: MeteoraSwapInstructionAccounts,
    },
    // SwapEvent {
    //     data: MeteoraSwapEventInstructionData,
    //     accounts: MeteoraSwapEventInstructionAccounts,
    // },
}

impl MeteoraInstruction {
    pub fn unpack(input: &[u8]) -> CarbonResult<Self> {
        let discriminator = match input.len() {
            _ => Discriminator::eight_bytes_from_slice(input)?,
        };
        let ix = match discriminator {
            discriminator if discriminator == MeteoraSwapInstructionData::discriminator() => {
                // TODO: Revise errors here
                MeteoraInstruction::Swap {
                    data: InstructionData::unpack(input).map_err(|_| Error::InvalidDataLength)?,
                    // ASK: Should we even have accounts here? I'm having trouble with AccountMeta not having hash too... In this carbon design It looks like it's only data here?
                    accounts: MeteoraSwapInstructionAccounts::unpack(input)
                        .map_err(|_| Error::MissingAccountInInstruction)?,
                }
            }
            // discriminator if discriminator == MeteoraSwapEventInstructionData::discriminator() => {
            //     MeteoraInstruction::SwapEvent {
            //         data: InstructionData::unpack(input).map_err(Error::msg)?,
            //         accounts: MeteoraSwapEventInstructionAccounts::unpack(input)
            //             .map_err(Error::msg)?,
            //     }
            // }
            _discriminator => return Err(Error::MissingInstructionData),
        };

        Ok(ix)
    }
}
