use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x74ce1bbfa6130049")]
pub struct ClaimToken {
    pub id: u8,
}

pub struct ClaimTokenInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub program_authority: solana_sdk::pubkey::Pubkey,
    pub program_token_account: solana_sdk::pubkey::Pubkey,
    pub destination_token_account: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub associated_token_token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimToken {
    type ArrangedAccounts = ClaimTokenInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let wallet = accounts.get(1)?;
        let program_authority = accounts.get(2)?;
        let program_token_account = accounts.get(3)?;
        let destination_token_account = accounts.get(4)?;
        let mint = accounts.get(5)?;
        let associated_token_token_program = accounts.get(6)?;
        let associated_token_program = accounts.get(7)?;
        let system_program = accounts.get(8)?;

        Some(ClaimTokenInstructionAccounts {
            payer: payer.pubkey,
            wallet: wallet.pubkey,
            program_authority: program_authority.pubkey,
            program_token_account: program_token_account.pubkey,
            destination_token_account: destination_token_account.pubkey,
            mint: mint.pubkey,
            associated_token_token_program: associated_token_token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
