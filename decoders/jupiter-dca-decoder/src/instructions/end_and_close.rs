use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x537da645f7fc6785")]
pub struct EndAndClose {}

pub struct EndAndCloseInstructionAccounts {
    pub keeper: solana_sdk::pubkey::Pubkey,
    pub dca: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub in_ata: solana_sdk::pubkey::Pubkey,
    pub out_ata: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub user_out_ata: solana_sdk::pubkey::Pubkey,
    pub init_user_out_ata: solana_sdk::pubkey::Pubkey,
    pub intermediate_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EndAndClose {
    type ArrangedAccounts = EndAndCloseInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let keeper = accounts.get(0)?;
        let dca = accounts.get(1)?;
        let input_mint = accounts.get(2)?;
        let output_mint = accounts.get(3)?;
        let in_ata = accounts.get(4)?;
        let out_ata = accounts.get(5)?;
        let user = accounts.get(6)?;
        let user_out_ata = accounts.get(7)?;
        let init_user_out_ata = accounts.get(8)?;
        let intermediate_account = accounts.get(9)?;
        let system_program = accounts.get(10)?;
        let token_program = accounts.get(11)?;
        let associated_token_program = accounts.get(12)?;
        let event_authority = accounts.get(13)?;
        let program = accounts.get(14)?;

        Some(EndAndCloseInstructionAccounts {
            keeper: keeper.pubkey,
            dca: dca.pubkey,
            input_mint: input_mint.pubkey,
            output_mint: output_mint.pubkey,
            in_ata: in_ata.pubkey,
            out_ata: out_ata.pubkey,
            user: user.pubkey,
            user_out_ata: user_out_ata.pubkey,
            init_user_out_ata: init_user_out_ata.pubkey,
            intermediate_account: intermediate_account.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
