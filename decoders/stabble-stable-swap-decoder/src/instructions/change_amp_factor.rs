use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x38eebd23c89d2a42")]
pub struct ChangeAmpFactor {
    pub new_amp_factor: u16,
    pub ramp_duration: u32,
}

pub struct ChangeAmpFactorInstructionAccounts {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub vault: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ChangeAmpFactor {
    type ArrangedAccounts = ChangeAmpFactorInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, vault, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ChangeAmpFactorInstructionAccounts {
            pool: pool.pubkey,
            vault: vault.pubkey,
            admin: admin.pubkey,
        })
    }
}
