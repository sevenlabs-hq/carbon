use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1643176296b246dc")]
pub struct CollectProtocolFees {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CollectProtocolFeesInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub collect_protocol_fees_authority: solana_pubkey::Pubkey,
    pub token_vault_a: solana_pubkey::Pubkey,
    pub token_vault_b: solana_pubkey::Pubkey,
    pub token_destination_a: solana_pubkey::Pubkey,
    pub token_destination_b: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectProtocolFees {
    type ArrangedAccounts = CollectProtocolFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let whirlpool = next_account(&mut iter)?;
        let collect_protocol_fees_authority = next_account(&mut iter)?;
        let token_vault_a = next_account(&mut iter)?;
        let token_vault_b = next_account(&mut iter)?;
        let token_destination_a = next_account(&mut iter)?;
        let token_destination_b = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(CollectProtocolFeesInstructionAccounts {
            whirlpools_config,
            whirlpool,
            collect_protocol_fees_authority,
            token_vault_a,
            token_vault_b,
            token_destination_a,
            token_destination_b,
            token_program,
        })
    }
}
