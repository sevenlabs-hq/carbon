use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x02f1c3ace318fe9e")]
pub struct ForceDeleteUser {}

pub struct ForceDeleteUserInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub keeper: solana_pubkey::Pubkey,
    pub drift_signer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ForceDeleteUser {
    type ArrangedAccounts = ForceDeleteUserInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            user,
            user_stats,
            state,
            authority,
            keeper,
            drift_signer,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(ForceDeleteUserInstructionAccounts {
            user: user.pubkey,
            user_stats: user_stats.pubkey,
            state: state.pubkey,
            authority: authority.pubkey,
            keeper: keeper.pubkey,
            drift_signer: drift_signer.pubkey,
        })
    }
}
