use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x04000000")]
pub struct CloseLookupTable {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseLookupTableInstructionAccounts {
    pub look_up_table: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub lamports_recipient: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseLookupTable {
    type ArrangedAccounts = CloseLookupTableInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [look_up_table, authority, lamports_recipient, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CloseLookupTableInstructionAccounts {
            look_up_table: look_up_table.pubkey,
            authority: authority.pubkey,
            lamports_recipient: lamports_recipient.pubkey,
        })
    }
}
