use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf9036399a8f1f3e4")]
pub struct CreateProtocolLookupTable {
    pub version: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateProtocolLookupTableInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub address_lookup_program: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub lookup_table: solana_pubkey::Pubkey,
    pub protocol_owner_state: solana_pubkey::Pubkey,
    pub current_owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateProtocolLookupTable {
    type ArrangedAccounts = CreateProtocolLookupTableInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, system_program, address_lookup_program, authority, lookup_table, protocol_owner_state, current_owner, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateProtocolLookupTableInstructionAccounts {
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            address_lookup_program: address_lookup_program.pubkey,
            authority: authority.pubkey,
            lookup_table: lookup_table.pubkey,
            protocol_owner_state: protocol_owner_state.pubkey,
            current_owner: current_owner.pubkey,
        })
    }
}
