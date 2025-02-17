use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5f81edf00831df84")]
pub struct CancelOrder {}

pub struct CancelOrderInstructionAccounts {
    pub order: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub maker_input_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelOrder {
    type ArrangedAccounts = CancelOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [order, reserve, maker, maker_input_account, system_program, token_program, input_mint] =
            accounts
        else {
            return None;
        };

        Some(CancelOrderInstructionAccounts {
            order: order.pubkey,
            reserve: reserve.pubkey,
            maker: maker.pubkey,
            maker_input_account: maker_input_account.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            input_mint: input_mint.pubkey,
        })
    }
}
