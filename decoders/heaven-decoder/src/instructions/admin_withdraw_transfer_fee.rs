use carbon_core::{borsh, CarbonDeserialize};

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
        let [token_program, mint, receiver, protocol_fee_admin_state, admin, protocol_config, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AdminWithdrawTransferFeeInstructionAccounts {
            token_program: token_program.pubkey,
            mint: mint.pubkey,
            receiver: receiver.pubkey,
            protocol_fee_admin_state: protocol_fee_admin_state.pubkey,
            admin: admin.pubkey,
            protocol_config: protocol_config.pubkey,
        })
    }
}
