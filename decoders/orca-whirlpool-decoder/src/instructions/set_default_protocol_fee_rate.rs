use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6bcdf9e297235600")]
pub struct SetDefaultProtocolFeeRate {
    pub default_protocol_fee_rate: u16,
}

pub struct SetDefaultProtocolFeeRateInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SetDefaultProtocolFeeRate {
    type ArrangedAccounts = SetDefaultProtocolFeeRateInstructionAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let whirlpools_config = accounts.get(0)?;
        let fee_authority = accounts.get(1)?;

        Some(SetDefaultProtocolFeeRateInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            fee_authority: fee_authority.pubkey,
        })
    }
}
