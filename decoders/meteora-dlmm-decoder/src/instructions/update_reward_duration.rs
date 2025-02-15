use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8aaec4a9d5ebfe6b")]
pub struct UpdateRewardDuration {
    pub reward_index: u64,
    pub new_duration: u64,
}

pub struct UpdateRewardDurationInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub bin_array: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateRewardDuration {
    type ArrangedAccounts = UpdateRewardDurationInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, admin, bin_array, event_authority, program] = accounts else {
            return None;
        };

        Some(UpdateRewardDurationInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            admin: admin.pubkey,
            bin_array: bin_array.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
