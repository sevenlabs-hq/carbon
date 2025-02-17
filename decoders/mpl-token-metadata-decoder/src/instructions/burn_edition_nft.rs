use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x25")]
pub struct BurnEditionNft {}

pub struct BurnEditionNftInstructionAccounts {
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub print_edition_mint: solana_sdk::pubkey::Pubkey,
    pub master_edition_mint: solana_sdk::pubkey::Pubkey,
    pub print_edition_token_account: solana_sdk::pubkey::Pubkey,
    pub master_edition_token_account: solana_sdk::pubkey::Pubkey,
    pub master_edition_account: solana_sdk::pubkey::Pubkey,
    pub print_edition_account: solana_sdk::pubkey::Pubkey,
    pub edition_marker_account: solana_sdk::pubkey::Pubkey,
    pub spl_token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BurnEditionNft {
    type ArrangedAccounts = BurnEditionNftInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, owner, print_edition_mint, master_edition_mint, print_edition_token_account, master_edition_token_account, master_edition_account, print_edition_account, edition_marker_account, spl_token_program] =
            accounts
        else {
            return None;
        };

        Some(BurnEditionNftInstructionAccounts {
            metadata: metadata.pubkey,
            owner: owner.pubkey,
            print_edition_mint: print_edition_mint.pubkey,
            master_edition_mint: master_edition_mint.pubkey,
            print_edition_token_account: print_edition_token_account.pubkey,
            master_edition_token_account: master_edition_token_account.pubkey,
            master_edition_account: master_edition_account.pubkey,
            print_edition_account: print_edition_account.pubkey,
            edition_marker_account: edition_marker_account.pubkey,
            spl_token_program: spl_token_program.pubkey,
        })
    }
}
