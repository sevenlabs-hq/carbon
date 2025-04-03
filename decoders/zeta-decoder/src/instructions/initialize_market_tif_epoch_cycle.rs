use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc78fad93cacc40cc")]
pub struct InitializeMarketTifEpochCycle {
    pub epoch_length: u16,
}

pub struct InitializeMarketTifEpochCycleInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeMarketTifEpochCycle {
    type ArrangedAccounts = InitializeMarketTifEpochCycleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, admin, market, serum_authority, dex_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeMarketTifEpochCycleInstructionAccounts {
            state: state.pubkey,
            admin: admin.pubkey,
            market: market.pubkey,
            serum_authority: serum_authority.pubkey,
            dex_program: dex_program.pubkey,
        })
    }
}
