use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xae6de401f269e869")]
pub struct BeginSwap {
    pub in_market_index: u16,
    pub out_market_index: u16,
    pub amount_in: u64,
}

pub struct BeginSwapInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub user_stats: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub out_spot_market_vault: solana_pubkey::Pubkey,
    pub in_spot_market_vault: solana_pubkey::Pubkey,
    pub out_token_account: solana_pubkey::Pubkey,
    pub in_token_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub drift_signer: solana_pubkey::Pubkey,
    pub instructions: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BeginSwap {
    type ArrangedAccounts = BeginSwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [
            state,
            user,
            user_stats,
            authority,
            out_spot_market_vault,
            in_spot_market_vault,
            out_token_account,
            in_token_account,
            token_program,
            drift_signer,
            instructions,
            _remaining @ ..,
        ] = accounts
        else {
            return None;
        };

        Some(BeginSwapInstructionAccounts {
            state: state.pubkey,
            user: user.pubkey,
            user_stats: user_stats.pubkey,
            authority: authority.pubkey,
            out_spot_market_vault: out_spot_market_vault.pubkey,
            in_spot_market_vault: in_spot_market_vault.pubkey,
            out_token_account: out_token_account.pubkey,
            in_token_account: in_token_account.pubkey,
            token_program: token_program.pubkey,
            drift_signer: drift_signer.pubkey,
            instructions: instructions.pubkey,
        })
    }
}
