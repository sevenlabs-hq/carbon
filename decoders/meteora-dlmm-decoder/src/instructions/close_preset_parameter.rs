use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x04949164861ab53d")]
pub struct ClosePresetParameter {}

pub struct ClosePresetParameterInstructionAccounts {
    pub preset_parameter: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub rent_receiver: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ClosePresetParameter {
    type ArrangedAccounts = ClosePresetParameterInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let preset_parameter = accounts.get(0)?;
        let admin = accounts.get(1)?;
        let rent_receiver = accounts.get(2)?;

        Some(ClosePresetParameterInstructionAccounts {
            preset_parameter: preset_parameter.pubkey,
            admin: admin.pubkey,
            rent_receiver: rent_receiver.pubkey,
        })
    }
}
