use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x49905c774a6a10c8")]
pub struct UpdateReferralsAdmin {}

pub struct UpdateReferralsAdminInstructionAccounts {
    pub state: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub new_admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateReferralsAdmin {
    type ArrangedAccounts = UpdateReferralsAdminInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, new_admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateReferralsAdminInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
            new_admin: new_admin.pubkey,
        })
    }
}
