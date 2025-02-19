use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x13")]
pub struct InitializeMultisig2 {
    pub m: u8,
}

pub struct InitializeMultisig2InstructionAccounts {
    pub multisig: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeMultisig2 {
    type ArrangedAccounts = InitializeMultisig2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [multisig, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeMultisig2InstructionAccounts {
            multisig: multisig.pubkey,
        })
    }
}
