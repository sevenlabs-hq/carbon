use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x83c2c88caff4d9b7")]
pub struct WithdrawPendingFees {}

pub struct WithdrawPendingFeesInstructionAccounts {
    pub admin_authority: solana_pubkey::Pubkey,
    pub vault_state: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub token_vault: solana_pubkey::Pubkey,
    pub ctoken_vault: solana_pubkey::Pubkey,
    pub base_vault_authority: solana_pubkey::Pubkey,
    pub token_ata: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub lending_market_authority: solana_pubkey::Pubkey,
    pub reserve_liquidity_supply: solana_pubkey::Pubkey,
    pub reserve_collateral_mint: solana_pubkey::Pubkey,
    pub klend_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub reserve_collateral_token_program: solana_pubkey::Pubkey,
    pub instruction_sysvar_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawPendingFees {
    type ArrangedAccounts = WithdrawPendingFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            admin_authority,
            vault_state,
            reserve,
            token_vault,
            ctoken_vault,
            base_vault_authority,
            token_ata,
            token_mint,
            lending_market,
            lending_market_authority,
            reserve_liquidity_supply,
            reserve_collateral_mint,
            klend_program,
            token_program,
            reserve_collateral_token_program,
            instruction_sysvar_account,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(WithdrawPendingFeesInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            vault_state: vault_state.pubkey,
            reserve: reserve.pubkey,
            token_vault: token_vault.pubkey,
            ctoken_vault: ctoken_vault.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            token_ata: token_ata.pubkey,
            token_mint: token_mint.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            reserve_liquidity_supply: reserve_liquidity_supply.pubkey,
            reserve_collateral_mint: reserve_collateral_mint.pubkey,
            klend_program: klend_program.pubkey,
            token_program: token_program.pubkey,
            reserve_collateral_token_program: reserve_collateral_token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}
