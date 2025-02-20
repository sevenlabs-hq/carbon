use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa4639c7e9c3963b4")]
pub struct InitializeSignedMsgUserOrders {
    pub num_orders: u16,
}

pub struct InitializeSignedMsgUserOrdersInstructionAccounts {
    pub signed_msg_user_orders: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeSignedMsgUserOrders {
    type ArrangedAccounts = InitializeSignedMsgUserOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [signed_msg_user_orders, authority, payer, rent, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeSignedMsgUserOrdersInstructionAccounts {
            signed_msg_user_orders: signed_msg_user_orders.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
