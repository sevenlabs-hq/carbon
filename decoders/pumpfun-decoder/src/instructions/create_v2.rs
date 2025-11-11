use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd6904cec5f8b31b4")]
pub struct CreateV2 {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creator: solana_pubkey::Pubkey,
    pub is_mayhem_mode: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateV2InstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub associated_bonding_curve: solana_pubkey::Pubkey,
    pub global: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub mayhem_program_id: solana_pubkey::Pubkey,
    pub global_params: solana_pubkey::Pubkey,
    pub sol_vault: solana_pubkey::Pubkey,
    pub mayhem_state: solana_pubkey::Pubkey,
    pub mayhem_token_vault: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateV2 {
    type ArrangedAccounts = CreateV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let mint = next_account(&mut iter)?;
        let mint_authority = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let associated_bonding_curve = next_account(&mut iter)?;
        let global = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let mayhem_program_id = next_account(&mut iter)?;
        let global_params = next_account(&mut iter)?;
        let sol_vault = next_account(&mut iter)?;
        let mayhem_state = next_account(&mut iter)?;
        let mayhem_token_vault = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        Some(CreateV2InstructionAccounts {
            mint,
            mint_authority,
            bonding_curve,
            associated_bonding_curve,
            global,
            user,
            system_program,
            token_program,
            associated_token_program,
            mayhem_program_id,
            global_params,
            sol_vault,
            mayhem_state,
            mayhem_token_vault,
            event_authority,
            program,
        })
    }
}
