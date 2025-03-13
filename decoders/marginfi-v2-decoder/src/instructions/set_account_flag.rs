use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x38ee12cfc1528aae")]
pub struct SetAccountFlag {
    pub flag: u64,
}

pub struct SetAccountFlagInstructionAccounts {
    pub marginfi_group: solana_sdk::pubkey::Pubkey,
    pub marginfi_account: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetAccountFlag {
    type ArrangedAccounts = SetAccountFlagInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_group, marginfi_account, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetAccountFlagInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            marginfi_account: marginfi_account.pubkey,
            admin: admin.pubkey,
        })
    }
}
