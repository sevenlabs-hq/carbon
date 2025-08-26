use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xda0c583a962c9848")]
pub struct DeactivateProtocolLookupTable {
    pub version: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct DeactivateProtocolLookupTableInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub address_lookup_program: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub lookup_table: solana_pubkey::Pubkey,
    pub protocol_owner_state: solana_pubkey::Pubkey,
    pub current_owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeactivateProtocolLookupTable {
    type ArrangedAccounts = DeactivateProtocolLookupTableInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let address_lookup_program = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let lookup_table = next_account(&mut iter)?;
        let protocol_owner_state = next_account(&mut iter)?;
        let current_owner = next_account(&mut iter)?;

        Some(DeactivateProtocolLookupTableInstructionAccounts {
            payer,
            system_program,
            address_lookup_program,
            authority,
            lookup_table,
            protocol_owner_state,
            current_owner,
        })
    }
}
