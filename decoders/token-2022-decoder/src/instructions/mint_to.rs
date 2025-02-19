use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x07")]
pub struct MintTo {
    pub amount: u64,
}

pub struct MintToInstructionAccounts {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub token: solana_sdk::pubkey::Pubkey,
    pub mint_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MintTo {
    type ArrangedAccounts = MintToInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, token, mint_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(MintToInstructionAccounts {
            mint: mint.pubkey,
            token: token.pubkey,
            mint_authority: mint_authority.pubkey,
        })
    }
}
