use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00")]
pub struct Initialize {
    pub fees: Fees,
    pub swap_curve: SwapCurve,
}

#[derive(Debug, PartialEq)]
pub struct InitializeInstructionAccounts {
    pub swap: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub token_a: solana_pubkey::Pubkey,
    pub token_b: solana_pubkey::Pubkey,
    pub pool: solana_pubkey::Pubkey,
    pub fee: solana_pubkey::Pubkey,
    pub destination: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [swap, authority, token_a, token_b, pool, fee, destination, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeInstructionAccounts {
            swap: swap.pubkey,
            authority: authority.pubkey,
            token_a: token_a.pubkey,
            token_b: token_b.pubkey,
            pool: pool.pubkey,
            fee: fee.pubkey,
            destination: destination.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
