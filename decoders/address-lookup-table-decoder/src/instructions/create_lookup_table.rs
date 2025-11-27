use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00000000")]
pub struct CreateLookupTable {
    pub recent_slot: u64,
    pub bump_seed: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateLookupTableInstructionAccounts {
    pub look_up_table: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateLookupTable {
    type ArrangedAccounts = CreateLookupTableInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [look_up_table, authority, funder, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateLookupTableInstructionAccounts {
            look_up_table: look_up_table.pubkey,
            authority: authority.pubkey,
            funder: funder.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
