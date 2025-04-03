use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc8d83fef07e6ff14")]
pub struct CloseOpenOrders {
    pub map_nonce: u8,
}

pub struct CloseOpenOrdersInstructionAccounts {
    pub state: solana_pubkey::Pubkey,
    pub zeta_group: solana_pubkey::Pubkey,
    pub dex_program: solana_pubkey::Pubkey,
    pub open_orders: solana_pubkey::Pubkey,
    pub margin_account: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub market: solana_pubkey::Pubkey,
    pub serum_authority: solana_pubkey::Pubkey,
    pub open_orders_map: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseOpenOrders {
    type ArrangedAccounts = CloseOpenOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [state, zeta_group, dex_program, open_orders, margin_account, authority, market, serum_authority, open_orders_map, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CloseOpenOrdersInstructionAccounts {
            state: state.pubkey,
            zeta_group: zeta_group.pubkey,
            dex_program: dex_program.pubkey,
            open_orders: open_orders.pubkey,
            margin_account: margin_account.pubkey,
            authority: authority.pubkey,
            market: market.pubkey,
            serum_authority: serum_authority.pubkey,
            open_orders_map: open_orders_map.pubkey,
        })
    }
}
