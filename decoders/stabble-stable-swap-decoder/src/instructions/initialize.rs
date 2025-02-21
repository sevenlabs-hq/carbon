use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xafaf6d1f0d989bed")]
pub struct Initialize {
    pub amp_factor: u16,
    pub swap_fee: u64,
    pub max_caps: Vec<u64>,
}

pub struct InitializeInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub pool_authority: solana_sdk::pubkey::Pubkey,
    pub withdraw_authority: solana_sdk::pubkey::Pubkey,
    pub vault: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, mint, pool, pool_authority, withdraw_authority, vault, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeInstructionAccounts {
            owner: owner.pubkey,
            mint: mint.pubkey,
            pool: pool.pubkey,
            pool_authority: pool_authority.pubkey,
            withdraw_authority: withdraw_authority.pubkey,
            vault: vault.pubkey,
        })
    }
}
