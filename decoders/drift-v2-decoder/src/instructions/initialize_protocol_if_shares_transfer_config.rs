use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5983efc8b28d6ac2")]
pub struct InitializeProtocolIfSharesTransferConfig {}

pub struct InitializeProtocolIfSharesTransferConfigInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub protocol_if_shares_transfer_config: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeProtocolIfSharesTransferConfig {
    type ArrangedAccounts = InitializeProtocolIfSharesTransferConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, protocol_if_shares_transfer_config, state, rent, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(
            InitializeProtocolIfSharesTransferConfigInstructionAccounts {
                admin: admin.pubkey,
                protocol_if_shares_transfer_config: protocol_if_shares_transfer_config.pubkey,
                state: state.pubkey,
                rent: rent.pubkey,
                system_program: system_program.pubkey,
            },
        )
    }
}
