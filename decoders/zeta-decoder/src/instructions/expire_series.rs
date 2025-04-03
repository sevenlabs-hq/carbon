use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2da269622c15ab7f")]
pub struct ExpireSeries {
    pub settlement_nonce: u8,
}

pub struct ExpireSeriesInstructionAccounts {}

impl carbon_core::deserialize::ArrangeAccounts for ExpireSeries {
    type ArrangedAccounts = ExpireSeriesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let _remaining = accounts else {
            return None;
        };

        Some(ExpireSeriesInstructionAccounts {})
    }
}
