use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x22a2740e65895eef")]
pub struct InitLendingMarket {
    pub quote_currency: [u8; 32],
}

pub struct InitLendingMarketInstructionAccounts {
    pub lending_market_owner: solana_pubkey::Pubkey,
    pub lending_market: solana_pubkey::Pubkey,
    pub lending_market_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitLendingMarket {
    type ArrangedAccounts = InitLendingMarketInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lending_market_owner, lending_market, lending_market_authority, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitLendingMarketInstructionAccounts {
            lending_market_owner: lending_market_owner.pubkey,
            lending_market: lending_market.pubkey,
            lending_market_authority: lending_market_authority.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
