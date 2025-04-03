use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0d09d36b3eace043")]
pub struct InitFee {
    pub maker_fee: u64,
    pub maker_stable_fee: u64,
    pub taker_fee: u64,
    pub taker_stable_fee: u64,
}

pub struct InitFeeInstructionAccounts {
    pub keeper: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitFee {
    type ArrangedAccounts = InitFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [keeper, fee_authority, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitFeeInstructionAccounts {
            keeper: keeper.pubkey,
            fee_authority: fee_authority.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
