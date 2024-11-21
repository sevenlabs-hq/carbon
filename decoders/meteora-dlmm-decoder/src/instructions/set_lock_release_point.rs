use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9447381437da9885")]
pub struct SetLockReleasePoint {
    pub new_lock_release_point: u64,
}

pub struct SetLockReleasePointInstructionAccounts {
    pub position: solana_sdk::pubkey::Pubkey,
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub sender: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetLockReleasePoint {
    type ArrangedAccounts = SetLockReleasePointInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let position = accounts.get(0)?;
        let lb_pair = accounts.get(1)?;
        let sender = accounts.get(2)?;
        let event_authority = accounts.get(3)?;
        let program = accounts.get(4)?;

        Some(SetLockReleasePointInstructionAccounts {
            position: position.pubkey,
            lb_pair: lb_pair.pubkey,
            sender: sender.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
