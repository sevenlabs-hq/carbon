use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8af547e19904032b")]
pub struct InitReserve {}

pub struct InitReserveInstructionAccounts {
    pub lending_market_owner: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub lending_market_authority: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_pubkey::Pubkey,
    pub reserve_liquidity_supply: solana_pubkey::Pubkey,
    pub fee_receiver: solana_pubkey::Pubkey,
    pub reserve_collateral_mint: solana_pubkey::Pubkey,
    pub reserve_collateral_supply: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub liquidity_token_program: solana_pubkey::Pubkey,
    pub collateral_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitReserve {
    type ArrangedAccounts = InitReserveInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lending_market_owner, lending_market, lending_market_authority, reserve, reserve_liquidity_mint, reserve_liquidity_supply, fee_receiver, reserve_collateral_mint, reserve_collateral_supply, rent, liquidity_token_program, collateral_token_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitReserveInstructionAccounts {
            lending_market_owner: lending_market_owner.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            reserve: reserve.pubkey,
            reserve_liquidity_mint: reserve_liquidity_mint.pubkey,
            reserve_liquidity_supply: reserve_liquidity_supply.pubkey,
            fee_receiver: fee_receiver.pubkey,
            reserve_collateral_mint: reserve_collateral_mint.pubkey,
            reserve_collateral_supply: reserve_collateral_supply.pubkey,
            rent: rent.pubkey,
            liquidity_token_program: liquidity_token_program.pubkey,
            collateral_token_program: collateral_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
