use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7ff5139e52fa2112")]
pub struct PerpsV2Swap {}

pub struct PerpsV2SwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub funding_account: solana_sdk::pubkey::Pubkey,
    pub receiving_account: solana_sdk::pubkey::Pubkey,
    pub transfer_authority: solana_sdk::pubkey::Pubkey,
    pub perpetuals: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub receiving_custody: solana_sdk::pubkey::Pubkey,
    pub receiving_custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub receiving_custody_pythnet_price_account: solana_sdk::pubkey::Pubkey,
    pub receiving_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub dispensing_custody: solana_sdk::pubkey::Pubkey,
    pub dispensing_custody_doves_price_account: solana_sdk::pubkey::Pubkey,
    pub dispensing_custody_pythnet_price_account: solana_sdk::pubkey::Pubkey,
    pub dispensing_custody_token_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PerpsV2Swap {
    type ArrangedAccounts = PerpsV2SwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let funding_account = accounts.get(2)?;
        let receiving_account = accounts.get(3)?;
        let transfer_authority = accounts.get(4)?;
        let perpetuals = accounts.get(5)?;
        let pool = accounts.get(6)?;
        let receiving_custody = accounts.get(7)?;
        let receiving_custody_doves_price_account = accounts.get(8)?;
        let receiving_custody_pythnet_price_account = accounts.get(9)?;
        let receiving_custody_token_account = accounts.get(10)?;
        let dispensing_custody = accounts.get(11)?;
        let dispensing_custody_doves_price_account = accounts.get(12)?;
        let dispensing_custody_pythnet_price_account = accounts.get(13)?;
        let dispensing_custody_token_account = accounts.get(14)?;
        let token_program = accounts.get(15)?;
        let event_authority = accounts.get(16)?;
        let program = accounts.get(17)?;

        Some(PerpsV2SwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            owner: owner.pubkey,
            funding_account: funding_account.pubkey,
            receiving_account: receiving_account.pubkey,
            transfer_authority: transfer_authority.pubkey,
            perpetuals: perpetuals.pubkey,
            pool: pool.pubkey,
            receiving_custody: receiving_custody.pubkey,
            receiving_custody_doves_price_account: receiving_custody_doves_price_account.pubkey,
            receiving_custody_pythnet_price_account: receiving_custody_pythnet_price_account.pubkey,
            receiving_custody_token_account: receiving_custody_token_account.pubkey,
            dispensing_custody: dispensing_custody.pubkey,
            dispensing_custody_doves_price_account: dispensing_custody_doves_price_account.pubkey,
            dispensing_custody_pythnet_price_account: dispensing_custody_pythnet_price_account
                .pubkey,
            dispensing_custody_token_account: dispensing_custody_token_account.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
