use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x72e44820c130a066")]
pub struct SetRewardEmissionsV2 {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
}

#[derive(Debug, PartialEq)]
pub struct SetRewardEmissionsV2InstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub reward_authority: solana_pubkey::Pubkey,
    pub reward_vault: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetRewardEmissionsV2 {
    type ArrangedAccounts = SetRewardEmissionsV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpool, reward_authority, reward_vault, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetRewardEmissionsV2InstructionAccounts {
            whirlpool: whirlpool.pubkey,
            reward_authority: reward_authority.pubkey,
            reward_vault: reward_vault.pubkey,
        })
    }
}
