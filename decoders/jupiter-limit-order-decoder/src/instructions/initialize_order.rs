use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x856e4aaf709ff59f")]
pub struct InitializeOrder {
    pub making_amount: u64,
    pub taking_amount: u64,
    pub expired_at: Option<i64>,
}

pub struct InitializeOrderInstructionAccounts {
    pub base: solana_sdk::pubkey::Pubkey,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub order: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub maker_input_account: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub maker_output_account: solana_sdk::pubkey::Pubkey,
    pub referral: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeOrder {
    type ArrangedAccounts = InitializeOrderInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let base = accounts.get(0)?;
        let maker = accounts.get(1)?;
        let order = accounts.get(2)?;
        let reserve = accounts.get(3)?;
        let maker_input_account = accounts.get(4)?;
        let input_mint = accounts.get(5)?;
        let maker_output_account = accounts.get(6)?;
        let referral = accounts.get(7)?;
        let output_mint = accounts.get(8)?;
        let system_program = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let rent = accounts.get(11)?;

        Some(InitializeOrderInstructionAccounts {
            base: base.pubkey,
            maker: maker.pubkey,
            order: order.pubkey,
            reserve: reserve.pubkey,
            maker_input_account: maker_input_account.pubkey,
            input_mint: input_mint.pubkey,
            maker_output_account: maker_output_account.pubkey,
            referral: referral.pubkey,
            output_mint: output_mint.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
