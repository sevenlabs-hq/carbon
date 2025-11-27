use carbon_core::deserialize::ArrangeAccounts;
use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x6c92566eb3fe0a68")]
pub struct CloseTokenBadge {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseTokenBadgeInstructionAccounts {
    pub token_badge: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
    pub rent_receiver: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl ArrangeAccounts for CloseTokenBadge {
    type ArrangedAccounts = CloseTokenBadgeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_badge, admin, rent_receiver, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CloseTokenBadgeInstructionAccounts {
            token_badge: token_badge.pubkey,
            admin: admin.pubkey,
            rent_receiver: rent_receiver.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
