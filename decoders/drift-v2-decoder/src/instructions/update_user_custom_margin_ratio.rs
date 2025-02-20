use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x15dd8cbb20810b7b")]
pub struct UpdateUserCustomMarginRatio {
    pub sub_account_id: u16,
    pub margin_ratio: u32,
}

pub struct UpdateUserCustomMarginRatioInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateUserCustomMarginRatio {
    type ArrangedAccounts = UpdateUserCustomMarginRatioInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateUserCustomMarginRatioInstructionAccounts {
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
