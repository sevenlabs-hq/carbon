use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd31c3020d7a02317")]
pub struct UpdateRewardFunder {
    pub reward_index: u64,
    pub new_funder: solana_sdk::pubkey::Pubkey,
}

pub struct UpdateRewardFunderInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateRewardFunder {
    type ArrangedAccounts = UpdateRewardFunderInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let lb_pair = accounts.get(0)?;
        let admin = accounts.get(1)?;
        let event_authority = accounts.get(2)?;
        let program = accounts.get(3)?;

        Some(UpdateRewardFunderInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            admin: admin.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
