use {
    super::super::types::*,
    carbon_core::{CarbonDeserialize, borsh},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfc681286a44e128c")]
pub struct FlashFillOrder {
    pub params: FlashFillOrderParams,
}

pub struct FlashFillOrderInstructionAccounts {
    pub taker: solana_pubkey::Pubkey,
    pub maker: solana_pubkey::Pubkey,
    pub order: solana_pubkey::Pubkey,
    pub input_mint_reserve: solana_pubkey::Pubkey,
    pub maker_output_mint_account: solana_pubkey::Pubkey,
    pub taker_output_mint_account: solana_pubkey::Pubkey,
    pub fee_account: solana_pubkey::Pubkey,
    pub input_token_program: solana_pubkey::Pubkey,
    pub output_mint: solana_pubkey::Pubkey,
    pub output_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FlashFillOrder {
    type ArrangedAccounts = FlashFillOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            taker,
            maker,
            order,
            input_mint_reserve,
            maker_output_mint_account,
            taker_output_mint_account,
            fee_account,
            input_token_program,
            output_mint,
            output_token_program,
            system_program,
            event_authority,
            program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(FlashFillOrderInstructionAccounts {
            taker: taker.pubkey,
            maker: maker.pubkey,
            order: order.pubkey,
            input_mint_reserve: input_mint_reserve.pubkey,
            maker_output_mint_account: maker_output_mint_account.pubkey,
            taker_output_mint_account: taker_output_mint_account.pubkey,
            fee_account: fee_account.pubkey,
            input_token_program: input_token_program.pubkey,
            output_mint: output_mint.pubkey,
            output_token_program: output_token_program.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
