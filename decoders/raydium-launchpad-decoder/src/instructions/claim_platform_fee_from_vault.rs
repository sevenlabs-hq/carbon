use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x75f1c6a8f8da501d")]
pub struct ClaimPlatformFeeFromVault {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct ClaimPlatformFeeFromVaultInstructionAccounts {
    pub platform_fee_wallet: solana_pubkey::Pubkey,
    pub fee_vault_authority: solana_pubkey::Pubkey,
    pub platform_config: solana_pubkey::Pubkey,
    pub platform_fee_vault: solana_pubkey::Pubkey,
    pub recipient_token_account: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimPlatformFeeFromVault {
    type ArrangedAccounts = ClaimPlatformFeeFromVaultInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let platform_fee_wallet = next_account(&mut iter)?;
        let fee_vault_authority = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;
        let platform_fee_vault = next_account(&mut iter)?;
        let recipient_token_account = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;

        Some(ClaimPlatformFeeFromVaultInstructionAccounts {
            platform_fee_wallet,
            fee_vault_authority,
            platform_config,
            platform_fee_vault,
            recipient_token_account,
            quote_mint,
            token_program,
            system_program,
            associated_token_program,
        })
    }
}
