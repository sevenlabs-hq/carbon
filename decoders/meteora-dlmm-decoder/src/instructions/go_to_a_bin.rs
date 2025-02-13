use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9248aee028fd54ae")]
pub struct GoToABin {
    pub bin_id: i32,
}

pub struct GoToABinInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_array_bitmap_extension: solana_sdk::pubkey::Pubkey,
    pub from_bin_array: solana_sdk::pubkey::Pubkey,
    pub to_bin_array: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GoToABin {
    type ArrangedAccounts = GoToABinInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let lb_pair = accounts.get(0)?;
        let bin_array_bitmap_extension = accounts.get(1)?;
        let from_bin_array = accounts.get(2)?;
        let to_bin_array = accounts.get(3)?;
        let event_authority = accounts.get(4)?;
        let program = accounts.get(5)?;

        Some(GoToABinInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            bin_array_bitmap_extension: bin_array_bitmap_extension.pubkey,
            from_bin_array: from_bin_array.pubkey,
            to_bin_array: to_bin_array.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
