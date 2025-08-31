use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3ec6d6c1d59f6cd2")]
pub struct Claim {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub claimer: solana_pubkey::Pubkey,
    pub mint_a: solana_pubkey::Pubkey,
    pub vault_a: solana_pubkey::Pubkey,
    pub receiver_ta_a: solana_pubkey::Pubkey,
    pub token_program_a: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Claim {
    type ArrangedAccounts = ClaimInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, system_program, claimer, mint_a, vault_a, receiver_ta_a, token_program_a, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ClaimInstructionAccounts {
            pool: pool.pubkey,
            system_program: system_program.pubkey,
            claimer: claimer.pubkey,
            mint_a: mint_a.pubkey,
            vault_a: vault_a.pubkey,
            receiver_ta_a: receiver_ta_a.pubkey,
            token_program_a: token_program_a.pubkey,
        })
    }
}
