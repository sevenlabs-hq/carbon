use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa53dc9f4829f1664")]
pub struct SetPreActivationDuration {
    pub pre_activation_duration: u64,
}

pub struct SetPreActivationDurationInstructionAccounts {
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub creator: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetPreActivationDuration {
    type ArrangedAccounts = SetPreActivationDurationInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lb_pair, creator] = accounts else {
            return None;
        };

        Some(SetPreActivationDurationInstructionAccounts {
            lb_pair: lb_pair.pubkey,
            creator: creator.pubkey,
        })
    }
}
