use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa423b6130074f37f")]
pub struct MarkObligationForDeleveraging {
    pub autodeleverage_target_ltv_pct: u8,
}

pub struct MarkObligationForDeleveragingInstructionAccounts {
    pub risk_council: solana_pubkey::Pubkey,
    pub obligation: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MarkObligationForDeleveraging {
    type ArrangedAccounts = MarkObligationForDeleveragingInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [risk_council, obligation, lending_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(MarkObligationForDeleveragingInstructionAccounts {
            risk_council: risk_council.pubkey,
            obligation: obligation.pubkey,
            lending_market: lending_market.pubkey,
        })
    }
}
