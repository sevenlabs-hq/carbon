use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc6d4ab6d90d7ae59")]
pub struct WithdrawFees {
    pub amount: u64,
}

pub struct WithdrawFeesInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub program_fee_ata: solana_sdk::pubkey::Pubkey,
    pub admin_fee_ata: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawFees {
    type ArrangedAccounts = WithdrawFeesInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let admin = accounts.get(0)?;
        let mint = accounts.get(1)?;
        let fee_authority = accounts.get(2)?;
        let program_fee_ata = accounts.get(3)?;
        let admin_fee_ata = accounts.get(4)?;
        let system_program = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let associated_token_program = accounts.get(7)?;

        Some(WithdrawFeesInstructionAccounts {
            admin: admin.pubkey,
            mint: mint.pubkey,
            fee_authority: fee_authority.pubkey,
            program_fee_ata: program_fee_ata.pubkey,
            admin_fee_ata: admin_fee_ata.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
        })
    }
}
