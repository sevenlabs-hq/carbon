use crate::{LimitlessLaunchpadDecoder, PROGRAM_ID};

pub mod create_and_first_buy;
pub mod create_or_buy_pda;
pub mod graduate;
pub mod launch_coin;
pub mod trade;

pub use self::{
    create_and_first_buy::*, create_or_buy_pda::*, graduate::*, launch_coin::*, trade::*,
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "data"))]
#[cfg_attr(feature = "serde", derive(carbon_core::InstructionType))]
pub enum LimitlessLaunchpadInstruction {
    LaunchCoin {
        program_id: solana_pubkey::Pubkey,
        data: LaunchCoin,
        accounts: LaunchCoinInstructionAccounts,
    },
    CreateAndFirstBuy {
        program_id: solana_pubkey::Pubkey,
        data: CreateAndFirstBuy,
        accounts: CreateAndFirstBuyInstructionAccounts,
    },
    CreateOrBuyPda {
        program_id: solana_pubkey::Pubkey,
        data: CreateOrBuyPda,
        accounts: CreateOrBuyPdaInstructionAccounts,
    },
    CreateOrBuyPdaWithVanityFee {
        program_id: solana_pubkey::Pubkey,
        data: CreateOrBuyPdaWithVanityFee,
        accounts: CreateOrBuyPdaInstructionAccounts,
    },
    Buy {
        program_id: solana_pubkey::Pubkey,
        data: Buy,
        accounts: BuyInstructionAccounts,
    },
    Sell {
        program_id: solana_pubkey::Pubkey,
        data: Sell,
        accounts: SellInstructionAccounts,
    },
    Graduate {
        program_id: solana_pubkey::Pubkey,
        data: Graduate,
        accounts: GraduateInstructionAccounts,
    },
    PrepareMeteoraGraduation {
        program_id: solana_pubkey::Pubkey,
        data: PrepareMeteoraGraduation,
        accounts: MeteoraGraduationInstructionAccounts,
    },
    GraduateMeteora {
        program_id: solana_pubkey::Pubkey,
        data: GraduateMeteora,
        accounts: MeteoraGraduationInstructionAccounts,
    },
    ClaimMeteoraFees {
        program_id: solana_pubkey::Pubkey,
        data: ClaimMeteoraFees,
        accounts: ClaimMeteoraFeesInstructionAccounts,
    },
}

impl carbon_core::instruction::InstructionDecoder<'_> for LimitlessLaunchpadDecoder {
    type InstructionType = LimitlessLaunchpadInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<Self::InstructionType> {
        if instruction.program_id != PROGRAM_ID {
            return None;
        }

        carbon_core::try_decode_instructions!(
            instruction,
            PROGRAM_ID,
            LimitlessLaunchpadInstruction::LaunchCoin => LaunchCoin,
            LimitlessLaunchpadInstruction::CreateAndFirstBuy => CreateAndFirstBuy,
            LimitlessLaunchpadInstruction::CreateOrBuyPda => CreateOrBuyPda,
            LimitlessLaunchpadInstruction::CreateOrBuyPdaWithVanityFee => CreateOrBuyPdaWithVanityFee,
            LimitlessLaunchpadInstruction::Buy => Buy,
            LimitlessLaunchpadInstruction::Sell => Sell,
            LimitlessLaunchpadInstruction::Graduate => Graduate,
            LimitlessLaunchpadInstruction::PrepareMeteoraGraduation => PrepareMeteoraGraduation,
            LimitlessLaunchpadInstruction::GraduateMeteora => GraduateMeteora,
            LimitlessLaunchpadInstruction::ClaimMeteoraFees => ClaimMeteoraFees,
        )
    }
}
