use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x23765f4de72e8026")]
pub struct LogUserSwapBalances {}

pub struct LogUserSwapBalancesInstructionAccounts {
    pub maker: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub output_mint: solana_pubkey::Pubkey,
    pub input_ta: solana_pubkey::Pubkey,
    pub output_ta: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for LogUserSwapBalances {
    type ArrangedAccounts = LogUserSwapBalancesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            maker,
            input_mint,
            output_mint,
            input_ta,
            output_ta,
            event_authority,
            program,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(LogUserSwapBalancesInstructionAccounts {
            maker: maker.pubkey,
            input_mint: input_mint.pubkey,
            output_mint: output_mint.pubkey,
            input_ta: input_ta.pubkey,
            output_ta: output_ta.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
