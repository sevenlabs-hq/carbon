use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x28")]
pub struct InitializeGroupPointer {
    pub group_pointer_discriminator: u8,
    pub authority: Option<solana_sdk::pubkey::Pubkey>,
    pub group_address: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct InitializeGroupPointerInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeGroupPointer {
    type ArrangedAccounts = InitializeGroupPointerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeGroupPointerInstructionAccounts { mint: mint.pubkey })
    }
}
