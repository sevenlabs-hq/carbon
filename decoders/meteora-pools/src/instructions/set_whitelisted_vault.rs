use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0c945e2a373953f7")]
pub struct SetWhitelistedVault {
    pub whitelisted_vault: solana_pubkey::Pubkey,
}

pub struct SetWhitelistedVaultInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetWhitelistedVault {
    type ArrangedAccounts = SetWhitelistedVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, admin, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetWhitelistedVaultInstructionAccounts {
            pool: pool.pubkey,
            admin: admin.pubkey,
        })
    }
}
