

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x883f0fbad398a8a4")]
pub struct InitObligationFarmsForReserve{
    pub mode: u8,
}

pub struct InitObligationFarmsForReserveInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub obligation: solana_sdk::pubkey::Pubkey,
    pub lending_market_authority: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub reserve_farm_state: solana_sdk::pubkey::Pubkey,
    pub obligation_farm: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub farms_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitObligationFarmsForReserve {
    type ArrangedAccounts = InitObligationFarmsForReserveInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let obligation = accounts.get(2)?;
        let lending_market_authority = accounts.get(3)?;
        let reserve = accounts.get(4)?;
        let reserve_farm_state = accounts.get(5)?;
        let obligation_farm = accounts.get(6)?;
        let lending_market = accounts.get(7)?;
        let farms_program = accounts.get(8)?;
        let rent = accounts.get(9)?;
        let system_program = accounts.get(10)?;

        Some(InitObligationFarmsForReserveInstructionAccounts {
            payer: payer.pubkey,
            owner: owner.pubkey,
            obligation: obligation.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            reserve: reserve.pubkey,
            reserve_farm_state: reserve_farm_state.pubkey,
            obligation_farm: obligation_farm.pubkey,
            lending_market: lending_market.pubkey,
            farms_program: farms_program.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}