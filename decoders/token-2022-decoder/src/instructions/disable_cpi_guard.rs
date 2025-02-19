use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x22")]
pub struct DisableCpiGuard {
    pub cpi_guard_discriminator: u8,
}

pub struct DisableCpiGuardInstructionAccounts {
    pub token: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DisableCpiGuard {
    type ArrangedAccounts = DisableCpiGuardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DisableCpiGuardInstructionAccounts {
            token: token.pubkey,
            owner: owner.pubkey,
        })
    }
}
