use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x22872f5bdc18d435")]
pub struct UpdateProtocolIfSharesTransferConfig {
    pub whitelisted_signers: Option<[solana_pubkey::Pubkey; 4]>,
    pub max_transfer_per_epoch: Option<u128>,
}

pub struct UpdateProtocolIfSharesTransferConfigInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub protocol_if_shares_transfer_config: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateProtocolIfSharesTransferConfig {
    type ArrangedAccounts = UpdateProtocolIfSharesTransferConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, protocol_if_shares_transfer_config, state, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateProtocolIfSharesTransferConfigInstructionAccounts {
            admin: admin.pubkey,
            protocol_if_shares_transfer_config: protocol_if_shares_transfer_config.pubkey,
            state: state.pubkey,
        })
    }
}
