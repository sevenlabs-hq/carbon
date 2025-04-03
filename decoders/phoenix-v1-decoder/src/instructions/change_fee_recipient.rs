use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6d")]
pub struct ChangeFeeRecipient {}

pub struct ChangeFeeRecipientInstructionAccounts {
    pub phoenix_program: solana_pubkey::Pubkey,
    pub log_authority: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub market_authority: solana_pubkey::Pubkey,
    pub new_fee_recipient: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ChangeFeeRecipient {
    type ArrangedAccounts = ChangeFeeRecipientInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [phoenix_program, log_authority, market, market_authority, new_fee_recipient, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(ChangeFeeRecipientInstructionAccounts {
            phoenix_program: phoenix_program.pubkey,
            log_authority: log_authority.pubkey,
            market: market.pubkey,
            market_authority: market_authority.pubkey,
            new_fee_recipient: new_fee_recipient.pubkey,
        })
    }
}
