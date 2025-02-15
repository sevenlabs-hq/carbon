use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x313cae889a1c74c8")]
pub struct UpdateAmmConfig {
    pub param: u8,
    pub value: u32,
}

pub struct UpdateAmmConfigInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub amm_config: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateAmmConfig {
    type ArrangedAccounts = UpdateAmmConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let amm_config = accounts.get(1)?;

        Some(UpdateAmmConfigInstructionAccounts {
            owner: owner.pubkey,
            amm_config: amm_config.pubkey,
        })
    }
}
