use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa039592ab58b2b42")]
pub struct CollectCoinCreatorFee {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CollectCoinCreatorFeeInstructionAccounts {
    pub quote_mint: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub coin_creator: solana_pubkey::Pubkey,
    pub coin_creator_vault_authority: solana_pubkey::Pubkey,
    pub coin_creator_vault_ata: solana_pubkey::Pubkey,
    pub coin_creator_token_account: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectCoinCreatorFee {
    type ArrangedAccounts = CollectCoinCreatorFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [quote_mint, quote_token_program, coin_creator, coin_creator_vault_authority, coin_creator_vault_ata, coin_creator_token_account, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectCoinCreatorFeeInstructionAccounts {
            quote_mint: quote_mint.pubkey,
            quote_token_program: quote_token_program.pubkey,
            coin_creator: coin_creator.pubkey,
            coin_creator_vault_authority: coin_creator_vault_authority.pubkey,
            coin_creator_vault_ata: coin_creator_vault_ata.pubkey,
            coin_creator_token_account: coin_creator_token_account.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
