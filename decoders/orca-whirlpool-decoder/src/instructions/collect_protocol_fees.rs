use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1643176296b246dc")]
pub struct CollectProtocolFees {}

pub struct CollectProtocolFeesInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub collect_protocol_fees_authority: solana_sdk::pubkey::Pubkey,
    pub token_vault_a: solana_sdk::pubkey::Pubkey,
    pub token_vault_b: solana_sdk::pubkey::Pubkey,
    pub token_destination_a: solana_sdk::pubkey::Pubkey,
    pub token_destination_b: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectProtocolFees {
    type ArrangedAccounts = CollectProtocolFeesInstructionAccounts;

fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let whirlpools_config = accounts.get(0)?;
        let whirlpool = accounts.get(1)?;
        let collect_protocol_fees_authority = accounts.get(2)?;
        let token_vault_a = accounts.get(3)?;
        let token_vault_b = accounts.get(4)?;
        let token_destination_a = accounts.get(5)?;
        let token_destination_b = accounts.get(6)?;
        let token_program = accounts.get(7)?;

        Some(CollectProtocolFeesInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            whirlpool: whirlpool.pubkey,
            collect_protocol_fees_authority: collect_protocol_fees_authority.pubkey,
            token_vault_a: token_vault_a.pubkey,
            token_vault_b: token_vault_b.pubkey,
            token_destination_a: token_destination_a.pubkey,
            token_destination_b: token_destination_b.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
