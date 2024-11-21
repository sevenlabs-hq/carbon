

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x8c90fd150a4af803")]
pub struct RefreshObligationFarmsForReserve{
    pub mode: u8,
}

pub struct RefreshObligationFarmsForReserveInstructionAccounts {
    pub crank: solana_sdk::pubkey::Pubkey,
    pub obligation: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub reserve_farm_state: solana_sdk::pubkey::Pubkey,
    pub obligation_farm_user_state: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub farms_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RefreshObligationFarmsForReserve {
    type ArrangedAccounts = RefreshObligationFarmsForReserveInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let crank = accounts.get(0)?;
        let obligation = accounts.get(1)?;
        let lending_market_authority = accounts.get(2)?;
        let reserve = accounts.get(3)?;
        let reserve_farm_state = accounts.get(4)?;
        let obligation_farm_user_state = accounts.get(5)?;
        let lending_market = accounts.get(6)?;
        let farms_program = accounts.get(7)?;
        let rent = accounts.get(8)?;
        let system_program = accounts.get(9)?;

        Some(RefreshObligationFarmsForReserveInstructionAccounts {
            crank: crank.pubkey,
            obligation: obligation.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            reserve: reserve.pubkey,
            reserve_farm_state: reserve_farm_state.pubkey,
            obligation_farm_user_state: obligation_farm_user_state.pubkey,
            lending_market: lending_market.pubkey,
            farms_program: farms_program.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}