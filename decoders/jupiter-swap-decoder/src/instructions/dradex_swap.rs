use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2292a02633553a97")]
pub struct DradexSwap {}

pub struct DradexSwapInstructionAccounts {
    pub swap_program: solana_sdk::pubkey::Pubkey,
    pub pair: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub event_queue: solana_sdk::pubkey::Pubkey,
    pub dex_user: solana_sdk::pubkey::Pubkey,
    pub market_user: solana_sdk::pubkey::Pubkey,
    pub bids: solana_sdk::pubkey::Pubkey,
    pub asks: solana_sdk::pubkey::Pubkey,
    pub t0_vault: solana_sdk::pubkey::Pubkey,
    pub t1_vault: solana_sdk::pubkey::Pubkey,
    pub t0_user: solana_sdk::pubkey::Pubkey,
    pub t1_user: solana_sdk::pubkey::Pubkey,
    pub master: solana_sdk::pubkey::Pubkey,
    pub signer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub logger: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DradexSwap {
    type ArrangedAccounts = DradexSwapInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let swap_program = accounts.get(0)?;
        let pair = accounts.get(1)?;
        let market = accounts.get(2)?;
        let event_queue = accounts.get(3)?;
        let dex_user = accounts.get(4)?;
        let market_user = accounts.get(5)?;
        let bids = accounts.get(6)?;
        let asks = accounts.get(7)?;
        let t0_vault = accounts.get(8)?;
        let t1_vault = accounts.get(9)?;
        let t0_user = accounts.get(10)?;
        let t1_user = accounts.get(11)?;
        let master = accounts.get(12)?;
        let signer = accounts.get(13)?;
        let system_program = accounts.get(14)?;
        let token_program = accounts.get(15)?;
        let logger = accounts.get(16)?;

        Some(DradexSwapInstructionAccounts {
            swap_program: swap_program.pubkey,
            pair: pair.pubkey,
            market: market.pubkey,
            event_queue: event_queue.pubkey,
            dex_user: dex_user.pubkey,
            market_user: market_user.pubkey,
            bids: bids.pubkey,
            asks: asks.pubkey,
            t0_vault: t0_vault.pubkey,
            t1_vault: t1_vault.pubkey,
            t0_user: t0_user.pubkey,
            t1_user: t1_user.pubkey,
            master: master.pubkey,
            signer: signer.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            logger: logger.pubkey,
        })
    }
}
