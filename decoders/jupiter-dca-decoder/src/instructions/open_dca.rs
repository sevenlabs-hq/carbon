use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2441b93601d264a3")]
pub struct OpenDca {
    pub application_idx: u64,
    pub in_amount: u64,
    pub in_amount_per_cycle: u64,
    pub cycle_frequency: i64,
    pub min_out_amount: Option<u64>,
    pub max_out_amount: Option<u64>,
    pub start_at: Option<i64>,
    pub close_wsol_in_ata: Option<bool>,
}

#[derive(Debug, PartialEq)]
pub struct OpenDcaInstructionAccounts {
    pub dca: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub output_mint: solana_pubkey::Pubkey,
    pub user_ata: solana_pubkey::Pubkey,
    pub in_ata: solana_pubkey::Pubkey,
    pub out_ata: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OpenDca {
    type ArrangedAccounts = OpenDcaInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [dca, user, input_mint, output_mint, user_ata, in_ata, out_ata, system_program, token_program, associated_token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(OpenDcaInstructionAccounts {
            dca: dca.pubkey,
            user: user.pubkey,
            input_mint: input_mint.pubkey,
            output_mint: output_mint.pubkey,
            user_ata: user_ata.pubkey,
            in_ata: in_ata.pubkey,
            out_ata: out_ata.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
