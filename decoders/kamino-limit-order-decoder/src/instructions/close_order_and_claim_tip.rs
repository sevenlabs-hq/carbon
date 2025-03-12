

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf41b0ce22df7e62b")]
pub struct CloseOrderAndClaimTip{
}

pub struct CloseOrderAndClaimTipInstructionAccounts {
    pub maker: solana_sdk::pubkey::Pubkey,
    pub order: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub pda_authority: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub maker_input_ata: solana_sdk::pubkey::Pubkey,
    pub input_vault: solana_sdk::pubkey::Pubkey,
    pub input_token_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseOrderAndClaimTip {
    type ArrangedAccounts = CloseOrderAndClaimTipInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            maker,
            order,
            global_config,
            pda_authority,
            input_mint,
            output_mint,
            maker_input_ata,
            input_vault,
            input_token_program,
            system_program,
            event_authority,
            program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(CloseOrderAndClaimTipInstructionAccounts {
            maker: maker.pubkey,
            order: order.pubkey,
            global_config: global_config.pubkey,
            pda_authority: pda_authority.pubkey,
            input_mint: input_mint.pubkey,
            output_mint: output_mint.pubkey,
            maker_input_ata: maker_input_ata.pubkey,
            input_vault: input_vault.pubkey,
            input_token_program: input_token_program.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}