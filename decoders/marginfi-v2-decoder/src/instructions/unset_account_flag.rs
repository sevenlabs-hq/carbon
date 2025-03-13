use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x385138555c31ff46")]
pub struct UnsetAccountFlag {
    pub flag: u64,
}

pub struct UnsetAccountFlagInstructionAccounts {
    pub marginfi_group: solana_sdk::pubkey::Pubkey,
    pub marginfi_account: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UnsetAccountFlag {
    type ArrangedAccounts = UnsetAccountFlagInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_group, marginfi_account, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UnsetAccountFlagInstructionAccounts {
            marginfi_group: marginfi_group.pubkey,
            marginfi_account: marginfi_account.pubkey,
            admin: admin.pubkey,
        })
    }
}
