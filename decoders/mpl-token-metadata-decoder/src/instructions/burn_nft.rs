

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1d")]
pub struct BurnNft{
}

pub struct BurnNftInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub master_edition_account: solana_sdk::pubkey::Pubkey,
    pub spl_token_program: solana_sdk::pubkey::Pubkey,
    pub collection_metadata: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BurnNft {
    type ArrangedAccounts = BurnNftInstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let metadata = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let mint = accounts.get(2)?;
        let token_account = accounts.get(3)?;
        let master_edition_account = accounts.get(4)?;
        let spl_token_program = accounts.get(5)?;
        let collection_metadata = accounts.get(6)?;

        Some(BurnNftInstructionAccounts {
            metadata: metadata.pubkey,
            owner: owner.pubkey,
            mint: mint.pubkey,
            token_account: token_account.pubkey,
            master_edition_account: master_edition_account.pubkey,
            spl_token_program: spl_token_program.pubkey,
            collection_metadata: collection_metadata.pubkey,
        })
    }
}