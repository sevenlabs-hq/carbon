use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x697cc96a9902089c")]
pub struct LendingAccountEndFlashloan {}

pub struct LendingAccountEndFlashloanInstructionAccounts {
    pub marginfi_account: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LendingAccountEndFlashloan {
    type ArrangedAccounts = LendingAccountEndFlashloanInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [marginfi_account, signer, _remaining @ ..] = accounts else {
            return None;
        };

        Some(LendingAccountEndFlashloanInstructionAccounts {
            marginfi_account: marginfi_account.pubkey,
            signer: signer.pubkey,
        })
    }
}
