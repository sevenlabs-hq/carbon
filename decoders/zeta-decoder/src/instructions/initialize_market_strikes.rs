use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbd2eff217e852bab")]
pub struct InitializeMarketStrikes {}

pub struct InitializeMarketStrikesInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub oracle_backup_feed: solana_pubkey::Pubkey,
    pub oracle_backup_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeMarketStrikes {
    type ArrangedAccounts = InitializeMarketStrikesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, zeta_group, oracle, oracle_backup_feed, oracle_backup_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeMarketStrikesInstructionAccounts {
            state: state.pubkey,
            zeta_group: zeta_group.pubkey,
            oracle: oracle.pubkey,
            oracle_backup_feed: oracle_backup_feed.pubkey,
            oracle_backup_program: oracle_backup_program.pubkey,
        })
    }
}
