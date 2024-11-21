use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa39fa319f3a16c4a")]
pub struct HeliumTreasuryManagementRedeemV0 {}

pub struct HeliumTreasuryManagementRedeemV0InstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub treasury_management: solana_sdk::pubkey::Pubkey,
    pub treasury_mint: solana_sdk::pubkey::Pubkey,
    pub supply_mint: solana_sdk::pubkey::Pubkey,
    pub treasury: solana_sdk::pubkey::Pubkey,
    pub circuit_breaker: solana_sdk::pubkey::Pubkey,
    pub from: solana_sdk::pubkey::Pubkey,
    pub to: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub circuit_breaker_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for HeliumTreasuryManagementRedeemV0 {
    type ArrangedAccounts = HeliumTreasuryManagementRedeemV0InstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let treasury_management = accounts.get(1)?;
        let treasury_mint = accounts.get(2)?;
        let supply_mint = accounts.get(3)?;
        let treasury = accounts.get(4)?;
        let circuit_breaker = accounts.get(5)?;
        let from = accounts.get(6)?;
        let to = accounts.get(7)?;
        let owner = accounts.get(8)?;
        let circuit_breaker_program = accounts.get(9)?;
        let token_program = accounts.get(10)?;

        Some(HeliumTreasuryManagementRedeemV0InstructionAccounts {
            swap_program: swap_program.pubkey,
            treasury_management: treasury_management.pubkey,
            treasury_mint: treasury_mint.pubkey,
            supply_mint: supply_mint.pubkey,
            treasury: treasury.pubkey,
            circuit_breaker: circuit_breaker.pubkey,
            from: from.pubkey,
            to: to.pubkey,
            owner: owner.pubkey,
            circuit_breaker_program: circuit_breaker_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
