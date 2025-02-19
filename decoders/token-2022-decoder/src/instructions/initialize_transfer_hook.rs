use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x24")]
pub struct InitializeTransferHook {
    pub transfer_hook_discriminator: u8,
    pub authority: Option<solana_sdk::pubkey::Pubkey>,
    pub program_id: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct InitializeTransferHookInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeTransferHook {
    type ArrangedAccounts = InitializeTransferHookInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeTransferHookInstructionAccounts { mint: mint.pubkey })
    }
}
