use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2477fb8122f00793")]
pub struct RequestElevationGroup {
    pub elevation_group: u8,
}

pub struct RequestElevationGroupInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub obligation: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RequestElevationGroup {
    type ArrangedAccounts = RequestElevationGroupInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, obligation, lending_market, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RequestElevationGroupInstructionAccounts {
            owner: owner.pubkey,
            obligation: obligation.pubkey,
            lending_market: lending_market.pubkey,
        })
    }
}
