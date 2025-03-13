use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x854d1262d301e703")]
pub struct TicketAccountData {
    pub state_address: solana_sdk::pubkey::Pubkey,
    pub beneficiary: solana_sdk::pubkey::Pubkey,
    pub lamports_amount: u64,
    pub created_epoch: u64,
}
