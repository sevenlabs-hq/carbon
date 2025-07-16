use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x86addfb94d561c33")]
pub struct Order {
    pub global_config: solana_pubkey::Pubkey,
    pub maker: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub input_mint_program_id: solana_pubkey::Pubkey,
    pub output_mint: solana_pubkey::Pubkey,
    pub output_mint_program_id: solana_pubkey::Pubkey,
    pub initial_input_amount: u64,
    pub expected_output_amount: u64,
    pub remaining_input_amount: u64,
    pub filled_output_amount: u64,
    pub tip_amount: u64,
    pub number_of_fills: u64,
    pub order_type: u8,
    pub status: u8,
    pub in_vault_bump: u8,
    pub flash_ix_lock: u8,
    pub padding0: [u8; 4],
    pub last_updated_timestamp: u64,
    pub flash_start_taker_output_balance: u64,
    pub padding: [u64; 19],
}
