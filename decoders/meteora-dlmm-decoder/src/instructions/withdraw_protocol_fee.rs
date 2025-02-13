use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9ec99ebd215da267")]
pub struct WithdrawProtocolFee {
    pub amount_x: u64,
    pub amount_y: u64,
}

pub struct WithdrawProtocolFeeInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub reserve_x: solana_sdk::pubkey::Pubkey,
    pub reserve_y: solana_sdk::pubkey::Pubkey,
    pub token_x_mint: solana_sdk::pubkey::Pubkey,
    pub token_y_mint: solana_sdk::pubkey::Pubkey,
    pub receiver_token_x: solana_sdk::pubkey::Pubkey,
    pub receiver_token_y: solana_sdk::pubkey::Pubkey,
    pub token_x_program: solana_sdk::pubkey::Pubkey,
    pub token_y_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawProtocolFee {
    type ArrangedAccounts = WithdrawProtocolFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let lb_pair = accounts.get(0)?;
        let reserve_x = accounts.get(1)?;
        let reserve_y = accounts.get(2)?;
        let token_x_mint = accounts.get(3)?;
        let token_y_mint = accounts.get(4)?;
        let receiver_token_x = accounts.get(5)?;
        let receiver_token_y = accounts.get(6)?;
        let token_x_program = accounts.get(7)?;
        let token_y_program = accounts.get(8)?;

        Some(WithdrawProtocolFeeInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            reserve_x: reserve_x.pubkey,
            reserve_y: reserve_y.pubkey,
            token_x_mint: token_x_mint.pubkey,
            token_y_mint: token_y_mint.pubkey,
            receiver_token_x: receiver_token_x.pubkey,
            receiver_token_y: receiver_token_y.pubkey,
            token_x_program: token_x_program.pubkey,
            token_y_program: token_y_program.pubkey,
        })
    }
}
