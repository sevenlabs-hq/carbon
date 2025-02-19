use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf02f99440dbee12a")]
pub struct PreFlashFillOrder {
    pub params: PreFlashFillOrderParams,
}

pub struct PreFlashFillOrderInstructionAccounts {
    pub taker: solana_sdk::pubkey::Pubkey,
    pub order: solana_sdk::pubkey::Pubkey,
    pub input_mint_reserve: solana_sdk::pubkey::Pubkey,
    pub taker_input_mint_account: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub input_token_program: solana_sdk::pubkey::Pubkey,
    pub instruction: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PreFlashFillOrder {
    type ArrangedAccounts = PreFlashFillOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [taker, order, input_mint_reserve, taker_input_mint_account, input_mint, input_token_program, instruction, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(PreFlashFillOrderInstructionAccounts {
            taker: taker.pubkey,
            order: order.pubkey,
            input_mint_reserve: input_mint_reserve.pubkey,
            taker_input_mint_account: taker_input_mint_account.pubkey,
            input_mint: input_mint.pubkey,
            input_token_program: input_token_program.pubkey,
            instruction: instruction.pubkey,
        })
    }
}
