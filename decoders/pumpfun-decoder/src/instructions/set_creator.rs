use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfe94ff70cf8eaaa5")]
pub struct SetCreator {
    pub creator: solana_pubkey::Pubkey,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetCreatorInstructionAccounts {
    pub set_creator_authority: solana_pubkey::Pubkey,
    pub global: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetCreator {
    type ArrangedAccounts = SetCreatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [set_creator_authority, global, mint, metadata, bonding_curve, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SetCreatorInstructionAccounts {
            set_creator_authority: set_creator_authority.pubkey,
            global: global.pubkey,
            mint: mint.pubkey,
            metadata: metadata.pubkey,
            bonding_curve: bonding_curve.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
