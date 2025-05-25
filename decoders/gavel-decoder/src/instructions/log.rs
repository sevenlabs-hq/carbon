use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x08")]
pub struct Log {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct LogInstructionAccounts {
    pub log_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Log {
    type ArrangedAccounts = LogInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [log_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(LogInstructionAccounts {
            log_authority: log_authority.pubkey,
        })
    }
}
