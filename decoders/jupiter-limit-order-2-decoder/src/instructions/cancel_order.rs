use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5f81edf00831df84")]
pub struct CancelOrder {}

pub struct CancelOrderInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub maker: solana_pubkey::Pubkey,
    pub order: solana_pubkey::Pubkey,
    pub input_mint_reserve: solana_pubkey::Pubkey,
    pub maker_input_mint_account: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub input_token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelOrder {
    type ArrangedAccounts = CancelOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            signer,
            maker,
            order,
            input_mint_reserve,
            maker_input_mint_account,
            input_mint,
            input_token_program,
            event_authority,
            program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(CancelOrderInstructionAccounts {
            signer: signer.pubkey,
            maker: maker.pubkey,
            order: order.pubkey,
            input_mint_reserve: input_mint_reserve.pubkey,
            maker_input_mint_account: maker_input_mint_account.pubkey,
            input_mint: input_mint.pubkey,
            input_token_program: input_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
