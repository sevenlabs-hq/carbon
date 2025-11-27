use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbee4fd10c994a1f0")]
pub struct CloseSpreadAccount {}

pub struct CloseSpreadAccountInstructionAccounts {
    pub spread_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseSpreadAccount {
    type ArrangedAccounts = CloseSpreadAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [spread_account, authority, zeta_group, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CloseSpreadAccountInstructionAccounts {
            spread_account: spread_account.pubkey,
            authority: authority.pubkey,
            zeta_group: zeta_group.pubkey,
        })
    }
}
