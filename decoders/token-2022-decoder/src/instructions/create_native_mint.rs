use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1f")]
pub struct CreateNativeMint {}

pub struct CreateNativeMintInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub native_mint: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateNativeMint {
    type ArrangedAccounts = CreateNativeMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, native_mint, system_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateNativeMintInstructionAccounts {
            payer: payer.pubkey,
            native_mint: native_mint.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
