use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfc681286a44e128c")]
pub struct FlashFillOrder {
    pub max_taking_amount: u64,
}

pub struct FlashFillOrderInstructionAccounts {
    pub order: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub taker: solana_sdk::pubkey::Pubkey,
    pub maker_output_account: solana_sdk::pubkey::Pubkey,
    pub taker_input_account: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub program_fee_account: solana_sdk::pubkey::Pubkey,
    pub referral: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub input_mint_token_program: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub output_mint_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FlashFillOrder {
    type ArrangedAccounts = FlashFillOrderInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let order = accounts.get(0)?;
        let reserve = accounts.get(1)?;
        let maker = accounts.get(2)?;
        let taker = accounts.get(3)?;
        let maker_output_account = accounts.get(4)?;
        let taker_input_account = accounts.get(5)?;
        let fee_authority = accounts.get(6)?;
        let program_fee_account = accounts.get(7)?;
        let referral = accounts.get(8)?;
        let input_mint = accounts.get(9)?;
        let input_mint_token_program = accounts.get(10)?;
        let output_mint = accounts.get(11)?;
        let output_mint_token_program = accounts.get(12)?;
        let system_program = accounts.get(13)?;

        Some(FlashFillOrderInstructionAccounts {
            order: order.pubkey,
            reserve: reserve.pubkey,
            maker: maker.pubkey,
            taker: taker.pubkey,
            maker_output_account: maker_output_account.pubkey,
            taker_input_account: taker_input_account.pubkey,
            fee_authority: fee_authority.pubkey,
            program_fee_account: program_fee_account.pubkey,
            referral: referral.pubkey,
            input_mint: input_mint.pubkey,
            input_mint_token_program: input_mint_token_program.pubkey,
            output_mint: output_mint.pubkey,
            output_mint_token_program: output_mint_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
