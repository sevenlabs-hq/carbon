use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x283f7a9e90d85360")]
pub struct WithdrawTreasury {
    pub amount: u64,
}

pub struct WithdrawTreasuryInstructionAccounts {
    pub global_admin: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
    pub reward_treasury_vault: solana_sdk::pubkey::Pubkey,
    pub treasury_vault_authority: solana_sdk::pubkey::Pubkey,
    pub withdraw_destination_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawTreasury {
    type ArrangedAccounts = WithdrawTreasuryInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [global_admin, global_config, reward_mint, reward_treasury_vault, treasury_vault_authority, withdraw_destination_token_account, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawTreasuryInstructionAccounts {
            global_admin: global_admin.pubkey,
            global_config: global_config.pubkey,
            reward_mint: reward_mint.pubkey,
            reward_treasury_vault: reward_treasury_vault.pubkey,
            treasury_vault_authority: treasury_vault_authority.pubkey,
            withdraw_destination_token_account: withdraw_destination_token_account.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
