use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x754fa4cb7e4816f6")]
pub struct AdminWithdrawTransferFee {
    pub protocol_config_version: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct AdminWithdrawTransferFeeInstructionAccounts {
    pub token_program: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub receiver: solana_pubkey::Pubkey,
    pub protocol_fee_admin_state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub protocol_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AdminWithdrawTransferFee {
    type ArrangedAccounts = AdminWithdrawTransferFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let token_program = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let receiver = next_account(&mut iter)?;
        let protocol_fee_admin_state = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;
        let protocol_config = next_account(&mut iter)?;

        Some(AdminWithdrawTransferFeeInstructionAccounts {
            token_program,
            mint,
            receiver,
            protocol_fee_admin_state,
            admin,
            protocol_config,
        })
    }
}
