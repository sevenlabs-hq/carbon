use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x33e685a4017f83ad")]
pub struct Sell {
    pub amount: u64,
    pub min_amount_out: u64,
}

pub struct SellInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub vpool: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub user_virtuals_ata: solana_pubkey::Pubkey,
    pub user_token_ata: solana_pubkey::Pubkey,
    pub vpool_token_ata: solana_pubkey::Pubkey,
    pub platform_prototype: solana_pubkey::Pubkey,
    pub platform_prototype_virtuals_ata: solana_pubkey::Pubkey,
    pub vpool_virtuals_ata: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Sell {
    type ArrangedAccounts = SellInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, vpool, token_mint, user_virtuals_ata, user_token_ata, vpool_token_ata, platform_prototype, platform_prototype_virtuals_ata, vpool_virtuals_ata, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SellInstructionAccounts {
            user: user.pubkey,
            vpool: vpool.pubkey,
            token_mint: token_mint.pubkey,
            user_virtuals_ata: user_virtuals_ata.pubkey,
            user_token_ata: user_token_ata.pubkey,
            vpool_token_ata: vpool_token_ata.pubkey,
            platform_prototype: platform_prototype.pubkey,
            platform_prototype_virtuals_ata: platform_prototype_virtuals_ata.pubkey,
            vpool_virtuals_ata: vpool_virtuals_ata.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
