use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x3ec6d6c1d59f6cd2")]
pub struct Claim {
    pub id: u8,
}

pub struct ClaimInstructionAccounts {
    pub wallet: solana_sdk::pubkey::Pubkey,
    pub program_authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Claim {
    type ArrangedAccounts = ClaimInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let wallet = accounts.get(0)?;
        let program_authority = accounts.get(1)?;
        let system_program = accounts.get(2)?;

        Some(ClaimInstructionAccounts {
            wallet: wallet.pubkey,
            program_authority: program_authority.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
