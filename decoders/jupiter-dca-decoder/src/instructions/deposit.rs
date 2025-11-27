use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf223c68952e1f2b6")]
pub struct Deposit {
    pub deposit_in: u64,
}

pub struct DepositInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub dca: solana_pubkey::Pubkey,
    pub in_ata: solana_pubkey::Pubkey,
    pub user_in_ata: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, dca, in_ata, user_in_ata, token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DepositInstructionAccounts {
            user: user.pubkey,
            dca: dca.pubkey,
            in_ata: in_ata.pubkey,
            user_in_ata: user_in_ata.pubkey,
            token_program: token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
