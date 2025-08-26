use carbon_core::{account_utils::next_account, borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x208f535e17223bef")]
pub struct RemainingAccountsStub {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RemainingAccountsStubInstructionAccounts {
    pub create_protocol_config_remaining_accounts: solana_pubkey::Pubkey,
    pub kamino_lending_user_metadata: solana_pubkey::Pubkey,
    pub kamino_lending_init_obligation: solana_pubkey::Pubkey,
    pub kamino_lending_refresh_reserve: solana_pubkey::Pubkey,
    pub kamino_lending_refresh_obligation: solana_pubkey::Pubkey,
    pub refresh_msol_price_list: solana_pubkey::Pubkey,
    pub msol_liquid_staking: solana_pubkey::Pubkey,
    pub msol_liquid_staking_cpi: solana_pubkey::Pubkey,
    pub kamino_deposit_with_farm: solana_pubkey::Pubkey,
    pub kamino_deposit_with_farm_cpi: solana_pubkey::Pubkey,
    pub kamino_deposit_with_farm_client: solana_pubkey::Pubkey,
    pub kamino_borrow_with_farm: solana_pubkey::Pubkey,
    pub kamino_borrow_with_farm_cpi: solana_pubkey::Pubkey,
    pub kamino_borrow_with_farm_client: solana_pubkey::Pubkey,
    pub kamino_repay_with_farm: solana_pubkey::Pubkey,
    pub kamino_repay_with_farm_cpi: solana_pubkey::Pubkey,
    pub kamino_repay_with_farm_client: solana_pubkey::Pubkey,
    pub order_unstake_msol_client: solana_pubkey::Pubkey,
    pub order_unstake_msol_cpi: solana_pubkey::Pubkey,
    pub claim_msol_cpi: solana_pubkey::Pubkey,
    pub kamino_withdraw_with_farm: solana_pubkey::Pubkey,
    pub kamino_withdraw_with_farm_cpi: solana_pubkey::Pubkey,
    pub kamino_withdraw_with_farm_client: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemainingAccountsStub {
    type ArrangedAccounts = RemainingAccountsStubInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let create_protocol_config_remaining_accounts = next_account(&mut iter)?;
        let kamino_lending_user_metadata = next_account(&mut iter)?;
        let kamino_lending_init_obligation = next_account(&mut iter)?;
        let kamino_lending_refresh_reserve = next_account(&mut iter)?;
        let kamino_lending_refresh_obligation = next_account(&mut iter)?;
        let refresh_msol_price_list = next_account(&mut iter)?;
        let msol_liquid_staking = next_account(&mut iter)?;
        let msol_liquid_staking_cpi = next_account(&mut iter)?;
        let kamino_deposit_with_farm = next_account(&mut iter)?;
        let kamino_deposit_with_farm_cpi = next_account(&mut iter)?;
        let kamino_deposit_with_farm_client = next_account(&mut iter)?;
        let kamino_borrow_with_farm = next_account(&mut iter)?;
        let kamino_borrow_with_farm_cpi = next_account(&mut iter)?;
        let kamino_borrow_with_farm_client = next_account(&mut iter)?;
        let kamino_repay_with_farm = next_account(&mut iter)?;
        let kamino_repay_with_farm_cpi = next_account(&mut iter)?;
        let kamino_repay_with_farm_client = next_account(&mut iter)?;
        let order_unstake_msol_client = next_account(&mut iter)?;
        let order_unstake_msol_cpi = next_account(&mut iter)?;
        let claim_msol_cpi = next_account(&mut iter)?;
        let kamino_withdraw_with_farm = next_account(&mut iter)?;
        let kamino_withdraw_with_farm_cpi = next_account(&mut iter)?;
        let kamino_withdraw_with_farm_client = next_account(&mut iter)?;

        Some(RemainingAccountsStubInstructionAccounts {
            create_protocol_config_remaining_accounts,
            kamino_lending_user_metadata,
            kamino_lending_init_obligation,
            kamino_lending_refresh_reserve,
            kamino_lending_refresh_obligation,
            refresh_msol_price_list,
            msol_liquid_staking,
            msol_liquid_staking_cpi,
            kamino_deposit_with_farm,
            kamino_deposit_with_farm_cpi,
            kamino_deposit_with_farm_client,
            kamino_borrow_with_farm,
            kamino_borrow_with_farm_cpi,
            kamino_borrow_with_farm_client,
            kamino_repay_with_farm,
            kamino_repay_with_farm_cpi,
            kamino_repay_with_farm_client,
            order_unstake_msol_client,
            order_unstake_msol_cpi,
            claim_msol_cpi,
            kamino_withdraw_with_farm,
            kamino_withdraw_with_farm_cpi,
            kamino_withdraw_with_farm_client,
        })
    }
}
