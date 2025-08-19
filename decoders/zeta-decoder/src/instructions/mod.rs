use {super::ZetaDecoder, crate::PROGRAM_ID};
pub mod add_market_indexes;
pub mod add_perp_market_index;
pub mod admin_crank_event_queue;
pub mod admin_force_cancel_orders;
pub mod admin_reset_dex_open_orders;
pub mod admin_set_order_state;
pub mod apply_funding_event;
pub mod apply_perp_funding;
pub mod burn_vault_tokens;
pub mod cancel_all_market_orders;
pub mod cancel_order;
pub mod cancel_order_by_client_order_id;
pub mod cancel_order_by_client_order_id_no_error;
pub mod cancel_order_halted;
pub mod cancel_order_no_error;
pub mod cancel_trigger_order;
pub mod cancel_trigger_order_v2;
pub mod choose_airdrop_community;
pub mod clean_zeta_market_halted;
pub mod clean_zeta_markets;
pub mod close_cross_margin_account;
pub mod close_cross_margin_account_manager;
pub mod close_margin_account;
pub mod close_open_orders;
pub mod close_open_orders_v2;
pub mod close_open_orders_v3;
pub mod close_open_orders_v4;
pub mod close_referrer_accounts;
pub mod close_spread_account;
pub mod collect_treasury_funds;
pub mod crank_event_queue;
pub mod deposit;
pub mod deposit_insurance_vault;
pub mod deposit_insurance_vault_v2;
pub mod deposit_permissionless;
pub mod deposit_v2;
pub mod edit_delegated_pubkey;
pub mod edit_ma_type;
pub mod edit_trigger_order;
pub mod edit_trigger_order_v2;
pub mod execute_trigger_order;
pub mod execute_trigger_order_v2;
pub mod expire_series;
pub mod expire_series_override;
pub mod force_cancel_order_by_order_id;
pub mod force_cancel_order_by_order_id_v2;
pub mod force_cancel_orders;
pub mod force_cancel_orders_v2;
pub mod force_cancel_trigger_order;
pub mod halt;
pub mod initialize_combined_insurance_vault;
pub mod initialize_combined_socialized_loss_account;
pub mod initialize_combined_vault;
pub mod initialize_cross_margin_account;
pub mod initialize_cross_margin_account_manager;
pub mod initialize_cross_margin_account_manager_v2;
pub mod initialize_insurance_deposit_account;
pub mod initialize_margin_account;
pub mod initialize_market_indexes;
pub mod initialize_market_node;
pub mod initialize_market_pda;
pub mod initialize_market_strikes;
pub mod initialize_market_tif_epoch_cycle;
pub mod initialize_min_lots_and_tick_sizes;
pub mod initialize_open_orders;
pub mod initialize_open_orders_v2;
pub mod initialize_open_orders_v3;
pub mod initialize_perp_sync_queue;
pub mod initialize_referrer_accounts;
pub mod initialize_spread_account;
pub mod initialize_underlying;
pub mod initialize_whitelist_deposit_account;
pub mod initialize_whitelist_insurance_account;
pub mod initialize_whitelist_trading_fees_account;
pub mod initialize_zeta_group;
pub mod initialize_zeta_market;
pub mod initialize_zeta_pricing;
pub mod initialize_zeta_referrals_rewards_wallet;
pub mod initialize_zeta_specific_market_vaults;
pub mod initialize_zeta_state;
pub mod initialize_zeta_treasury_wallet;
pub mod liquidate;
pub mod liquidate_v2;
pub mod liquidation_event;
pub mod migrate_to_cross_margin_account;
pub mod migrate_to_new_cross_margin_account;
pub mod order_complete_event;
pub mod override_expiry;
pub mod place_multi_orders;
pub mod place_multi_orders_event;
pub mod place_order;
pub mod place_order_event;
pub mod place_order_v2;
pub mod place_order_v3;
pub mod place_order_v4;
pub mod place_perp_order;
pub mod place_perp_order_v2;
pub mod place_perp_order_v3;
pub mod place_perp_order_v4;
pub mod place_perp_order_v5;
pub mod place_trigger_order;
pub mod position_movement;
pub mod position_movement_event;
pub mod prune_expired_tif_orders;
pub mod prune_expired_tif_orders_v2;
pub mod rebalance_insurance_vault;
pub mod rebalance_insurance_vault_v2;
pub mod reset_num_flex_underlyings;
pub mod settle_dex_funds;
pub mod settle_positions_halted;
pub mod take_trigger_order;
pub mod toggle_market_maker;
pub mod toggle_zeta_group_perps_only;
pub mod trade_event;
pub mod trade_event_v2_event;
pub mod trade_event_v3_event;
pub mod transfer_excess_spread_balance;
pub mod treasury_movement;
pub mod unhalt;
pub mod update_admin;
pub mod update_halt_state;
pub mod update_interest_rate;
pub mod update_ma_type_admin;
pub mod update_maker_rebate_percentage;
pub mod update_margin_parameters;
pub mod update_min_lot;
pub mod update_oracle;
pub mod update_oracle_backup_feed;
pub mod update_perp_parameters;
pub mod update_pricing_admin;
pub mod update_pricing_parameters;
pub mod update_pricing_v2;
pub mod update_pricing_v3;
pub mod update_referrals_admin;
pub mod update_secondary_admin;
pub mod update_take_trigger_order_fee_percentage;
pub mod update_tick_size;
pub mod update_treasury_split_token_account;
pub mod update_trigger_admin;
pub mod update_volatility;
pub mod update_zeta_group_expiry_parameters;
pub mod update_zeta_group_margin_parameters;
pub mod update_zeta_group_perp_parameters;
pub mod update_zeta_pricing_pubkeys;
pub mod update_zeta_state;
pub mod withdraw;
pub mod withdraw_insurance_vault;
pub mod withdraw_insurance_vault_v2;
pub mod withdraw_v2;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum ZetaInstruction {
    InitializeZetaPricing(initialize_zeta_pricing::InitializeZetaPricing),
    UpdateZetaPricingPubkeys(update_zeta_pricing_pubkeys::UpdateZetaPricingPubkeys),
    InitializeZetaGroup(initialize_zeta_group::InitializeZetaGroup),
    OverrideExpiry(override_expiry::OverrideExpiry),
    MigrateToNewCrossMarginAccount(
        migrate_to_new_cross_margin_account::MigrateToNewCrossMarginAccount,
    ),
    MigrateToCrossMarginAccount(migrate_to_cross_margin_account::MigrateToCrossMarginAccount),
    InitializeCrossMarginAccountManager(
        initialize_cross_margin_account_manager::InitializeCrossMarginAccountManager,
    ),
    InitializeCrossMarginAccountManagerV2(
        initialize_cross_margin_account_manager_v2::InitializeCrossMarginAccountManagerV2,
    ),
    InitializeCrossMarginAccount(initialize_cross_margin_account::InitializeCrossMarginAccount),
    InitializeMarginAccount(initialize_margin_account::InitializeMarginAccount),
    InitializeSpreadAccount(initialize_spread_account::InitializeSpreadAccount),
    CloseCrossMarginAccountManager(
        close_cross_margin_account_manager::CloseCrossMarginAccountManager,
    ),
    CloseCrossMarginAccount(close_cross_margin_account::CloseCrossMarginAccount),
    CloseMarginAccount(close_margin_account::CloseMarginAccount),
    CloseSpreadAccount(close_spread_account::CloseSpreadAccount),
    InitializeUnderlying(initialize_underlying::InitializeUnderlying),
    InitializePerpSyncQueue(initialize_perp_sync_queue::InitializePerpSyncQueue),
    InitializeMarketIndexes(initialize_market_indexes::InitializeMarketIndexes),
    InitializeMarketNode(initialize_market_node::InitializeMarketNode),
    Halt(halt::Halt),
    Unhalt(unhalt::Unhalt),
    UpdateHaltState(update_halt_state::UpdateHaltState),
    UpdateVolatility(update_volatility::UpdateVolatility),
    UpdateInterestRate(update_interest_rate::UpdateInterestRate),
    AddPerpMarketIndex(add_perp_market_index::AddPerpMarketIndex),
    AddMarketIndexes(add_market_indexes::AddMarketIndexes),
    InitializeZetaState(initialize_zeta_state::InitializeZetaState),
    InitializeZetaTreasuryWallet(initialize_zeta_treasury_wallet::InitializeZetaTreasuryWallet),
    InitializeZetaReferralsRewardsWallet(
        initialize_zeta_referrals_rewards_wallet::InitializeZetaReferralsRewardsWallet,
    ),
    UpdateAdmin(update_admin::UpdateAdmin),
    UpdateSecondaryAdmin(update_secondary_admin::UpdateSecondaryAdmin),
    UpdateTriggerAdmin(update_trigger_admin::UpdateTriggerAdmin),
    UpdateMaTypeAdmin(update_ma_type_admin::UpdateMaTypeAdmin),
    UpdateReferralsAdmin(update_referrals_admin::UpdateReferralsAdmin),
    UpdatePricingAdmin(update_pricing_admin::UpdatePricingAdmin),
    UpdateTreasurySplitTokenAccount(
        update_treasury_split_token_account::UpdateTreasurySplitTokenAccount,
    ),
    UpdateMakerRebatePercentage(update_maker_rebate_percentage::UpdateMakerRebatePercentage),
    UpdateTakeTriggerOrderFeePercentage(
        update_take_trigger_order_fee_percentage::UpdateTakeTriggerOrderFeePercentage,
    ),
    UpdateZetaState(update_zeta_state::UpdateZetaState),
    UpdateOracle(update_oracle::UpdateOracle),
    UpdateOracleBackupFeed(update_oracle_backup_feed::UpdateOracleBackupFeed),
    UpdatePricingParameters(update_pricing_parameters::UpdatePricingParameters),
    UpdateMarginParameters(update_margin_parameters::UpdateMarginParameters),
    UpdateZetaGroupMarginParameters(
        update_zeta_group_margin_parameters::UpdateZetaGroupMarginParameters,
    ),
    UpdatePerpParameters(update_perp_parameters::UpdatePerpParameters),
    UpdateZetaGroupPerpParameters(update_zeta_group_perp_parameters::UpdateZetaGroupPerpParameters),
    UpdateZetaGroupExpiryParameters(
        update_zeta_group_expiry_parameters::UpdateZetaGroupExpiryParameters,
    ),
    ToggleZetaGroupPerpsOnly(toggle_zeta_group_perps_only::ToggleZetaGroupPerpsOnly),
    CleanZetaMarkets(clean_zeta_markets::CleanZetaMarkets),
    CleanZetaMarketHalted(clean_zeta_market_halted::CleanZetaMarketHalted),
    SettlePositionsHalted(settle_positions_halted::SettlePositionsHalted),
    InitializeMarketStrikes(initialize_market_strikes::InitializeMarketStrikes),
    ExpireSeriesOverride(expire_series_override::ExpireSeriesOverride),
    ExpireSeries(expire_series::ExpireSeries),
    InitializeMarketPda(initialize_market_pda::InitializeMarketPda),
    InitializeZetaSpecificMarketVaults(
        initialize_zeta_specific_market_vaults::InitializeZetaSpecificMarketVaults,
    ),
    InitializeZetaMarket(initialize_zeta_market::InitializeZetaMarket),
    InitializeMarketTifEpochCycle(initialize_market_tif_epoch_cycle::InitializeMarketTifEpochCycle),
    UpdatePricingV2(update_pricing_v2::UpdatePricingV2),
    UpdatePricingV3(update_pricing_v3::UpdatePricingV3),
    ApplyPerpFunding(apply_perp_funding::ApplyPerpFunding),
    Deposit(deposit::Deposit),
    DepositV2(deposit_v2::DepositV2),
    DepositPermissionless(deposit_permissionless::DepositPermissionless),
    DepositInsuranceVault(deposit_insurance_vault::DepositInsuranceVault),
    DepositInsuranceVaultV2(deposit_insurance_vault_v2::DepositInsuranceVaultV2),
    ChooseAirdropCommunity(choose_airdrop_community::ChooseAirdropCommunity),
    Withdraw(withdraw::Withdraw),
    WithdrawV2(withdraw_v2::WithdrawV2),
    WithdrawInsuranceVault(withdraw_insurance_vault::WithdrawInsuranceVault),
    WithdrawInsuranceVaultV2(withdraw_insurance_vault_v2::WithdrawInsuranceVaultV2),
    InitializeOpenOrders(initialize_open_orders::InitializeOpenOrders),
    InitializeOpenOrdersV2(initialize_open_orders_v2::InitializeOpenOrdersV2),
    InitializeOpenOrdersV3(initialize_open_orders_v3::InitializeOpenOrdersV3),
    CloseOpenOrders(close_open_orders::CloseOpenOrders),
    CloseOpenOrdersV2(close_open_orders_v2::CloseOpenOrdersV2),
    CloseOpenOrdersV3(close_open_orders_v3::CloseOpenOrdersV3),
    CloseOpenOrdersV4(close_open_orders_v4::CloseOpenOrdersV4),
    AdminResetDexOpenOrders(admin_reset_dex_open_orders::AdminResetDexOpenOrders),
    InitializeWhitelistDepositAccount(
        initialize_whitelist_deposit_account::InitializeWhitelistDepositAccount,
    ),
    InitializeWhitelistInsuranceAccount(
        initialize_whitelist_insurance_account::InitializeWhitelistInsuranceAccount,
    ),
    InitializeWhitelistTradingFeesAccount(
        initialize_whitelist_trading_fees_account::InitializeWhitelistTradingFeesAccount,
    ),
    InitializeInsuranceDepositAccount(
        initialize_insurance_deposit_account::InitializeInsuranceDepositAccount,
    ),
    InitializeCombinedInsuranceVault(
        initialize_combined_insurance_vault::InitializeCombinedInsuranceVault,
    ),
    InitializeCombinedVault(initialize_combined_vault::InitializeCombinedVault),
    InitializeCombinedSocializedLossAccount(
        initialize_combined_socialized_loss_account::InitializeCombinedSocializedLossAccount,
    ),
    PlaceOrder(place_order::PlaceOrder),
    PlaceOrderV2(place_order_v2::PlaceOrderV2),
    PlaceOrderV3(place_order_v3::PlaceOrderV3),
    PlacePerpOrder(place_perp_order::PlacePerpOrder),
    PlacePerpOrderV2(place_perp_order_v2::PlacePerpOrderV2),
    PlaceOrderV4(place_order_v4::PlaceOrderV4),
    PlacePerpOrderV3(place_perp_order_v3::PlacePerpOrderV3),
    PlacePerpOrderV4(place_perp_order_v4::PlacePerpOrderV4),
    PlacePerpOrderV5(place_perp_order_v5::PlacePerpOrderV5),
    PlaceMultiOrders(place_multi_orders::PlaceMultiOrders),
    PlaceTriggerOrder(place_trigger_order::PlaceTriggerOrder),
    ExecuteTriggerOrderV2(execute_trigger_order_v2::ExecuteTriggerOrderV2),
    TakeTriggerOrder(take_trigger_order::TakeTriggerOrder),
    ExecuteTriggerOrder(execute_trigger_order::ExecuteTriggerOrder),
    ForceCancelTriggerOrder(force_cancel_trigger_order::ForceCancelTriggerOrder),
    CancelTriggerOrderV2(cancel_trigger_order_v2::CancelTriggerOrderV2),
    CancelTriggerOrder(cancel_trigger_order::CancelTriggerOrder),
    UpdateMinLot(update_min_lot::UpdateMinLot),
    UpdateTickSize(update_tick_size::UpdateTickSize),
    InitializeMinLotsAndTickSizes(
        initialize_min_lots_and_tick_sizes::InitializeMinLotsAndTickSizes,
    ),
    EditTriggerOrder(edit_trigger_order::EditTriggerOrder),
    EditTriggerOrderV2(edit_trigger_order_v2::EditTriggerOrderV2),
    CancelOrder(cancel_order::CancelOrder),
    CancelOrderNoError(cancel_order_no_error::CancelOrderNoError),
    CancelAllMarketOrders(cancel_all_market_orders::CancelAllMarketOrders),
    CancelOrderHalted(cancel_order_halted::CancelOrderHalted),
    CancelOrderByClientOrderId(cancel_order_by_client_order_id::CancelOrderByClientOrderId),
    CancelOrderByClientOrderIdNoError(
        cancel_order_by_client_order_id_no_error::CancelOrderByClientOrderIdNoError,
    ),
    PruneExpiredTifOrders(prune_expired_tif_orders::PruneExpiredTifOrders),
    PruneExpiredTifOrdersV2(prune_expired_tif_orders_v2::PruneExpiredTifOrdersV2),
    ForceCancelOrderByOrderIdV2(force_cancel_order_by_order_id_v2::ForceCancelOrderByOrderIdV2),
    ForceCancelOrderByOrderId(force_cancel_order_by_order_id::ForceCancelOrderByOrderId),
    AdminSetOrderState(admin_set_order_state::AdminSetOrderState),
    AdminForceCancelOrders(admin_force_cancel_orders::AdminForceCancelOrders),
    ForceCancelOrdersV2(force_cancel_orders_v2::ForceCancelOrdersV2),
    ForceCancelOrders(force_cancel_orders::ForceCancelOrders),
    AdminCrankEventQueue(admin_crank_event_queue::AdminCrankEventQueue),
    CrankEventQueue(crank_event_queue::CrankEventQueue),
    CollectTreasuryFunds(collect_treasury_funds::CollectTreasuryFunds),
    TreasuryMovement(treasury_movement::TreasuryMovement),
    RebalanceInsuranceVault(rebalance_insurance_vault::RebalanceInsuranceVault),
    RebalanceInsuranceVaultV2(rebalance_insurance_vault_v2::RebalanceInsuranceVaultV2),
    LiquidateV2(liquidate_v2::LiquidateV2),
    Liquidate(liquidate::Liquidate),
    BurnVaultTokens(burn_vault_tokens::BurnVaultTokens),
    SettleDexFunds(settle_dex_funds::SettleDexFunds),
    PositionMovement(position_movement::PositionMovement),
    TransferExcessSpreadBalance(transfer_excess_spread_balance::TransferExcessSpreadBalance),
    ToggleMarketMaker(toggle_market_maker::ToggleMarketMaker),
    InitializeReferrerAccounts(initialize_referrer_accounts::InitializeReferrerAccounts),
    CloseReferrerAccounts(close_referrer_accounts::CloseReferrerAccounts),
    EditMaType(edit_ma_type::EditMaType),
    EditDelegatedPubkey(edit_delegated_pubkey::EditDelegatedPubkey),
    ResetNumFlexUnderlyings(reset_num_flex_underlyings::ResetNumFlexUnderlyings),
    TradeEvent(trade_event::TradeEvent),
    TradeEventV2Event(trade_event_v2_event::TradeEventV2Event),
    TradeEventV3Event(trade_event_v3_event::TradeEventV3Event),
    PositionMovementEvent(position_movement_event::PositionMovementEvent),
    PlaceOrderEvent(place_order_event::PlaceOrderEvent),
    LiquidationEvent(liquidation_event::LiquidationEvent),
    OrderCompleteEvent(order_complete_event::OrderCompleteEvent),
    ApplyFundingEvent(apply_funding_event::ApplyFundingEvent),
    PlaceMultiOrdersEvent(place_multi_orders_event::PlaceMultiOrdersEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for ZetaDecoder {
    type InstructionType = ZetaInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            ZetaInstruction::InitializeZetaPricing => initialize_zeta_pricing::InitializeZetaPricing,
            ZetaInstruction::UpdateZetaPricingPubkeys => update_zeta_pricing_pubkeys::UpdateZetaPricingPubkeys,
            ZetaInstruction::InitializeZetaGroup => initialize_zeta_group::InitializeZetaGroup,
            ZetaInstruction::OverrideExpiry => override_expiry::OverrideExpiry,
            ZetaInstruction::MigrateToNewCrossMarginAccount => migrate_to_new_cross_margin_account::MigrateToNewCrossMarginAccount,
            ZetaInstruction::MigrateToCrossMarginAccount => migrate_to_cross_margin_account::MigrateToCrossMarginAccount,
            ZetaInstruction::InitializeCrossMarginAccountManager => initialize_cross_margin_account_manager::InitializeCrossMarginAccountManager,
            ZetaInstruction::InitializeCrossMarginAccountManagerV2 => initialize_cross_margin_account_manager_v2::InitializeCrossMarginAccountManagerV2,
            ZetaInstruction::InitializeCrossMarginAccount => initialize_cross_margin_account::InitializeCrossMarginAccount,
            ZetaInstruction::InitializeMarginAccount => initialize_margin_account::InitializeMarginAccount,
            ZetaInstruction::InitializeSpreadAccount => initialize_spread_account::InitializeSpreadAccount,
            ZetaInstruction::CloseCrossMarginAccountManager => close_cross_margin_account_manager::CloseCrossMarginAccountManager,
            ZetaInstruction::CloseCrossMarginAccount => close_cross_margin_account::CloseCrossMarginAccount,
            ZetaInstruction::CloseMarginAccount => close_margin_account::CloseMarginAccount,
            ZetaInstruction::CloseSpreadAccount => close_spread_account::CloseSpreadAccount,
            ZetaInstruction::InitializeUnderlying => initialize_underlying::InitializeUnderlying,
            ZetaInstruction::InitializePerpSyncQueue => initialize_perp_sync_queue::InitializePerpSyncQueue,
            ZetaInstruction::InitializeMarketIndexes => initialize_market_indexes::InitializeMarketIndexes,
            ZetaInstruction::InitializeMarketNode => initialize_market_node::InitializeMarketNode,
            ZetaInstruction::Halt => halt::Halt,
            ZetaInstruction::Unhalt => unhalt::Unhalt,
            ZetaInstruction::UpdateHaltState => update_halt_state::UpdateHaltState,
            ZetaInstruction::UpdateVolatility => update_volatility::UpdateVolatility,
            ZetaInstruction::UpdateInterestRate => update_interest_rate::UpdateInterestRate,
            ZetaInstruction::AddPerpMarketIndex => add_perp_market_index::AddPerpMarketIndex,
            ZetaInstruction::AddMarketIndexes => add_market_indexes::AddMarketIndexes,
            ZetaInstruction::InitializeZetaState => initialize_zeta_state::InitializeZetaState,
            ZetaInstruction::InitializeZetaTreasuryWallet => initialize_zeta_treasury_wallet::InitializeZetaTreasuryWallet,
            ZetaInstruction::InitializeZetaReferralsRewardsWallet => initialize_zeta_referrals_rewards_wallet::InitializeZetaReferralsRewardsWallet,
            ZetaInstruction::UpdateAdmin => update_admin::UpdateAdmin,
            ZetaInstruction::UpdateSecondaryAdmin => update_secondary_admin::UpdateSecondaryAdmin,
            ZetaInstruction::UpdateTriggerAdmin => update_trigger_admin::UpdateTriggerAdmin,
            ZetaInstruction::UpdateMaTypeAdmin => update_ma_type_admin::UpdateMaTypeAdmin,
            ZetaInstruction::UpdateReferralsAdmin => update_referrals_admin::UpdateReferralsAdmin,
            ZetaInstruction::UpdatePricingAdmin => update_pricing_admin::UpdatePricingAdmin,
            ZetaInstruction::UpdateTreasurySplitTokenAccount => update_treasury_split_token_account::UpdateTreasurySplitTokenAccount,
            ZetaInstruction::UpdateMakerRebatePercentage => update_maker_rebate_percentage::UpdateMakerRebatePercentage,
            ZetaInstruction::UpdateTakeTriggerOrderFeePercentage => update_take_trigger_order_fee_percentage::UpdateTakeTriggerOrderFeePercentage,
            ZetaInstruction::UpdateZetaState => update_zeta_state::UpdateZetaState,
            ZetaInstruction::UpdateOracle => update_oracle::UpdateOracle,
            ZetaInstruction::UpdateOracleBackupFeed => update_oracle_backup_feed::UpdateOracleBackupFeed,
            ZetaInstruction::UpdatePricingParameters => update_pricing_parameters::UpdatePricingParameters,
            ZetaInstruction::UpdateMarginParameters => update_margin_parameters::UpdateMarginParameters,
            ZetaInstruction::UpdateZetaGroupMarginParameters => update_zeta_group_margin_parameters::UpdateZetaGroupMarginParameters,
            ZetaInstruction::UpdatePerpParameters => update_perp_parameters::UpdatePerpParameters,
            ZetaInstruction::UpdateZetaGroupPerpParameters => update_zeta_group_perp_parameters::UpdateZetaGroupPerpParameters,
            ZetaInstruction::UpdateZetaGroupExpiryParameters => update_zeta_group_expiry_parameters::UpdateZetaGroupExpiryParameters,
            ZetaInstruction::ToggleZetaGroupPerpsOnly => toggle_zeta_group_perps_only::ToggleZetaGroupPerpsOnly,
            ZetaInstruction::CleanZetaMarkets => clean_zeta_markets::CleanZetaMarkets,
            ZetaInstruction::CleanZetaMarketHalted => clean_zeta_market_halted::CleanZetaMarketHalted,
            ZetaInstruction::SettlePositionsHalted => settle_positions_halted::SettlePositionsHalted,
            ZetaInstruction::InitializeMarketStrikes => initialize_market_strikes::InitializeMarketStrikes,
            ZetaInstruction::ExpireSeriesOverride => expire_series_override::ExpireSeriesOverride,
            ZetaInstruction::ExpireSeries => expire_series::ExpireSeries,
            ZetaInstruction::InitializeMarketPda => initialize_market_pda::InitializeMarketPda,
            ZetaInstruction::InitializeZetaSpecificMarketVaults => initialize_zeta_specific_market_vaults::InitializeZetaSpecificMarketVaults,
            ZetaInstruction::InitializeZetaMarket => initialize_zeta_market::InitializeZetaMarket,
            ZetaInstruction::InitializeMarketTifEpochCycle => initialize_market_tif_epoch_cycle::InitializeMarketTifEpochCycle,
            ZetaInstruction::UpdatePricingV2 => update_pricing_v2::UpdatePricingV2,
            ZetaInstruction::UpdatePricingV3 => update_pricing_v3::UpdatePricingV3,
            ZetaInstruction::ApplyPerpFunding => apply_perp_funding::ApplyPerpFunding,
            ZetaInstruction::Deposit => deposit::Deposit,
            ZetaInstruction::DepositV2 => deposit_v2::DepositV2,
            ZetaInstruction::DepositPermissionless => deposit_permissionless::DepositPermissionless,
            ZetaInstruction::DepositInsuranceVault => deposit_insurance_vault::DepositInsuranceVault,
            ZetaInstruction::DepositInsuranceVaultV2 => deposit_insurance_vault_v2::DepositInsuranceVaultV2,
            ZetaInstruction::ChooseAirdropCommunity => choose_airdrop_community::ChooseAirdropCommunity,
            ZetaInstruction::Withdraw => withdraw::Withdraw,
            ZetaInstruction::WithdrawV2 => withdraw_v2::WithdrawV2,
            ZetaInstruction::WithdrawInsuranceVault => withdraw_insurance_vault::WithdrawInsuranceVault,
            ZetaInstruction::WithdrawInsuranceVaultV2 => withdraw_insurance_vault_v2::WithdrawInsuranceVaultV2,
            ZetaInstruction::InitializeOpenOrders => initialize_open_orders::InitializeOpenOrders,
            ZetaInstruction::InitializeOpenOrdersV2 => initialize_open_orders_v2::InitializeOpenOrdersV2,
            ZetaInstruction::InitializeOpenOrdersV3 => initialize_open_orders_v3::InitializeOpenOrdersV3,
            ZetaInstruction::CloseOpenOrders => close_open_orders::CloseOpenOrders,
            ZetaInstruction::CloseOpenOrdersV2 => close_open_orders_v2::CloseOpenOrdersV2,
            ZetaInstruction::CloseOpenOrdersV3 => close_open_orders_v3::CloseOpenOrdersV3,
            ZetaInstruction::CloseOpenOrdersV4 => close_open_orders_v4::CloseOpenOrdersV4,
            ZetaInstruction::AdminResetDexOpenOrders => admin_reset_dex_open_orders::AdminResetDexOpenOrders,
            ZetaInstruction::InitializeWhitelistDepositAccount => initialize_whitelist_deposit_account::InitializeWhitelistDepositAccount,
            ZetaInstruction::InitializeWhitelistInsuranceAccount => initialize_whitelist_insurance_account::InitializeWhitelistInsuranceAccount,
            ZetaInstruction::InitializeWhitelistTradingFeesAccount => initialize_whitelist_trading_fees_account::InitializeWhitelistTradingFeesAccount,
            ZetaInstruction::InitializeInsuranceDepositAccount => initialize_insurance_deposit_account::InitializeInsuranceDepositAccount,
            ZetaInstruction::InitializeCombinedInsuranceVault => initialize_combined_insurance_vault::InitializeCombinedInsuranceVault,
            ZetaInstruction::InitializeCombinedVault => initialize_combined_vault::InitializeCombinedVault,
            ZetaInstruction::InitializeCombinedSocializedLossAccount => initialize_combined_socialized_loss_account::InitializeCombinedSocializedLossAccount,
            ZetaInstruction::PlaceOrder => place_order::PlaceOrder,
            ZetaInstruction::PlaceOrderV2 => place_order_v2::PlaceOrderV2,
            ZetaInstruction::PlaceOrderV3 => place_order_v3::PlaceOrderV3,
            ZetaInstruction::PlacePerpOrder => place_perp_order::PlacePerpOrder,
            ZetaInstruction::PlacePerpOrderV2 => place_perp_order_v2::PlacePerpOrderV2,
            ZetaInstruction::PlaceOrderV4 => place_order_v4::PlaceOrderV4,
            ZetaInstruction::PlacePerpOrderV3 => place_perp_order_v3::PlacePerpOrderV3,
            ZetaInstruction::PlacePerpOrderV4 => place_perp_order_v4::PlacePerpOrderV4,
            ZetaInstruction::PlacePerpOrderV5 => place_perp_order_v5::PlacePerpOrderV5,
            ZetaInstruction::PlaceMultiOrders => place_multi_orders::PlaceMultiOrders,
            ZetaInstruction::PlaceTriggerOrder => place_trigger_order::PlaceTriggerOrder,
            ZetaInstruction::ExecuteTriggerOrderV2 => execute_trigger_order_v2::ExecuteTriggerOrderV2,
            ZetaInstruction::TakeTriggerOrder => take_trigger_order::TakeTriggerOrder,
            ZetaInstruction::ExecuteTriggerOrder => execute_trigger_order::ExecuteTriggerOrder,
            ZetaInstruction::ForceCancelTriggerOrder => force_cancel_trigger_order::ForceCancelTriggerOrder,
            ZetaInstruction::CancelTriggerOrderV2 => cancel_trigger_order_v2::CancelTriggerOrderV2,
            ZetaInstruction::CancelTriggerOrder => cancel_trigger_order::CancelTriggerOrder,
            ZetaInstruction::UpdateMinLot => update_min_lot::UpdateMinLot,
            ZetaInstruction::UpdateTickSize => update_tick_size::UpdateTickSize,
            ZetaInstruction::InitializeMinLotsAndTickSizes => initialize_min_lots_and_tick_sizes::InitializeMinLotsAndTickSizes,
            ZetaInstruction::EditTriggerOrder => edit_trigger_order::EditTriggerOrder,
            ZetaInstruction::EditTriggerOrderV2 => edit_trigger_order_v2::EditTriggerOrderV2,
            ZetaInstruction::CancelOrder => cancel_order::CancelOrder,
            ZetaInstruction::CancelOrderNoError => cancel_order_no_error::CancelOrderNoError,
            ZetaInstruction::CancelAllMarketOrders => cancel_all_market_orders::CancelAllMarketOrders,
            ZetaInstruction::CancelOrderHalted => cancel_order_halted::CancelOrderHalted,
            ZetaInstruction::CancelOrderByClientOrderId => cancel_order_by_client_order_id::CancelOrderByClientOrderId,
            ZetaInstruction::CancelOrderByClientOrderIdNoError => cancel_order_by_client_order_id_no_error::CancelOrderByClientOrderIdNoError,
            ZetaInstruction::PruneExpiredTifOrders => prune_expired_tif_orders::PruneExpiredTifOrders,
            ZetaInstruction::PruneExpiredTifOrdersV2 => prune_expired_tif_orders_v2::PruneExpiredTifOrdersV2,
            ZetaInstruction::ForceCancelOrderByOrderIdV2 => force_cancel_order_by_order_id_v2::ForceCancelOrderByOrderIdV2,
            ZetaInstruction::ForceCancelOrderByOrderId => force_cancel_order_by_order_id::ForceCancelOrderByOrderId,
            ZetaInstruction::AdminSetOrderState => admin_set_order_state::AdminSetOrderState,
            ZetaInstruction::AdminForceCancelOrders => admin_force_cancel_orders::AdminForceCancelOrders,
            ZetaInstruction::ForceCancelOrdersV2 => force_cancel_orders_v2::ForceCancelOrdersV2,
            ZetaInstruction::ForceCancelOrders => force_cancel_orders::ForceCancelOrders,
            ZetaInstruction::AdminCrankEventQueue => admin_crank_event_queue::AdminCrankEventQueue,
            ZetaInstruction::CrankEventQueue => crank_event_queue::CrankEventQueue,
            ZetaInstruction::CollectTreasuryFunds => collect_treasury_funds::CollectTreasuryFunds,
            ZetaInstruction::TreasuryMovement => treasury_movement::TreasuryMovement,
            ZetaInstruction::RebalanceInsuranceVault => rebalance_insurance_vault::RebalanceInsuranceVault,
            ZetaInstruction::RebalanceInsuranceVaultV2 => rebalance_insurance_vault_v2::RebalanceInsuranceVaultV2,
            ZetaInstruction::LiquidateV2 => liquidate_v2::LiquidateV2,
            ZetaInstruction::Liquidate => liquidate::Liquidate,
            ZetaInstruction::BurnVaultTokens => burn_vault_tokens::BurnVaultTokens,
            ZetaInstruction::SettleDexFunds => settle_dex_funds::SettleDexFunds,
            ZetaInstruction::PositionMovement => position_movement::PositionMovement,
            ZetaInstruction::TransferExcessSpreadBalance => transfer_excess_spread_balance::TransferExcessSpreadBalance,
            ZetaInstruction::ToggleMarketMaker => toggle_market_maker::ToggleMarketMaker,
            ZetaInstruction::InitializeReferrerAccounts => initialize_referrer_accounts::InitializeReferrerAccounts,
            ZetaInstruction::CloseReferrerAccounts => close_referrer_accounts::CloseReferrerAccounts,
            ZetaInstruction::EditMaType => edit_ma_type::EditMaType,
            ZetaInstruction::EditDelegatedPubkey => edit_delegated_pubkey::EditDelegatedPubkey,
            ZetaInstruction::ResetNumFlexUnderlyings => reset_num_flex_underlyings::ResetNumFlexUnderlyings,
            ZetaInstruction::TradeEvent => trade_event::TradeEvent,
            ZetaInstruction::TradeEventV2Event => trade_event_v2_event::TradeEventV2Event,
            ZetaInstruction::TradeEventV3Event => trade_event_v3_event::TradeEventV3Event,
            ZetaInstruction::PositionMovementEvent => position_movement_event::PositionMovementEvent,
            ZetaInstruction::PlaceOrderEvent => place_order_event::PlaceOrderEvent,
            ZetaInstruction::LiquidationEvent => liquidation_event::LiquidationEvent,
            ZetaInstruction::OrderCompleteEvent => order_complete_event::OrderCompleteEvent,
            ZetaInstruction::ApplyFundingEvent => apply_funding_event::ApplyFundingEvent,
            ZetaInstruction::PlaceMultiOrdersEvent => place_multi_orders_event::PlaceMultiOrdersEvent,
        )
    }
}
