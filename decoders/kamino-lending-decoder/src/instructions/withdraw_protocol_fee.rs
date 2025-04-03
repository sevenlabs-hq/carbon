use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9ec99ebd215da267")]
pub struct WithdrawProtocolFee {
    pub amount: u64,
}

pub struct WithdrawProtocolFeeInstructionAccounts {
    pub lending_market_owner: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub reserve: solana_pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_pubkey::Pubkey,
    pub lending_market_authority: solana_pubkey::Pubkey,
    pub fee_vault: solana_pubkey::Pubkey,
    pub lending_market_owner_ata: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawProtocolFee {
    type ArrangedAccounts = WithdrawProtocolFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lending_market_owner, lending_market, reserve, reserve_liquidity_mint, lending_market_authority, fee_vault, lending_market_owner_ata, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawProtocolFeeInstructionAccounts {
            lending_market_owner: lending_market_owner.pubkey,
            lending_market: lending_market.pubkey,
            reserve: reserve.pubkey,
            reserve_liquidity_mint: reserve_liquidity_mint.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            fee_vault: fee_vault.pubkey,
            lending_market_owner_ata: lending_market_owner_ata.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
