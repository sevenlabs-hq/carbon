use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfc681286a44e128c")]
pub struct FlashFillOrder {
    pub max_taking_amount: u64,
}

pub struct FlashFillOrderInstructionAccounts {
    pub order: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub maker: solana_pubkey::Pubkey,
    pub taker: solana_pubkey::Pubkey,
    pub maker_output_account: solana_pubkey::Pubkey,
    pub taker_input_account: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
    pub program_fee_account: solana_pubkey::Pubkey,
    pub referral: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub input_mint_token_program: solana_pubkey::Pubkey,
    pub output_mint: solana_pubkey::Pubkey,
    pub output_mint_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FlashFillOrder {
    type ArrangedAccounts = FlashFillOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            order,
            reserve,
            maker,
            taker,
            maker_output_account,
            taker_input_account,
            fee_authority,
            program_fee_account,
            referral,
            input_mint,
            input_mint_token_program,
            output_mint,
            output_mint_token_program,
            system_program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(FlashFillOrderInstructionAccounts {
            order: order.pubkey,
            reserve: reserve.pubkey,
            maker: maker.pubkey,
            taker: taker.pubkey,
            maker_output_account: maker_output_account.pubkey,
            taker_input_account: taker_input_account.pubkey,
            fee_authority: fee_authority.pubkey,
            program_fee_account: program_fee_account.pubkey,
            referral: referral.pubkey,
            input_mint: input_mint.pubkey,
            input_mint_token_program: input_mint_token_program.pubkey,
            output_mint: output_mint.pubkey,
            output_mint_token_program: output_mint_token_program.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
