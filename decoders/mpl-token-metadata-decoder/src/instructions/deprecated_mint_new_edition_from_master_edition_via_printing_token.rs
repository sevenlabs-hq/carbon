use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x03")]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingToken {}

pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub edition: solana_pubkey::Pubkey,
    pub master_edition: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
    pub printing_mint: solana_pubkey::Pubkey,
    pub master_token_account: solana_pubkey::Pubkey,
    pub edition_marker: solana_pubkey::Pubkey,
    pub burn_authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub master_update_authority: solana_pubkey::Pubkey,
    pub master_metadata: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub reservation_list: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts
    for DeprecatedMintNewEditionFromMasterEditionViaPrintingToken
{
    type ArrangedAccounts =
        DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, edition, master_edition, mint, mint_authority, printing_mint, master_token_account, edition_marker, burn_authority, payer, master_update_authority, master_metadata, token_program, system_program, rent, reservation_list, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(
            DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenInstructionAccounts {
                metadata: metadata.pubkey,
                edition: edition.pubkey,
                master_edition: master_edition.pubkey,
                mint: mint.pubkey,
                mint_authority: mint_authority.pubkey,
                printing_mint: printing_mint.pubkey,
                master_token_account: master_token_account.pubkey,
                edition_marker: edition_marker.pubkey,
                burn_authority: burn_authority.pubkey,
                payer: payer.pubkey,
                master_update_authority: master_update_authority.pubkey,
                master_metadata: master_metadata.pubkey,
                token_program: token_program.pubkey,
                system_program: system_program.pubkey,
                rent: rent.pubkey,
                reservation_list: reservation_list.pubkey,
            },
        )
    }
}
