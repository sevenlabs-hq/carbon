use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd87840eb9b13e563")]
pub struct CancelExpiredOrder {}

pub struct CancelExpiredOrderInstructionAccounts {
    pub order: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub maker_input_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelExpiredOrder {
    type ArrangedAccounts = CancelExpiredOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [order, reserve, maker, maker_input_account, system_program, token_program, input_mint] =
            accounts
        else {
            return None;
        };

        Some(CancelExpiredOrderInstructionAccounts {
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
