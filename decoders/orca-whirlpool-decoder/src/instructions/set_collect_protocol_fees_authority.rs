use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x22965df48be1e943")]
pub struct SetCollectProtocolFeesAuthority {}

pub struct SetCollectProtocolFeesAuthorityInstructionAccounts {
    pub whirlpools_config: solana_sdk::pubkey::Pubkey,
    pub collect_protocol_fees_authority: solana_sdk::pubkey::Pubkey,
    pub new_collect_protocol_fees_authority: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for SetCollectProtocolFeesAuthority {
    type ArrangedAccounts = SetCollectProtocolFeesAuthorityInstructionAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let whirlpools_config = accounts.get(0)?;
        let collect_protocol_fees_authority = accounts.get(1)?;
        let new_collect_protocol_fees_authority = accounts.get(2)?;

        Some(SetCollectProtocolFeesAuthorityInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            collect_protocol_fees_authority: collect_protocol_fees_authority.pubkey,
            new_collect_protocol_fees_authority: new_collect_protocol_fees_authority.pubkey,
        })
    }
}
