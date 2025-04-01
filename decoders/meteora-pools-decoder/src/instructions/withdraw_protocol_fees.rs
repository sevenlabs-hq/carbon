use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0b44a56212d08649")]
pub struct WithdrawProtocolFees {}

pub struct WithdrawProtocolFeesInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub a_vault_lp: solana_pubkey::Pubkey,
    pub protocol_token_a_fee: solana_pubkey::Pubkey,
    pub protocol_token_b_fee: solana_pubkey::Pubkey,
    pub treasury_token_a: solana_pubkey::Pubkey,
    pub treasury_token_b: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawProtocolFees {
    type ArrangedAccounts = WithdrawProtocolFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, a_vault_lp, protocol_token_a_fee, protocol_token_b_fee, treasury_token_a, treasury_token_b, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawProtocolFeesInstructionAccounts {
            pool: pool.pubkey,
            a_vault_lp: a_vault_lp.pubkey,
            protocol_token_a_fee: protocol_token_a_fee.pubkey,
            protocol_token_b_fee: protocol_token_b_fee.pubkey,
            treasury_token_a: treasury_token_a.pubkey,
            treasury_token_b: treasury_token_b.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
