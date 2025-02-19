use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x24")]
pub struct UpdateTransferHook {
    pub transfer_hook_discriminator: u8,
    pub program_id: Option<solana_sdk::pubkey::Pubkey>,
}

pub struct UpdateTransferHookInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateTransferHook {
    type ArrangedAccounts = UpdateTransferHookInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateTransferHookInstructionAccounts {
            mint: mint.pubkey,
            authority: authority.pubkey,
        })
    }
}
