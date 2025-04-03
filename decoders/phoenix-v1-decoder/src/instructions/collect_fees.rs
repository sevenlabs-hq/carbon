use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6c")]
pub struct CollectFees {}

pub struct CollectFeesInstructionAccounts {
    pub phoenix_program: solana_pubkey::Pubkey,
    pub log_authority: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub sweeper: solana_pubkey::Pubkey,
    pub fee_recipient: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectFees {
    type ArrangedAccounts = CollectFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [phoenix_program, log_authority, market, sweeper, fee_recipient, quote_vault, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectFeesInstructionAccounts {
            phoenix_program: phoenix_program.pubkey,
            log_authority: log_authority.pubkey,
            market: market.pubkey,
            sweeper: sweeper.pubkey,
            fee_recipient: fee_recipient.pubkey,
            quote_vault: quote_vault.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
