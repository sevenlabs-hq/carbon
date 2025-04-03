use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x856e4aaf709ff59f")]
pub struct InitializeOrder {
    pub params: InitializeOrderParams,
}

pub struct InitializeOrderInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub maker: solana_pubkey::Pubkey,
    pub order: solana_pubkey::Pubkey,
    pub input_mint_reserve: solana_pubkey::Pubkey,
    pub maker_input_mint_account: solana_pubkey::Pubkey,
    pub fee: solana_pubkey::Pubkey,
    pub referral: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub output_mint: solana_pubkey::Pubkey,
    pub input_token_program: solana_pubkey::Pubkey,
    pub output_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeOrder {
    type ArrangedAccounts = InitializeOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, maker, order, input_mint_reserve, maker_input_mint_account, fee, referral, input_mint, output_mint, input_token_program, output_token_program, system_program, associated_token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeOrderInstructionAccounts {
            payer: payer.pubkey,
            maker: maker.pubkey,
            order: order.pubkey,
            input_mint_reserve: input_mint_reserve.pubkey,
            maker_input_mint_account: maker_input_mint_account.pubkey,
            fee: fee.pubkey,
            referral: referral.pubkey,
            input_mint: input_mint.pubkey,
            output_mint: output_mint.pubkey,
            input_token_program: input_token_program.pubkey,
            output_token_program: output_token_program.pubkey,
            system_program: system_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
