use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa75a899a4b2f1154")]
pub struct CreateLocker {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateLockerInstructionAccounts {
    pub virtual_pool: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub base: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub escrow: solana_pubkey::Pubkey,
    pub escrow_token: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub locker_program: solana_pubkey::Pubkey,
    pub locker_event_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateLocker {
    type ArrangedAccounts = CreateLockerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let virtual_pool = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let base = next_account(&mut iter)?;
        let creator = next_account(&mut iter)?;
        let escrow = next_account(&mut iter)?;
        let escrow_token = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let locker_program = next_account(&mut iter)?;
        let locker_event_authority = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateLockerInstructionAccounts {
            virtual_pool,
            config,
            pool_authority,
            base_vault,
            base_mint,
            base,
            creator,
            escrow,
            escrow_token,
            payer,
            token_program,
            locker_program,
            locker_event_authority,
            system_program,
        })
    }
}
