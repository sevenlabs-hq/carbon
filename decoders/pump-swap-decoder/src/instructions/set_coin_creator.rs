use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd295802dbc3a4eaf")]
pub struct SetCoinCreator {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetCoinCreatorInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetCoinCreator {
    type ArrangedAccounts = SetCoinCreatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, metadata, bonding_curve, event_authority, program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(SetCoinCreatorInstructionAccounts {
            pool: pool.pubkey,
            metadata: metadata.pubkey,
            bonding_curve: bonding_curve.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
