use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d476fbe76070384de")]
pub struct DeleteUserRecordEvent {
    pub ts: i64,
    pub user_authority: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub sub_account_id: u16,
    pub keeper: Option<solana_pubkey::Pubkey>,
}
