use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa334c8e78c0345ba")]
pub struct Transfer {}

pub struct TransferInstructionAccounts {
    pub keeper: solana_pubkey::Pubkey,
    pub dca: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub output_mint: solana_pubkey::Pubkey,
    pub dca_out_ata: solana_pubkey::Pubkey,
    pub user_out_ata: solana_pubkey::Pubkey,
    pub intermediate_account: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Transfer {
    type ArrangedAccounts = TransferInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, dca, user, output_mint, dca_out_ata, user_out_ata, intermediate_account, system_program, token_program, associated_token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(TransferInstructionAccounts {
            keeper: keeper.pubkey,
            dca: dca.pubkey,
            user: user.pubkey,
            output_mint: output_mint.pubkey,
            dca_out_ata: dca_out_ata.pubkey,
            user_out_ata: user_out_ata.pubkey,
            intermediate_account: intermediate_account.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
