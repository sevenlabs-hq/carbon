use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x20")]
pub struct InitializeNonTransferableMint {}

pub struct InitializeNonTransferableMintInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeNonTransferableMint {
    type ArrangedAccounts = InitializeNonTransferableMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeNonTransferableMintInstructionAccounts { mint: mint.pubkey })
    }
}
