use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5e06ca73ff60e8b7")]
pub struct InitUserVolumeAccumulator {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitUserVolumeAccumulatorInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_volume_accumulator: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitUserVolumeAccumulator {
    type ArrangedAccounts = InitUserVolumeAccumulatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let user_volume_accumulator = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(InitUserVolumeAccumulatorInstructionAccounts {
            payer,
            user,
            user_volume_accumulator,
            system_program,
            event_authority,
            program,
        })
    }
}
