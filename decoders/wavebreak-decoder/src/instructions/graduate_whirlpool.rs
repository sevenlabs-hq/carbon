use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x20")]
pub struct GraduateWhirlpool {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct GraduateWhirlpoolInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub lp_authority: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub signer_quote_ata: solana_pubkey::Pubkey,
    pub lp_authority_quote_ata: solana_pubkey::Pubkey,
    pub whirlpool_quote_vault: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub lp_authority_base_ata: solana_pubkey::Pubkey,
    pub whirlpool_base_vault: solana_pubkey::Pubkey,
    pub whirlpool_config: solana_pubkey::Pubkey,
    pub fee_tier: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub position: solana_pubkey::Pubkey,
    pub position_mint: solana_pubkey::Pubkey,
    pub position_token_account: solana_pubkey::Pubkey,
    pub lp_authority_token_account: solana_pubkey::Pubkey,
    pub lower_tick_array: solana_pubkey::Pubkey,
    pub upper_tick_array: solana_pubkey::Pubkey,
    pub quote_token_badge: solana_pubkey::Pubkey,
    pub base_token_badge: solana_pubkey::Pubkey,
    pub whirlpool_init_authority: solana_pubkey::Pubkey,
    pub whirlpool_update_authority: solana_pubkey::Pubkey,
    pub lock_config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub ata_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
    pub base_token_program: solana_pubkey::Pubkey,
    pub token22_program: solana_pubkey::Pubkey,
    pub memo_program: solana_pubkey::Pubkey,
    pub whirlpool_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GraduateWhirlpool {
    type ArrangedAccounts = GraduateWhirlpoolInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let signer = next_account(&mut iter)?;
        let lp_authority = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let signer_quote_ata = next_account(&mut iter)?;
        let lp_authority_quote_ata = next_account(&mut iter)?;
        let whirlpool_quote_vault = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let lp_authority_base_ata = next_account(&mut iter)?;
        let whirlpool_base_vault = next_account(&mut iter)?;
        let whirlpool_config = next_account(&mut iter)?;
        let fee_tier = next_account(&mut iter)?;
        let whirlpool = next_account(&mut iter)?;
        let oracle = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let position_mint = next_account(&mut iter)?;
        let position_token_account = next_account(&mut iter)?;
        let lp_authority_token_account = next_account(&mut iter)?;
        let lower_tick_array = next_account(&mut iter)?;
        let upper_tick_array = next_account(&mut iter)?;
        let quote_token_badge = next_account(&mut iter)?;
        let base_token_badge = next_account(&mut iter)?;
        let whirlpool_init_authority = next_account(&mut iter)?;
        let whirlpool_update_authority = next_account(&mut iter)?;
        let lock_config = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let ata_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let token22_program = next_account(&mut iter)?;
        let memo_program = next_account(&mut iter)?;
        let whirlpool_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;

        Some(GraduateWhirlpoolInstructionAccounts {
            signer,
            lp_authority,
            bonding_curve,
            quote_mint,
            quote_vault,
            signer_quote_ata,
            lp_authority_quote_ata,
            whirlpool_quote_vault,
            base_mint,
            base_vault,
            lp_authority_base_ata,
            whirlpool_base_vault,
            whirlpool_config,
            fee_tier,
            whirlpool,
            oracle,
            position,
            position_mint,
            position_token_account,
            lp_authority_token_account,
            lower_tick_array,
            upper_tick_array,
            quote_token_badge,
            base_token_badge,
            whirlpool_init_authority,
            whirlpool_update_authority,
            lock_config,
            system_program,
            ata_program,
            quote_token_program,
            base_token_program,
            token22_program,
            memo_program,
            whirlpool_program,
            rent,
        })
    }
}
