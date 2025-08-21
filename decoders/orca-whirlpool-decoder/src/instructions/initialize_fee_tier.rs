use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb74a9ca070022a1e")]
pub struct InitializeFeeTier {
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct InitializeFeeTierInstructionAccounts {
    pub config: solana_pubkey::Pubkey,
    pub fee_tier: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeFeeTier {
    type ArrangedAccounts = InitializeFeeTierInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let config = next_account(&mut iter)?;
        let fee_tier = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let fee_authority = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(InitializeFeeTierInstructionAccounts {
            config,
            fee_tier,
            funder,
            fee_authority,
            system_program,
        })
    }
}
