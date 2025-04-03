use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8c90fd150a4af803")]
pub struct RefreshObligationFarmsForReserve {
    pub mode: u8,
}

pub struct RefreshObligationFarmsForReserveInstructionAccounts {
    pub crank: solana_pubkey::Pubkey,
    pub obligation: solana_pubkey::Pubkey,
    pub lending_market_authority: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub reserve_farm_state: solana_pubkey::Pubkey,
    pub obligation_farm_user_state: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub farms_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RefreshObligationFarmsForReserve {
    type ArrangedAccounts = RefreshObligationFarmsForReserveInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [crank, obligation, lending_market_authority, reserve, reserve_farm_state, obligation_farm_user_state, lending_market, farms_program, rent, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(RefreshObligationFarmsForReserveInstructionAccounts {
            crank: crank.pubkey,
            obligation: obligation.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            reserve: reserve.pubkey,
            reserve_farm_state: reserve_farm_state.pubkey,
            obligation_farm_user_state: obligation_farm_user_state.pubkey,
            lending_market: lending_market.pubkey,
            farms_program: farms_program.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
