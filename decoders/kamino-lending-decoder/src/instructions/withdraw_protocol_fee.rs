

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x9ec99ebd215da267")]
pub struct WithdrawProtocolFee{
    pub amount: u64,
}

pub struct WithdrawProtocolFeeInstructionAccounts {
    pub lending_market_owner: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub fee_vault: solana_sdk::pubkey::Pubkey,
    pub lending_market_owner_ata: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawProtocolFee {
    type ArrangedAccounts = WithdrawProtocolFeeInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let lending_market_owner = accounts.get(0)?;
        let lending_market = accounts.get(1)?;
        let reserve = accounts.get(2)?;
        let reserve_liquidity_mint = accounts.get(3)?;
        let lending_market_authority = accounts.get(4)?;
        let fee_vault = accounts.get(5)?;
        let lending_market_owner_ata = accounts.get(6)?;
        let token_program = accounts.get(7)?;

        Some(WithdrawProtocolFeeInstructionAccounts {
            lending_market_owner: lending_market_owner.pubkey,
            lending_market: lending_market.pubkey,
            reserve: reserve.pubkey,
            reserve_liquidity_mint: reserve_liquidity_mint.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            fee_vault: fee_vault.pubkey,
            lending_market_owner_ata: lending_market_owner_ata.pubkey,
            token_program: token_program.pubkey,
        })
    }
}