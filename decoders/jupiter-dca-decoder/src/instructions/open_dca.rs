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

pub struct OpenDcaInstructionAccounts {
    pub dca: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub user_ata: solana_sdk::pubkey::Pubkey,
    pub in_ata: solana_sdk::pubkey::Pubkey,
    pub out_ata: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OpenDca {
    type ArrangedAccounts = OpenDcaInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let dca = accounts.get(0)?;
        let user = accounts.get(1)?;
        let input_mint = accounts.get(2)?;
        let output_mint = accounts.get(3)?;
        let user_ata = accounts.get(4)?;
        let in_ata = accounts.get(5)?;
        let out_ata = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let token_program = accounts.get(8)?;
        let associated_token_program = accounts.get(9)?;
        let event_authority = accounts.get(10)?;
        let program = accounts.get(11)?;

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
