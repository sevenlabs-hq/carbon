use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x856e4aaf709ff59f")]
pub struct InitializeOrder {
    pub making_amount: u64,
    pub taking_amount: u64,
    pub expired_at: Option<i64>,
}

pub struct InitializeOrderInstructionAccounts {
    pub base: solana_pubkey::Pubkey,
    pub maker: solana_pubkey::Pubkey,
    pub order: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub maker_input_account: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub maker_output_account: solana_pubkey::Pubkey,
    pub referral: solana_pubkey::Pubkey,
    pub output_mint: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeOrder {
    type ArrangedAccounts = InitializeOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [base, maker, order, reserve, maker_input_account, input_mint, maker_output_account, referral, output_mint, system_program, token_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeOrderInstructionAccounts {
            base: base.pubkey,
            maker: maker.pubkey,
            order: order.pubkey,
            reserve: reserve.pubkey,
            maker_input_account: maker_input_account.pubkey,
            input_mint: input_mint.pubkey,
            maker_output_account: maker_output_account.pubkey,
            referral: referral.pubkey,
            output_mint: output_mint.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
