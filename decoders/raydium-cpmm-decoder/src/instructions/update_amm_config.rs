use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x313cae889a1c74c8")]
pub struct UpdateAmmConfig {
    pub param: u8,
    pub value: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateAmmConfigInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateAmmConfig {
    type ArrangedAccounts = UpdateAmmConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let owner = next_account(&mut iter)?;
        let amm_config = next_account(&mut iter)?;

        Some(UpdateAmmConfigInstructionAccounts { owner, amm_config })
    }
}
