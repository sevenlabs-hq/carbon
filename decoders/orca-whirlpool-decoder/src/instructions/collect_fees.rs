use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa498cf631eba13b6")]
pub struct CollectFees {}

#[derive(Debug, PartialEq)]
pub struct CollectFeesInstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub position_authority: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_token_account: solana_pubkey::Pubkey,
    pub token_owner_account_a: solana_pubkey::Pubkey,
    pub token_vault_a: solana_pubkey::Pubkey,
    pub token_owner_account_b: solana_pubkey::Pubkey,
    pub token_vault_b: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectFees {
    type ArrangedAccounts = CollectFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpool, position_authority, position, position_token_account, token_owner_account_a, token_vault_a, token_owner_account_b, token_vault_b, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectFeesInstructionAccounts {
            whirlpool: whirlpool.pubkey,
            position_authority: position_authority.pubkey,
            position: position.pubkey,
            position_token_account: position_token_account.pubkey,
            token_owner_account_a: token_owner_account_a.pubkey,
            token_vault_a: token_vault_a.pubkey,
            token_owner_account_b: token_owner_account_b.pubkey,
            token_vault_b: token_vault_b.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
