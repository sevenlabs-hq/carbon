use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x19")]
pub struct MintConfigClose {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct MintConfigCloseInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub mint_config: solana_pubkey::Pubkey,
    pub authority_config: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MintConfigClose {
    type ArrangedAccounts = MintConfigCloseInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let mint_config = next_account(&mut iter)?;
        let authority_config = next_account(&mut iter)?;

        Some(MintConfigCloseInstructionAccounts {
            authority,
            mint_config,
            authority_config,
        })
    }
}
