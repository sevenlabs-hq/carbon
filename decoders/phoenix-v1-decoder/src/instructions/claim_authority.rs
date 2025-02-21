use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x65")]
pub struct ClaimAuthority {}

pub struct ClaimAuthorityInstructionAccounts {
    pub phoenix_program: solana_sdk::pubkey::Pubkey,
    pub log_authority: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub successor: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClaimAuthority {
    type ArrangedAccounts = ClaimAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [phoenix_program, log_authority, market, successor, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ClaimAuthorityInstructionAccounts {
            phoenix_program: phoenix_program.pubkey,
            log_authority: log_authority.pubkey,
            market: market.pubkey,
            successor: successor.pubkey,
        })
    }
}
