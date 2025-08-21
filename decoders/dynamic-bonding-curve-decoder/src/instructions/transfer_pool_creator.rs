use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1407a9213a93a621")]
pub struct TransferPoolCreator {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TransferPoolCreatorInstructionAccounts {
    pub virtual_pool: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub new_creator: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferPoolCreator {
    type ArrangedAccounts = TransferPoolCreatorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let virtual_pool = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let creator = next_account(&mut iter)?;
        let new_creator = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(TransferPoolCreatorInstructionAccounts {
            virtual_pool,
            config,
            creator,
            new_creator,
            event_authority,
            program,
        })
    }
}
