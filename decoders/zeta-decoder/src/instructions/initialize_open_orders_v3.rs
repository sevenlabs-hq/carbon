use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x16bf8b88792754ca")]
pub struct InitializeOpenOrdersV3 {
    pub asset: Asset,
}

pub struct InitializeOpenOrdersV3InstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub open_orders: solana_pubkey::Pubkey,
    pub cross_margin_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub open_orders_map: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeOpenOrdersV3 {
    type ArrangedAccounts = InitializeOpenOrdersV3InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, dex_program, system_program, open_orders, cross_margin_account, authority, payer, market, serum_authority, open_orders_map, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeOpenOrdersV3InstructionAccounts {
            state: state.pubkey,
            dex_program: dex_program.pubkey,
            system_program: system_program.pubkey,
            open_orders: open_orders.pubkey,
            cross_margin_account: cross_margin_account.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            market: market.pubkey,
            serum_authority: serum_authority.pubkey,
            open_orders_map: open_orders_map.pubkey,
            rent: rent.pubkey,
        })
    }
}
