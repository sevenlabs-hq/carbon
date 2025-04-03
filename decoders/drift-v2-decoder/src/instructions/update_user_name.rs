use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8719b938a5352288")]
pub struct UpdateUserName {
    pub sub_account_id: u16,
    pub name: [u8; 32],
}

pub struct UpdateUserNameInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateUserName {
    type ArrangedAccounts = UpdateUserNameInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateUserNameInstructionAccounts {
            user: user.pubkey,
            authority: authority.pubkey,
        })
    }
}
