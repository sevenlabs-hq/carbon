use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0e7ae7da1feedf96")]
pub struct WithdrawFee {
    pub amount: u64,
}

pub struct WithdrawFeeInstructionAccounts {
    pub admin: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub program_fee_account: solana_sdk::pubkey::Pubkey,
    pub admin_token_acocunt: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawFee {
    type ArrangedAccounts = WithdrawFeeInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let admin = accounts.get(0)?;
        let fee_authority = accounts.get(1)?;
        let program_fee_account = accounts.get(2)?;
        let admin_token_acocunt = accounts.get(3)?;
        let token_program = accounts.get(4)?;
        let mint = accounts.get(5)?;

        Some(WithdrawFeeInstructionAccounts {
            admin: admin.pubkey,
            fee_authority: fee_authority.pubkey,
            program_fee_account: program_fee_account.pubkey,
            admin_token_acocunt: admin_token_acocunt.pubkey,
            token_program: token_program.pubkey,
            mint: mint.pubkey,
        })
    }
}
