use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x169e0cb7765e9cff")]
pub struct SetLockupChecked {
    pub unix_timestamp: Option<i64>,
    pub epoch: Option<u64>,
}

pub struct SetLockupCheckedInstructionAccounts {
    pub stake: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetLockupChecked {
    type ArrangedAccounts = SetLockupCheckedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [stake, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetLockupCheckedInstructionAccounts {
            stake: stake.pubkey,
            authority: authority.pubkey,
        })
    }
}
