
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x8250269950d4b6fd")]
pub struct IdlMissingTypes{
    pub reserve_farm_kind: ReserveFarmKind,
    pub asset_tier: AssetTier,
    pub fee_calculation: FeeCalculation,
    pub reserve_status: ReserveStatus,
    pub update_config_mode: UpdateConfigMode,
    pub update_lending_market_config_value: UpdateLendingMarketConfigValue,
    pub update_lending_market_config_mode: UpdateLendingMarketMode,
}

pub struct IdlMissingTypesInstructionAccounts {
    pub lending_market_owner: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for IdlMissingTypes {
    type ArrangedAccounts = IdlMissingTypesInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let lending_market_owner = accounts.get(0)?;
        let lending_market = accounts.get(1)?;
        let reserve = accounts.get(2)?;

        Some(IdlMissingTypesInstructionAccounts {
            lending_market_owner: lending_market_owner.pubkey,
            lending_market: lending_market.pubkey,
            reserve: reserve.pubkey,
        })
    }
}