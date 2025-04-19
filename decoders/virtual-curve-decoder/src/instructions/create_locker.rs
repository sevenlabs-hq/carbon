use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa75a899a4b2f1154")]
pub struct CreateLocker {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateLockerInstructionAccounts {
    pub virtual_pool: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub pool_authority: solana_pubkey::Pubkey,
    pub base_vault: solana_pubkey::Pubkey,
    pub base_mint: solana_pubkey::Pubkey,
    pub base: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub escrow: solana_pubkey::Pubkey,
    pub escrow_token: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub locker_program: solana_pubkey::Pubkey,
    pub locker_event_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateLocker {
    type ArrangedAccounts = CreateLockerInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [virtual_pool, config, pool_authority, base_vault, base_mint, base, creator, escrow, escrow_token, payer, token_program, locker_program, locker_event_authority, system_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateLockerInstructionAccounts {
            virtual_pool: virtual_pool.pubkey,
            config: config.pubkey,
            pool_authority: pool_authority.pubkey,
            base_vault: base_vault.pubkey,
            base_mint: base_mint.pubkey,
            base: base.pubkey,
            creator: creator.pubkey,
            escrow: escrow.pubkey,
            escrow_token: escrow_token.pubkey,
            payer: payer.pubkey,
            token_program: token_program.pubkey,
            locker_program: locker_program.pubkey,
            locker_event_authority: locker_event_authority.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
