use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf939bb66b86825e7")]
pub struct InitializeZetaTreasuryWallet {}

pub struct InitializeZetaTreasuryWalletInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub treasury_wallet: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub usdc_mint: solana_pubkey::Pubkey,
    pub admin: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeZetaTreasuryWallet {
    type ArrangedAccounts = InitializeZetaTreasuryWalletInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, treasury_wallet, rent, system_program, token_program, usdc_mint, admin, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeZetaTreasuryWalletInstructionAccounts {
            state: state.pubkey,
            treasury_wallet: treasury_wallet.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            usdc_mint: usdc_mint.pubkey,
            admin: admin.pubkey,
        })
    }
}
