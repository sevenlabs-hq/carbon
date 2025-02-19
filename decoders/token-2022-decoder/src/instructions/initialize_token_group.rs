use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x79716c2736330004")]
pub struct InitializeTokenGroup {
    pub update_authority: Option<solana_sdk::pubkey::Pubkey>,
    pub max_size: u64,
}

pub struct InitializeTokenGroupInstructionAccounts {
    pub group: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeTokenGroup {
    type ArrangedAccounts = InitializeTokenGroupInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [group, mint, mint_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeTokenGroupInstructionAccounts {
            group: group.pubkey,
            mint: mint.pubkey,
            mint_authority: mint_authority.pubkey,
        })
    }
}
