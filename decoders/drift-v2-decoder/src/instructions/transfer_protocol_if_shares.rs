use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5e5de2f0c3c9b86d")]
pub struct TransferProtocolIfShares {
    pub market_index: u16,
    pub shares: u128,
}

pub struct TransferProtocolIfSharesInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub transfer_config: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub spot_market: solana_pubkey::Pubkey,
    pub insurance_fund_stake: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub insurance_fund_vault: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferProtocolIfShares {
    type ArrangedAccounts = TransferProtocolIfSharesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [signer, transfer_config, state, spot_market, insurance_fund_stake, user_stats, authority, insurance_fund_vault, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(TransferProtocolIfSharesInstructionAccounts {
            signer: signer.pubkey,
            transfer_config: transfer_config.pubkey,
            state: state.pubkey,
            spot_market: spot_market.pubkey,
            insurance_fund_stake: insurance_fund_stake.pubkey,
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
            insurance_fund_vault: insurance_fund_vault.pubkey,
        })
    }
}
