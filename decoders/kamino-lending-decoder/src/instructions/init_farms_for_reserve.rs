use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xda063ee90121e852")]
pub struct InitFarmsForReserve {
    pub mode: u8,
}

pub struct InitFarmsForReserveInstructionAccounts {
    pub lending_market_owner: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub lending_market_authority: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub farms_program: solana_pubkey::Pubkey,
    pub farms_global_config: solana_pubkey::Pubkey,
    pub farm_state: solana_pubkey::Pubkey,
    pub farms_vault_authority: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitFarmsForReserve {
    type ArrangedAccounts = InitFarmsForReserveInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lending_market_owner, lending_market, lending_market_authority, reserve, farms_program, farms_global_config, farm_state, farms_vault_authority, rent, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitFarmsForReserveInstructionAccounts {
            lending_market_owner: lending_market_owner.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            reserve: reserve.pubkey,
            farms_program: farms_program.pubkey,
            farms_global_config: farms_global_config.pubkey,
            farm_state: farm_state.pubkey,
            farms_vault_authority: farms_vault_authority.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
