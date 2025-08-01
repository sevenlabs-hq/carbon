use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x31")]
pub struct BondingCurveCollectFees {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BondingCurveCollectFeesInstructionAccounts {
    pub signer: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
    pub fee_authority_ata: solana_pubkey::Pubkey,
    pub bonding_curve: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub quote_mint: solana_pubkey::Pubkey,
    pub quote_vault: solana_pubkey::Pubkey,
    pub authority_config: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub ata_program: solana_pubkey::Pubkey,
    pub quote_token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BondingCurveCollectFees {
    type ArrangedAccounts = BondingCurveCollectFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let signer = next_account(&mut iter)?;
        let fee_authority = next_account(&mut iter)?;
        let fee_authority_ata = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let authority_config = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let ata_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;

        Some(BondingCurveCollectFeesInstructionAccounts {
            signer,
            fee_authority,
            fee_authority_ata,
            bonding_curve,
            base_mint,
            quote_mint,
            quote_vault,
            authority_config,
            system_program,
            ata_program,
            quote_token_program,
        })
    }
}
