use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x02")]
pub struct InitializeMultisig {
    pub m: u8,
}

pub struct InitializeMultisigInstructionAccounts {
    pub multisig: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeMultisig {
    type ArrangedAccounts = InitializeMultisigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [multisig, rent, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeMultisigInstructionAccounts {
            multisig: multisig.pubkey,
            rent: rent.pubkey,
        })
    }
}
