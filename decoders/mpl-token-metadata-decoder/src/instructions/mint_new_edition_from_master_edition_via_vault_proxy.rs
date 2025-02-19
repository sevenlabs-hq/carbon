use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0d")]
pub struct MintNewEditionFromMasterEditionViaVaultProxy {
    pub mint_new_edition_from_master_edition_via_token_args:
        MintNewEditionFromMasterEditionViaTokenArgs,
}

pub struct MintNewEditionFromMasterEditionViaVaultProxyInstructionAccounts {
    pub new_metadata: solana_sdk::pubkey::Pubkey,
    pub new_edition: solana_sdk::pubkey::Pubkey,
    pub master_edition: solana_sdk::pubkey::Pubkey,
    pub new_mint: solana_sdk::pubkey::Pubkey,
    pub edition_mark_pda: solana_sdk::pubkey::Pubkey,
    pub new_mint_authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub vault_authority: solana_sdk::pubkey::Pubkey,
    pub safety_deposit_store: solana_sdk::pubkey::Pubkey,
    pub safety_deposit_box: solana_sdk::pubkey::Pubkey,
    pub vault: solana_sdk::pubkey::Pubkey,
    pub new_metadata_update_authority: solana_sdk::pubkey::Pubkey,
    pub metadata: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_vault_program: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MintNewEditionFromMasterEditionViaVaultProxy {
    type ArrangedAccounts = MintNewEditionFromMasterEditionViaVaultProxyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [new_metadata, new_edition, master_edition, new_mint, edition_mark_pda, new_mint_authority, payer, vault_authority, safety_deposit_store, safety_deposit_box, vault, new_metadata_update_authority, metadata, token_program, token_vault_program, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(
            MintNewEditionFromMasterEditionViaVaultProxyInstructionAccounts {
                new_metadata: new_metadata.pubkey,
                new_edition: new_edition.pubkey,
                master_edition: master_edition.pubkey,
                new_mint: new_mint.pubkey,
                edition_mark_pda: edition_mark_pda.pubkey,
                new_mint_authority: new_mint_authority.pubkey,
                payer: payer.pubkey,
                vault_authority: vault_authority.pubkey,
                safety_deposit_store: safety_deposit_store.pubkey,
                safety_deposit_box: safety_deposit_box.pubkey,
                vault: vault.pubkey,
                new_metadata_update_authority: new_metadata_update_authority.pubkey,
                metadata: metadata.pubkey,
                token_program: token_program.pubkey,
                token_vault_program: token_vault_program.pubkey,
                system_program: system_program.pubkey,
                rent: rent.pubkey,
            },
        )
    }
}
