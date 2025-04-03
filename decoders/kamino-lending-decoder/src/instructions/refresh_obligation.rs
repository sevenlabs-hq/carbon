use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x218493e497c04859")]
pub struct RefreshObligation {}

pub struct RefreshObligationInstructionAccounts {
    pub lending_market: solana_pubkey::Pubkey,
    pub obligation: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RefreshObligation {
    type ArrangedAccounts = RefreshObligationInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lending_market, obligation, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RefreshObligationInstructionAccounts {
            lending_market: lending_market.pubkey,
            obligation: obligation.pubkey,
        })
    }
}
