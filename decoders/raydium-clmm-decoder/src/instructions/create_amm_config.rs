use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8934edd4d7756c68")]
pub struct CreateAmmConfig {
    pub index: u16,
    pub tick_spacing: u16,
    pub trade_fee_rate: u32,
    pub protocol_fee_rate: u32,
    pub fund_fee_rate: u32,
}

pub struct CreateAmmConfigInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateAmmConfig {
    type ArrangedAccounts = CreateAmmConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let amm_config = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(CreateAmmConfigInstructionAccounts {
            owner: owner.pubkey,
            amm_config: amm_config.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
