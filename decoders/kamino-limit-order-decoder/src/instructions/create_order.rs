use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8d3625cfedd2fad7")]
pub struct CreateOrder {
    pub input_amount: u64,
    pub output_amount: u64,
    pub order_type: u8,
}

pub struct CreateOrderInstructionAccounts {
    pub maker: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub pda_authority: solana_pubkey::Pubkey,
    pub order: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub output_mint: solana_pubkey::Pubkey,
    pub maker_ata: solana_pubkey::Pubkey,
    pub input_vault: solana_pubkey::Pubkey,
    pub input_token_program: solana_pubkey::Pubkey,
    pub output_token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOrder {
    type ArrangedAccounts = CreateOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [maker, global_config, pda_authority, order, input_mint, output_mint, maker_ata, input_vault, input_token_program, output_token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateOrderInstructionAccounts {
            maker: maker.pubkey,
            global_config: global_config.pubkey,
            pda_authority: pda_authority.pubkey,
            order: order.pubkey,
            input_mint: input_mint.pubkey,
            output_mint: output_mint.pubkey,
            maker_ata: maker_ata.pubkey,
            input_vault: input_vault.pubkey,
            input_token_program: input_token_program.pubkey,
            output_token_program: output_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
