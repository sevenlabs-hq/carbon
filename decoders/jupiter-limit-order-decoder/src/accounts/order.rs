use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x86addfb94d561c33")]
pub struct Order {
    pub maker: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub output_mint: solana_pubkey::Pubkey,
    pub waiting: bool,
    pub ori_making_amount: u64,
    pub ori_taking_amount: u64,
    pub making_amount: u64,
    pub taking_amount: u64,
    pub maker_input_account: solana_pubkey::Pubkey,
    pub maker_output_account: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub borrow_making_amount: u64,
    pub expired_at: Option<i64>,
    pub base: solana_pubkey::Pubkey,
    pub referral: Option<solana_pubkey::Pubkey>,
}
