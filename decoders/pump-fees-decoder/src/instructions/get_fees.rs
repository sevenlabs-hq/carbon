use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe7257e55cf5b3f34")]
pub struct GetFees {
    pub is_pump_pool: bool,
    pub market_cap_lamports: u128,
    pub trade_size_lamports: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct GetFeesInstructionAccounts {
    pub fee_config: solana_pubkey::Pubkey,
    pub config_program_id: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetFees {
    type ArrangedAccounts = GetFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let fee_config = next_account(&mut iter)?;
        let config_program_id = next_account(&mut iter)?;

        Some(GetFeesInstructionAccounts {
            fee_config,
            config_program_id,
        })
    }
}
