use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2caabd28807bfcc9")]
pub struct SetLockup {
    pub unix_timestamp: Option<i64>,
    pub epoch: Option<u64>,
    pub custodian: Option<solana_pubkey::Pubkey>,
}

pub struct SetLockupInstructionAccounts {
    pub stake: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetLockup {
    type ArrangedAccounts = SetLockupInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [stake, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetLockupInstructionAccounts {
            stake: stake.pubkey,
            authority: authority.pubkey,
        })
    }
}
