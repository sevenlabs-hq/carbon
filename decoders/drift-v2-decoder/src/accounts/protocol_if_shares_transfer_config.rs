use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbc01d56217941e01")]
pub struct ProtocolIfSharesTransferConfig {
    pub whitelisted_signers: [solana_pubkey::Pubkey; 4],
    pub max_transfer_per_epoch: u128,
    pub current_epoch_transfer: u128,
    pub next_epoch_ts: i64,
    pub padding: [u128; 8],
}
