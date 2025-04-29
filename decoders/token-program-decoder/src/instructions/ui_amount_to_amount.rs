use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x18")]
pub struct UiAmountToAmount {
    // TODO: Check what to do here, on github it's: &'a str
    pub ui_amount: String,
}

pub struct UiAmountToAmountAccounts {
    pub mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UiAmountToAmount {
    type ArrangedAccounts = UiAmountToAmountAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.first()?;

        Some(UiAmountToAmountAccounts { mint: mint.pubkey })
    }
}
