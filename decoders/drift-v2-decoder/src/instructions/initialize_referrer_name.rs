use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xeb7ee70a2aa41a3d")]
pub struct InitializeReferrerName {
    pub name: [u8; 32],
}

pub struct InitializeReferrerNameInstructionAccounts {
    pub referrer_name: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeReferrerName {
    type ArrangedAccounts = InitializeReferrerNameInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            referrer_name,
            user,
            user_stats,
            authority,
            payer,
            rent,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(InitializeReferrerNameInstructionAccounts {
            referrer_name: referrer_name.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
