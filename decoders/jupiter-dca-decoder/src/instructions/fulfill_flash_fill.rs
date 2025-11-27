use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x7340e24e21d369a2")]
pub struct FulfillFlashFill {
    pub repay_amount: u64,
}

#[derive(Debug, PartialEq)]
pub struct FulfillFlashFillInstructionAccounts {
    pub keeper: solana_pubkey::Pubkey,
    pub dca: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub output_mint: solana_pubkey::Pubkey,
    pub keeper_in_ata: solana_pubkey::Pubkey,
    pub in_ata: solana_pubkey::Pubkey,
    pub out_ata: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
    pub fee_ata: solana_pubkey::Pubkey,
    pub instructions_sysvar: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FulfillFlashFill {
    type ArrangedAccounts = FulfillFlashFillInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            keeper,
            dca,
            input_mint,
            output_mint,
            keeper_in_ata,
            in_ata,
            out_ata,
            fee_authority,
            fee_ata,
            instructions_sysvar,
            system_program,
            token_program,
            associated_token_program,
            event_authority,
            program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(FulfillFlashFillInstructionAccounts {
            keeper: keeper.pubkey,
            dca: dca.pubkey,
            input_mint: input_mint.pubkey,
            output_mint: output_mint.pubkey,
            keeper_in_ata: keeper_in_ata.pubkey,
            in_ata: in_ata.pubkey,
            out_ata: out_ata.pubkey,
            fee_authority: fee_authority.pubkey,
            fee_ata: fee_ata.pubkey,
            instructions_sysvar: instructions_sysvar.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
