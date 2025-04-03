use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0e7ae7da1feedf96")]
pub struct WithdrawFee {
    pub amount: u64,
}

pub struct WithdrawFeeInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
    pub program_fee_account: solana_pubkey::Pubkey,
    pub admin_token_acocunt: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawFee {
    type ArrangedAccounts = WithdrawFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, fee_authority, program_fee_account, admin_token_acocunt, token_program, mint, _remaining @ ..] =
            accounts
        else {
            return None;
        };

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
