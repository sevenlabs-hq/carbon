use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc9cff3724b6f2fbd")]
pub struct CreateConfig {
    pub curve_type: u8,
    pub index: u16,
    pub migrate_fee: u64,
    pub trade_fee_rate: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateConfigInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub quote_token_mint: solana_pubkey::Pubkey,
    pub protocol_fee_owner: solana_pubkey::Pubkey,
    pub migrate_fee_owner: solana_pubkey::Pubkey,
    pub migrate_to_amm_wallet: solana_pubkey::Pubkey,
    pub migrate_to_cpswap_wallet: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateConfig {
    type ArrangedAccounts = CreateConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let owner = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let quote_token_mint = next_account(&mut iter)?;
        let protocol_fee_owner = next_account(&mut iter)?;
        let migrate_fee_owner = next_account(&mut iter)?;
        let migrate_to_amm_wallet = next_account(&mut iter)?;
        let migrate_to_cpswap_wallet = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateConfigInstructionAccounts {
            owner,
            global_config,
            quote_token_mint,
            protocol_fee_owner,
            migrate_fee_owner,
            migrate_to_amm_wallet,
            migrate_to_cpswap_wallet,
            system_program,
        })
    }
}
