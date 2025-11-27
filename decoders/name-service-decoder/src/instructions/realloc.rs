use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x04")]
pub struct Realloc {
    pub space: u32,
}

pub struct ReallocInstructionAccounts {
    pub system_program: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub name_record: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Realloc {
    type ArrangedAccounts = ReallocInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [system_program, payer, name_record, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ReallocInstructionAccounts {
            system_program: system_program.pubkey,
            payer: payer.pubkey,
            name_record: name_record.pubkey,
            owner: owner.pubkey,
        })
    }
}
