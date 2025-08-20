use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x22965df48be1e943")]
pub struct SetCollectProtocolFeesAuthority {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct SetCollectProtocolFeesAuthorityInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub collect_protocol_fees_authority: solana_pubkey::Pubkey,
    pub new_collect_protocol_fees_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetCollectProtocolFeesAuthority {
    type ArrangedAccounts = SetCollectProtocolFeesAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let whirlpools_config = next_account(&mut iter)?;
        let collect_protocol_fees_authority = next_account(&mut iter)?;
        let new_collect_protocol_fees_authority = next_account(&mut iter)?;

        Some(SetCollectProtocolFeesAuthorityInstructionAccounts {
            whirlpools_config,
            collect_protocol_fees_authority,
            new_collect_protocol_fees_authority,
        })
    }
}
