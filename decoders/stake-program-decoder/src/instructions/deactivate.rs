use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2c7021ac711c8e0d")]
pub struct Deactivate {}

pub struct DeactivateInstructionAccounts {
    pub stake: solana_pubkey::Pubkey,
    pub clock: solana_pubkey::Pubkey,
    pub stake_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Deactivate {
    type ArrangedAccounts = DeactivateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [stake, clock, stake_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DeactivateInstructionAccounts {
            stake: stake.pubkey,
            clock: clock.pubkey,
            stake_authority: stake_authority.pubkey,
        })
    }
}
