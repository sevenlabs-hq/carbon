use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x66063d1201daebea")]
pub struct Buy {
    pub data: TradeParams,
}

#[derive(Debug)]
pub struct BuyInstructionAccounts {
    pub sender: solana_sdk::pubkey::Pubkey,
    pub sender_token_account: solana_sdk::pubkey::Pubkey,
    pub curve_account: solana_sdk::pubkey::Pubkey,
    pub curve_token_account: solana_sdk::pubkey::Pubkey,
    pub dex_fee: solana_sdk::pubkey::Pubkey,
    pub helio_fee: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub config_account: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Buy {
    type ArrangedAccounts = BuyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [sender, sender_token_account, curve_account, curve_token_account, dex_fee, helio_fee, mint, config_account, token_program, associated_token_program, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(BuyInstructionAccounts {
            sender: sender.pubkey,
            sender_token_account: sender_token_account.pubkey,
            curve_account: curve_account.pubkey,
            curve_token_account: curve_token_account.pubkey,
            dex_fee: dex_fee.pubkey,
            helio_fee: helio_fee.pubkey,
            mint: mint.pubkey,
            config_account: config_account.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
