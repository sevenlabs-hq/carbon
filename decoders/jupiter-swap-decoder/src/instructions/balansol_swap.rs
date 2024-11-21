use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x896dfdfd466d0b64")]
pub struct BalansolSwap {}

pub struct BalansolSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub tax_man: solana_sdk::pubkey::Pubkey,
    pub bid_mint: solana_sdk::pubkey::Pubkey,
    pub treasurer: solana_sdk::pubkey::Pubkey,
    pub src_treasury: solana_sdk::pubkey::Pubkey,
    pub src_associated_token_account: solana_sdk::pubkey::Pubkey,
    pub ask_mint: solana_sdk::pubkey::Pubkey,
    pub dst_treasury: solana_sdk::pubkey::Pubkey,
    pub dst_associated_token_account: solana_sdk::pubkey::Pubkey,
    pub dst_token_account_taxman: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BalansolSwap {
    type ArrangedAccounts = BalansolSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let authority = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let tax_man = accounts.get(3)?;
        let bid_mint = accounts.get(4)?;
        let treasurer = accounts.get(5)?;
        let src_treasury = accounts.get(6)?;
        let src_associated_token_account = accounts.get(7)?;
        let ask_mint = accounts.get(8)?;
        let dst_treasury = accounts.get(9)?;
        let dst_associated_token_account = accounts.get(10)?;
        let dst_token_account_taxman = accounts.get(11)?;
        let system_program = accounts.get(12)?;
        let token_program = accounts.get(13)?;
        let associated_token_program = accounts.get(14)?;
        let rent = accounts.get(15)?;

        Some(BalansolSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            authority: authority.pubkey,
            pool: pool.pubkey,
            tax_man: tax_man.pubkey,
            bid_mint: bid_mint.pubkey,
            treasurer: treasurer.pubkey,
            src_treasury: src_treasury.pubkey,
            src_associated_token_account: src_associated_token_account.pubkey,
            ask_mint: ask_mint.pubkey,
            dst_treasury: dst_treasury.pubkey,
            dst_associated_token_account: dst_associated_token_account.pubkey,
            dst_token_account_taxman: dst_token_account_taxman.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
