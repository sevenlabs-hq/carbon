use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0df5b467feb67904")]
pub struct Invest {}

pub struct InvestInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub payer_token_account: solana_pubkey::Pubkey,
    pub vault_state: solana_pubkey::Pubkey,
    pub token_vault: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub base_vault_authority: solana_pubkey::Pubkey,
    pub ctoken_vault: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub lending_market_authority: solana_pubkey::Pubkey,
    pub reserve_liquidity_supply: solana_pubkey::Pubkey,
    pub reserve_collateral_mint: solana_pubkey::Pubkey,
    pub klend_program: solana_pubkey::Pubkey,
    pub reserve_collateral_token_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub instruction_sysvar_account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Invest {
    type ArrangedAccounts = InvestInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, payer_token_account, vault_state, token_vault, token_mint, base_vault_authority, ctoken_vault, reserve, lending_market, lending_market_authority, reserve_liquidity_supply, reserve_collateral_mint, klend_program, reserve_collateral_token_program, token_program, instruction_sysvar_account, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InvestInstructionAccounts {
            payer: payer.pubkey,
            payer_token_account: payer_token_account.pubkey,
            vault_state: vault_state.pubkey,
            token_vault: token_vault.pubkey,
            token_mint: token_mint.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            ctoken_vault: ctoken_vault.pubkey,
            reserve: reserve.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            reserve_liquidity_supply: reserve_liquidity_supply.pubkey,
            reserve_collateral_mint: reserve_collateral_mint.pubkey,
            klend_program: klend_program.pubkey,
            reserve_collateral_token_program: reserve_collateral_token_program.pubkey,
            token_program: token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}
