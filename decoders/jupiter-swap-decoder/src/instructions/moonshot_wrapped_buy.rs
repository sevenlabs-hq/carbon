use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcf96d59c8a68ee8e")]
pub struct MoonshotWrappedBuy {}

pub struct MoonshotWrappedBuyInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub sender: solana_sdk::pubkey::Pubkey,
    pub sender_token_account: solana_sdk::pubkey::Pubkey,
    pub curve_account: solana_sdk::pubkey::Pubkey,
    pub curve_token_account: solana_sdk::pubkey::Pubkey,
    pub dex_fee: solana_sdk::pubkey::Pubkey,
    pub helio_fee: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub config_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub user_wsol_token_account: solana_sdk::pubkey::Pubkey,
    pub temp_wsol_token_account: solana_sdk::pubkey::Pubkey,
    pub wsol_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MoonshotWrappedBuy {
    type ArrangedAccounts = MoonshotWrappedBuyInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let sender = accounts.get(1)?;
        let sender_token_account = accounts.get(2)?;
        let curve_account = accounts.get(3)?;
        let curve_token_account = accounts.get(4)?;
        let dex_fee = accounts.get(5)?;
        let helio_fee = accounts.get(6)?;
        let mint = accounts.get(7)?;
        let config_account = accounts.get(8)?;
        let token_program = accounts.get(9)?;
        let associated_token_program = accounts.get(10)?;
        let system_program = accounts.get(11)?;
        let user_wsol_token_account = accounts.get(12)?;
        let temp_wsol_token_account = accounts.get(13)?;
        let wsol_mint = accounts.get(14)?;

        Some(MoonshotWrappedBuyInstructionAccounts {
            swap_program: swap_program.pubkey,
            sender: sender.pubkey,
            sender_token_account: sender_token_account.pubkey,
            curve_account: curve_account.pubkey,
            curve_token_account: curve_token_account.pubkey,
            dex_fee: dex_fee.pubkey,
            helio_fee: helio_fee.pubkey,
            mint: mint.pubkey,
            config_account: config_account.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            user_wsol_token_account: user_wsol_token_account.pubkey,
            temp_wsol_token_account: temp_wsol_token_account.pubkey,
            wsol_mint: wsol_mint.pubkey,
        })
    }
}
