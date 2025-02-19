use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x398b2f7bd850df0a")]
pub struct SetPreActivationSwapAddress {
    pub pre_activation_swap_address: solana_sdk::pubkey::Pubkey,
}

pub struct SetPreActivationSwapAddressInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub creator: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetPreActivationSwapAddress {
    type ArrangedAccounts = SetPreActivationSwapAddressInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, creator, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetPreActivationSwapAddressInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            creator: creator.pubkey,
        })
    }
}
