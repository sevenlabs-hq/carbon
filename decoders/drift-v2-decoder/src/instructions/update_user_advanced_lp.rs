use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x42506bba1bf2425f")]
pub struct UpdateUserAdvancedLp {
    pub sub_account_id: u16,
    pub advanced_lp: bool,
}

pub struct UpdateUserAdvancedLpInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateUserAdvancedLp {
    type ArrangedAccounts = UpdateUserAdvancedLpInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateUserAdvancedLpInstructionAccounts {
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
