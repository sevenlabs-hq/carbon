use {super::DriftDecoder, crate::PROGRAM_ID};
pub mod add_insurance_fund_stake;
pub mod add_perp_lp_shares;
pub mod admin_disable_update_perp_bid_ask_twap;
pub mod begin_swap;
pub mod cancel_order;
pub mod cancel_order_by_user_id;
pub mod cancel_orders;
pub mod cancel_orders_by_ids;
pub mod cancel_request_remove_insurance_fund_stake;
pub mod curve_record_event;
pub mod delete_initialized_perp_market;
pub mod delete_initialized_spot_market;
pub mod delete_prelaunch_oracle;
pub mod delete_signed_msg_user_orders;
pub mod delete_user;
pub mod delete_user_record_event;
pub mod deposit;
pub mod deposit_into_perp_market_fee_pool;
pub mod deposit_into_spot_market_revenue_pool;
pub mod deposit_into_spot_market_vault;
pub mod deposit_record_event;
pub mod disable_user_high_leverage_mode;
pub mod enable_user_high_leverage_mode;
pub mod end_swap;
pub mod fill_perp_order;
pub mod fill_spot_order;
pub mod force_cancel_orders;
pub mod force_delete_user;
pub mod fuel_season_record_event;
pub mod fuel_sweep_record_event;
pub mod funding_payment_record_event;
pub mod funding_rate_record_event;
pub mod init_user_fuel;
pub mod initialize;
pub mod initialize_fuel_overflow;
pub mod initialize_high_leverage_mode_config;
pub mod initialize_insurance_fund_stake;
pub mod initialize_openbook_v2_fulfillment_config;
pub mod initialize_perp_market;
pub mod initialize_phoenix_fulfillment_config;
pub mod initialize_prediction_market;
pub mod initialize_prelaunch_oracle;
pub mod initialize_protected_maker_mode_config;
pub mod initialize_protocol_if_shares_transfer_config;
pub mod initialize_pyth_lazer_oracle;
pub mod initialize_pyth_pull_oracle;
pub mod initialize_referrer_name;
pub mod initialize_serum_fulfillment_config;
pub mod initialize_signed_msg_user_orders;
pub mod initialize_spot_market;
pub mod initialize_user;
pub mod initialize_user_stats;
pub mod insurance_fund_record_event;
pub mod insurance_fund_stake_record_event;
pub mod liquidate_borrow_for_perp_pnl;
pub mod liquidate_perp;
pub mod liquidate_perp_pnl_for_deposit;
pub mod liquidate_perp_with_fill;
pub mod liquidate_spot;
pub mod liquidate_spot_with_swap_begin;
pub mod liquidate_spot_with_swap_end;
pub mod liquidation_record_event;
pub mod log_user_balances;
pub mod lp_record_event;
pub mod modify_order;
pub mod modify_order_by_user_id;
pub mod move_amm_price;
pub mod new_user_record_event;
pub mod openbook_v2_fulfillment_config_status;
pub mod order_action_record_event;
pub mod order_record_event;
pub mod pause_spot_market_deposit_withdraw;
pub mod phoenix_fulfillment_config_status;
pub mod place_and_make_perp_order;
pub mod place_and_make_signed_msg_perp_order;
pub mod place_and_make_spot_order;
pub mod place_and_take_perp_order;
pub mod place_and_take_spot_order;
pub mod place_orders;
pub mod place_perp_order;
pub mod place_signed_msg_taker_order;
pub mod place_spot_order;
pub mod post_multi_pyth_pull_oracle_updates_atomic;
pub mod post_pyth_lazer_oracle_update;
pub mod post_pyth_pull_oracle_update_atomic;
pub mod recenter_perp_market_amm;
pub mod reclaim_rent;
pub mod remove_insurance_fund_stake;
pub mod remove_perp_lp_shares;
pub mod remove_perp_lp_shares_in_expiring_market;
pub mod repeg_amm_curve;
pub mod request_remove_insurance_fund_stake;
pub mod reset_fuel_season;
pub mod reset_perp_market_amm_oracle_twap;
pub mod resize_signed_msg_user_orders;
pub mod resolve_perp_bankruptcy;
pub mod resolve_perp_pnl_deficit;
pub mod resolve_spot_bankruptcy;
pub mod revert_fill;
pub mod set_user_status_to_being_liquidated;
pub mod settle_expired_market;
pub mod settle_expired_market_pools_to_revenue_pool;
pub mod settle_funding_payment;
pub mod settle_lp;
pub mod settle_multiple_pnls;
pub mod settle_pnl;
pub mod settle_pnl_record_event;
pub mod settle_revenue_to_insurance_fund;
pub mod signed_msg_order_record_event;
pub mod spot_interest_record_event;
pub mod spot_market_vault_deposit_record_event;
pub mod swap_record_event;
pub mod sweep_fuel;
pub mod transfer_deposit;
pub mod transfer_pools;
pub mod transfer_protocol_if_shares;
pub mod trigger_order;
pub mod update_admin;
pub mod update_amm_jit_intensity;
pub mod update_amms;
pub mod update_discount_mint;
pub mod update_exchange_status;
pub mod update_funding_rate;
pub mod update_high_leverage_mode_config;
pub mod update_initial_pct_to_liquidate;
pub mod update_insurance_fund_unstaking_period;
pub mod update_k;
pub mod update_liquidation_duration;
pub mod update_liquidation_margin_buffer_ratio;
pub mod update_lp_cooldown_time;
pub mod update_oracle_guard_rails;
pub mod update_perp_auction_duration;
pub mod update_perp_bid_ask_twap;
pub mod update_perp_fee_structure;
pub mod update_perp_market_amm_oracle_twap;
pub mod update_perp_market_amm_summary_stats;
pub mod update_perp_market_base_spread;
pub mod update_perp_market_concentration_coef;
pub mod update_perp_market_contract_tier;
pub mod update_perp_market_curve_update_intensity;
pub mod update_perp_market_expiry;
pub mod update_perp_market_fee_adjustment;
pub mod update_perp_market_fuel;
pub mod update_perp_market_funding_period;
pub mod update_perp_market_high_leverage_margin_ratio;
pub mod update_perp_market_imf_factor;
pub mod update_perp_market_liquidation_fee;
pub mod update_perp_market_margin_ratio;
pub mod update_perp_market_max_fill_reserve_fraction;
pub mod update_perp_market_max_imbalances;
pub mod update_perp_market_max_open_interest;
pub mod update_perp_market_max_slippage_ratio;
pub mod update_perp_market_max_spread;
pub mod update_perp_market_min_order_size;
pub mod update_perp_market_name;
pub mod update_perp_market_number_of_users;
pub mod update_perp_market_oracle;
pub mod update_perp_market_paused_operations;
pub mod update_perp_market_per_lp_base;
pub mod update_perp_market_status;
pub mod update_perp_market_step_size_and_tick_size;
pub mod update_perp_market_target_base_asset_amount_per_lp;
pub mod update_perp_market_unrealized_asset_weight;
pub mod update_prelaunch_oracle;
pub mod update_prelaunch_oracle_params;
pub mod update_protected_maker_mode_config;
pub mod update_protocol_if_shares_transfer_config;
pub mod update_pyth_pull_oracle;
pub mod update_serum_fulfillment_config_status;
pub mod update_serum_vault;
pub mod update_spot_auction_duration;
pub mod update_spot_fee_structure;
pub mod update_spot_market_asset_tier;
pub mod update_spot_market_borrow_rate;
pub mod update_spot_market_cumulative_interest;
pub mod update_spot_market_expiry;
pub mod update_spot_market_fee_adjustment;
pub mod update_spot_market_fuel;
pub mod update_spot_market_if_factor;
pub mod update_spot_market_if_paused_operations;
pub mod update_spot_market_liquidation_fee;
pub mod update_spot_market_margin_weights;
pub mod update_spot_market_max_token_borrows;
pub mod update_spot_market_max_token_deposits;
pub mod update_spot_market_min_order_size;
pub mod update_spot_market_name;
pub mod update_spot_market_oracle;
pub mod update_spot_market_orders_enabled;
pub mod update_spot_market_paused_operations;
pub mod update_spot_market_pool_id;
pub mod update_spot_market_revenue_settle_period;
pub mod update_spot_market_scale_initial_asset_weight_start;
pub mod update_spot_market_status;
pub mod update_spot_market_step_size_and_tick_size;
pub mod update_state_max_initialize_user_fee;
pub mod update_state_max_number_of_sub_accounts;
pub mod update_state_settlement_duration;
pub mod update_user_advanced_lp;
pub mod update_user_custom_margin_ratio;
pub mod update_user_delegate;
pub mod update_user_fuel_bonus;
pub mod update_user_gov_token_insurance_stake;
pub mod update_user_gov_token_insurance_stake_devnet;
pub mod update_user_idle;
pub mod update_user_margin_trading_enabled;
pub mod update_user_name;
pub mod update_user_open_orders_count;
pub mod update_user_pool_id;
pub mod update_user_protected_maker_orders;
pub mod update_user_quote_asset_insurance_stake;
pub mod update_user_reduce_only;
pub mod update_user_stats_referrer_status;
pub mod update_whitelist_mint;
pub mod update_withdraw_guard_threshold;
pub mod withdraw;

#[allow(clippy::large_enum_variant)]
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
pub enum DriftInstruction {
    InitializeUser(initialize_user::InitializeUser),
    InitializeUserStats(initialize_user_stats::InitializeUserStats),
    InitializeSignedMsgUserOrders(initialize_signed_msg_user_orders::InitializeSignedMsgUserOrders),
    ResizeSignedMsgUserOrders(resize_signed_msg_user_orders::ResizeSignedMsgUserOrders),
    InitializeFuelOverflow(initialize_fuel_overflow::InitializeFuelOverflow),
    SweepFuel(sweep_fuel::SweepFuel),
    ResetFuelSeason(reset_fuel_season::ResetFuelSeason),
    InitializeReferrerName(initialize_referrer_name::InitializeReferrerName),
    Deposit(deposit::Deposit),
    Withdraw(withdraw::Withdraw),
    TransferDeposit(transfer_deposit::TransferDeposit),
    TransferPools(transfer_pools::TransferPools),
    PlacePerpOrder(place_perp_order::PlacePerpOrder),
    CancelOrder(cancel_order::CancelOrder),
    CancelOrderByUserId(cancel_order_by_user_id::CancelOrderByUserId),
    CancelOrders(cancel_orders::CancelOrders),
    CancelOrdersByIds(cancel_orders_by_ids::CancelOrdersByIds),
    ModifyOrder(modify_order::ModifyOrder),
    ModifyOrderByUserId(modify_order_by_user_id::ModifyOrderByUserId),
    PlaceAndTakePerpOrder(place_and_take_perp_order::PlaceAndTakePerpOrder),
    PlaceAndMakePerpOrder(place_and_make_perp_order::PlaceAndMakePerpOrder),
    PlaceAndMakeSignedMsgPerpOrder(place_and_make_signed_msg_perp_order::PlaceAndMakeSignedMsgPerpOrder),
    PlaceSignedMsgTakerOrder(place_signed_msg_taker_order::PlaceSignedMsgTakerOrder),
    PlaceSpotOrder(place_spot_order::PlaceSpotOrder),
    PlaceAndTakeSpotOrder(place_and_take_spot_order::PlaceAndTakeSpotOrder),
    PlaceAndMakeSpotOrder(place_and_make_spot_order::PlaceAndMakeSpotOrder),
    PlaceOrders(place_orders::PlaceOrders),
    BeginSwap(begin_swap::BeginSwap),
    EndSwap(end_swap::EndSwap),
    AddPerpLpShares(add_perp_lp_shares::AddPerpLpShares),
    RemovePerpLpShares(remove_perp_lp_shares::RemovePerpLpShares),
    RemovePerpLpSharesInExpiringMarket(remove_perp_lp_shares_in_expiring_market::RemovePerpLpSharesInExpiringMarket),
    UpdateUserName(update_user_name::UpdateUserName),
    UpdateUserCustomMarginRatio(update_user_custom_margin_ratio::UpdateUserCustomMarginRatio),
    UpdateUserMarginTradingEnabled(update_user_margin_trading_enabled::UpdateUserMarginTradingEnabled),
    UpdateUserPoolId(update_user_pool_id::UpdateUserPoolId),
    UpdateUserDelegate(update_user_delegate::UpdateUserDelegate),
    UpdateUserReduceOnly(update_user_reduce_only::UpdateUserReduceOnly),
    UpdateUserAdvancedLp(update_user_advanced_lp::UpdateUserAdvancedLp),
    UpdateUserProtectedMakerOrders(update_user_protected_maker_orders::UpdateUserProtectedMakerOrders),
    DeleteUser(delete_user::DeleteUser),
    ForceDeleteUser(force_delete_user::ForceDeleteUser),
    DeleteSignedMsgUserOrders(delete_signed_msg_user_orders::DeleteSignedMsgUserOrders),
    ReclaimRent(reclaim_rent::ReclaimRent),
    EnableUserHighLeverageMode(enable_user_high_leverage_mode::EnableUserHighLeverageMode),
    FillPerpOrder(fill_perp_order::FillPerpOrder),
    RevertFill(revert_fill::RevertFill),
    FillSpotOrder(fill_spot_order::FillSpotOrder),
    TriggerOrder(trigger_order::TriggerOrder),
    ForceCancelOrders(force_cancel_orders::ForceCancelOrders),
    UpdateUserIdle(update_user_idle::UpdateUserIdle),
    LogUserBalances(log_user_balances::LogUserBalances),
    DisableUserHighLeverageMode(disable_user_high_leverage_mode::DisableUserHighLeverageMode),
    UpdateUserFuelBonus(update_user_fuel_bonus::UpdateUserFuelBonus),
    UpdateUserStatsReferrerStatus(update_user_stats_referrer_status::UpdateUserStatsReferrerStatus),
    UpdateUserOpenOrdersCount(update_user_open_orders_count::UpdateUserOpenOrdersCount),
    AdminDisableUpdatePerpBidAskTwap(admin_disable_update_perp_bid_ask_twap::AdminDisableUpdatePerpBidAskTwap),
    SettlePnl(settle_pnl::SettlePnl),
    SettleMultiplePnls(settle_multiple_pnls::SettleMultiplePnls),
    SettleFundingPayment(settle_funding_payment::SettleFundingPayment),
    SettleLp(settle_lp::SettleLp),
    SettleExpiredMarket(settle_expired_market::SettleExpiredMarket),
    LiquidatePerp(liquidate_perp::LiquidatePerp),
    LiquidatePerpWithFill(liquidate_perp_with_fill::LiquidatePerpWithFill),
    LiquidateSpot(liquidate_spot::LiquidateSpot),
    LiquidateSpotWithSwapBegin(liquidate_spot_with_swap_begin::LiquidateSpotWithSwapBegin),
    LiquidateSpotWithSwapEnd(liquidate_spot_with_swap_end::LiquidateSpotWithSwapEnd),
    LiquidateBorrowForPerpPnl(liquidate_borrow_for_perp_pnl::LiquidateBorrowForPerpPnl),
    LiquidatePerpPnlForDeposit(liquidate_perp_pnl_for_deposit::LiquidatePerpPnlForDeposit),
    SetUserStatusToBeingLiquidated(set_user_status_to_being_liquidated::SetUserStatusToBeingLiquidated),
    ResolvePerpPnlDeficit(resolve_perp_pnl_deficit::ResolvePerpPnlDeficit),
    ResolvePerpBankruptcy(resolve_perp_bankruptcy::ResolvePerpBankruptcy),
    ResolveSpotBankruptcy(resolve_spot_bankruptcy::ResolveSpotBankruptcy),
    SettleRevenueToInsuranceFund(settle_revenue_to_insurance_fund::SettleRevenueToInsuranceFund),
    UpdateFundingRate(update_funding_rate::UpdateFundingRate),
    UpdatePrelaunchOracle(update_prelaunch_oracle::UpdatePrelaunchOracle),
    UpdatePerpBidAskTwap(update_perp_bid_ask_twap::UpdatePerpBidAskTwap),
    UpdateSpotMarketCumulativeInterest(update_spot_market_cumulative_interest::UpdateSpotMarketCumulativeInterest),
    UpdateAmms(update_amms::UpdateAmms),
    UpdateSpotMarketExpiry(update_spot_market_expiry::UpdateSpotMarketExpiry),
    UpdateUserQuoteAssetInsuranceStake(update_user_quote_asset_insurance_stake::UpdateUserQuoteAssetInsuranceStake),
    UpdateUserGovTokenInsuranceStake(update_user_gov_token_insurance_stake::UpdateUserGovTokenInsuranceStake),
    UpdateUserGovTokenInsuranceStakeDevnet(update_user_gov_token_insurance_stake_devnet::UpdateUserGovTokenInsuranceStakeDevnet),
    InitializeInsuranceFundStake(initialize_insurance_fund_stake::InitializeInsuranceFundStake),
    AddInsuranceFundStake(add_insurance_fund_stake::AddInsuranceFundStake),
    RequestRemoveInsuranceFundStake(request_remove_insurance_fund_stake::RequestRemoveInsuranceFundStake),
    CancelRequestRemoveInsuranceFundStake(cancel_request_remove_insurance_fund_stake::CancelRequestRemoveInsuranceFundStake),
    RemoveInsuranceFundStake(remove_insurance_fund_stake::RemoveInsuranceFundStake),
    TransferProtocolIfShares(transfer_protocol_if_shares::TransferProtocolIfShares),
    UpdatePythPullOracle(update_pyth_pull_oracle::UpdatePythPullOracle),
    PostPythPullOracleUpdateAtomic(post_pyth_pull_oracle_update_atomic::PostPythPullOracleUpdateAtomic),
    PostMultiPythPullOracleUpdatesAtomic(post_multi_pyth_pull_oracle_updates_atomic::PostMultiPythPullOracleUpdatesAtomic),
    PauseSpotMarketDepositWithdraw(pause_spot_market_deposit_withdraw::PauseSpotMarketDepositWithdraw),
    Initialize(initialize::Initialize),
    InitializeSpotMarket(initialize_spot_market::InitializeSpotMarket),
    DeleteInitializedSpotMarket(delete_initialized_spot_market::DeleteInitializedSpotMarket),
    InitializeSerumFulfillmentConfig(initialize_serum_fulfillment_config::InitializeSerumFulfillmentConfig),
    UpdateSerumFulfillmentConfigStatus(update_serum_fulfillment_config_status::UpdateSerumFulfillmentConfigStatus),
    InitializeOpenbookV2FulfillmentConfig(initialize_openbook_v2_fulfillment_config::InitializeOpenbookV2FulfillmentConfig),
    OpenbookV2FulfillmentConfigStatus(openbook_v2_fulfillment_config_status::OpenbookV2FulfillmentConfigStatus),
    InitializePhoenixFulfillmentConfig(initialize_phoenix_fulfillment_config::InitializePhoenixFulfillmentConfig),
    PhoenixFulfillmentConfigStatus(phoenix_fulfillment_config_status::PhoenixFulfillmentConfigStatus),
    UpdateSerumVault(update_serum_vault::UpdateSerumVault),
    InitializePerpMarket(initialize_perp_market::InitializePerpMarket),
    InitializePredictionMarket(initialize_prediction_market::InitializePredictionMarket),
    DeleteInitializedPerpMarket(delete_initialized_perp_market::DeleteInitializedPerpMarket),
    MoveAmmPrice(move_amm_price::MoveAmmPrice),
    RecenterPerpMarketAmm(recenter_perp_market_amm::RecenterPerpMarketAmm),
    UpdatePerpMarketAmmSummaryStats(update_perp_market_amm_summary_stats::UpdatePerpMarketAmmSummaryStats),
    UpdatePerpMarketExpiry(update_perp_market_expiry::UpdatePerpMarketExpiry),
    SettleExpiredMarketPoolsToRevenuePool(settle_expired_market_pools_to_revenue_pool::SettleExpiredMarketPoolsToRevenuePool),
    DepositIntoPerpMarketFeePool(deposit_into_perp_market_fee_pool::DepositIntoPerpMarketFeePool),
    DepositIntoSpotMarketVault(deposit_into_spot_market_vault::DepositIntoSpotMarketVault),
    DepositIntoSpotMarketRevenuePool(deposit_into_spot_market_revenue_pool::DepositIntoSpotMarketRevenuePool),
    RepegAmmCurve(repeg_amm_curve::RepegAmmCurve),
    UpdatePerpMarketAmmOracleTwap(update_perp_market_amm_oracle_twap::UpdatePerpMarketAmmOracleTwap),
    ResetPerpMarketAmmOracleTwap(reset_perp_market_amm_oracle_twap::ResetPerpMarketAmmOracleTwap),
    UpdateK(update_k::UpdateK),
    UpdatePerpMarketMarginRatio(update_perp_market_margin_ratio::UpdatePerpMarketMarginRatio),
    UpdatePerpMarketHighLeverageMarginRatio(update_perp_market_high_leverage_margin_ratio::UpdatePerpMarketHighLeverageMarginRatio),
    UpdatePerpMarketFundingPeriod(update_perp_market_funding_period::UpdatePerpMarketFundingPeriod),
    UpdatePerpMarketMaxImbalances(update_perp_market_max_imbalances::UpdatePerpMarketMaxImbalances),
    UpdatePerpMarketLiquidationFee(update_perp_market_liquidation_fee::UpdatePerpMarketLiquidationFee),
    UpdateInsuranceFundUnstakingPeriod(update_insurance_fund_unstaking_period::UpdateInsuranceFundUnstakingPeriod),
    UpdateSpotMarketPoolId(update_spot_market_pool_id::UpdateSpotMarketPoolId),
    UpdateSpotMarketLiquidationFee(update_spot_market_liquidation_fee::UpdateSpotMarketLiquidationFee),
    UpdateWithdrawGuardThreshold(update_withdraw_guard_threshold::UpdateWithdrawGuardThreshold),
    UpdateSpotMarketIfFactor(update_spot_market_if_factor::UpdateSpotMarketIfFactor),
    UpdateSpotMarketRevenueSettlePeriod(update_spot_market_revenue_settle_period::UpdateSpotMarketRevenueSettlePeriod),
    UpdateSpotMarketStatus(update_spot_market_status::UpdateSpotMarketStatus),
    UpdateSpotMarketPausedOperations(update_spot_market_paused_operations::UpdateSpotMarketPausedOperations),
    UpdateSpotMarketAssetTier(update_spot_market_asset_tier::UpdateSpotMarketAssetTier),
    UpdateSpotMarketMarginWeights(update_spot_market_margin_weights::UpdateSpotMarketMarginWeights),
    UpdateSpotMarketBorrowRate(update_spot_market_borrow_rate::UpdateSpotMarketBorrowRate),
    UpdateSpotMarketMaxTokenDeposits(update_spot_market_max_token_deposits::UpdateSpotMarketMaxTokenDeposits),
    UpdateSpotMarketMaxTokenBorrows(update_spot_market_max_token_borrows::UpdateSpotMarketMaxTokenBorrows),
    UpdateSpotMarketScaleInitialAssetWeightStart(update_spot_market_scale_initial_asset_weight_start::UpdateSpotMarketScaleInitialAssetWeightStart),
    UpdateSpotMarketOracle(update_spot_market_oracle::UpdateSpotMarketOracle),
    UpdateSpotMarketStepSizeAndTickSize(update_spot_market_step_size_and_tick_size::UpdateSpotMarketStepSizeAndTickSize),
    UpdateSpotMarketMinOrderSize(update_spot_market_min_order_size::UpdateSpotMarketMinOrderSize),
    UpdateSpotMarketOrdersEnabled(update_spot_market_orders_enabled::UpdateSpotMarketOrdersEnabled),
    UpdateSpotMarketIfPausedOperations(update_spot_market_if_paused_operations::UpdateSpotMarketIfPausedOperations),
    UpdateSpotMarketName(update_spot_market_name::UpdateSpotMarketName),
    UpdatePerpMarketStatus(update_perp_market_status::UpdatePerpMarketStatus),
    UpdatePerpMarketPausedOperations(update_perp_market_paused_operations::UpdatePerpMarketPausedOperations),
    UpdatePerpMarketContractTier(update_perp_market_contract_tier::UpdatePerpMarketContractTier),
    UpdatePerpMarketImfFactor(update_perp_market_imf_factor::UpdatePerpMarketImfFactor),
    UpdatePerpMarketUnrealizedAssetWeight(update_perp_market_unrealized_asset_weight::UpdatePerpMarketUnrealizedAssetWeight),
    UpdatePerpMarketConcentrationCoef(update_perp_market_concentration_coef::UpdatePerpMarketConcentrationCoef),
    UpdatePerpMarketCurveUpdateIntensity(update_perp_market_curve_update_intensity::UpdatePerpMarketCurveUpdateIntensity),
    UpdatePerpMarketTargetBaseAssetAmountPerLp(update_perp_market_target_base_asset_amount_per_lp::UpdatePerpMarketTargetBaseAssetAmountPerLp),
    UpdatePerpMarketPerLpBase(update_perp_market_per_lp_base::UpdatePerpMarketPerLpBase),
    UpdateLpCooldownTime(update_lp_cooldown_time::UpdateLpCooldownTime),
    UpdatePerpFeeStructure(update_perp_fee_structure::UpdatePerpFeeStructure),
    UpdateSpotFeeStructure(update_spot_fee_structure::UpdateSpotFeeStructure),
    UpdateInitialPctToLiquidate(update_initial_pct_to_liquidate::UpdateInitialPctToLiquidate),
    UpdateLiquidationDuration(update_liquidation_duration::UpdateLiquidationDuration),
    UpdateLiquidationMarginBufferRatio(update_liquidation_margin_buffer_ratio::UpdateLiquidationMarginBufferRatio),
    UpdateOracleGuardRails(update_oracle_guard_rails::UpdateOracleGuardRails),
    UpdateStateSettlementDuration(update_state_settlement_duration::UpdateStateSettlementDuration),
    UpdateStateMaxNumberOfSubAccounts(update_state_max_number_of_sub_accounts::UpdateStateMaxNumberOfSubAccounts),
    UpdateStateMaxInitializeUserFee(update_state_max_initialize_user_fee::UpdateStateMaxInitializeUserFee),
    UpdatePerpMarketOracle(update_perp_market_oracle::UpdatePerpMarketOracle),
    UpdatePerpMarketBaseSpread(update_perp_market_base_spread::UpdatePerpMarketBaseSpread),
    UpdateAmmJitIntensity(update_amm_jit_intensity::UpdateAmmJitIntensity),
    UpdatePerpMarketMaxSpread(update_perp_market_max_spread::UpdatePerpMarketMaxSpread),
    UpdatePerpMarketStepSizeAndTickSize(update_perp_market_step_size_and_tick_size::UpdatePerpMarketStepSizeAndTickSize),
    UpdatePerpMarketName(update_perp_market_name::UpdatePerpMarketName),
    UpdatePerpMarketMinOrderSize(update_perp_market_min_order_size::UpdatePerpMarketMinOrderSize),
    UpdatePerpMarketMaxSlippageRatio(update_perp_market_max_slippage_ratio::UpdatePerpMarketMaxSlippageRatio),
    UpdatePerpMarketMaxFillReserveFraction(update_perp_market_max_fill_reserve_fraction::UpdatePerpMarketMaxFillReserveFraction),
    UpdatePerpMarketMaxOpenInterest(update_perp_market_max_open_interest::UpdatePerpMarketMaxOpenInterest),
    UpdatePerpMarketNumberOfUsers(update_perp_market_number_of_users::UpdatePerpMarketNumberOfUsers),
    UpdatePerpMarketFeeAdjustment(update_perp_market_fee_adjustment::UpdatePerpMarketFeeAdjustment),
    UpdateSpotMarketFeeAdjustment(update_spot_market_fee_adjustment::UpdateSpotMarketFeeAdjustment),
    UpdatePerpMarketFuel(update_perp_market_fuel::UpdatePerpMarketFuel),
    UpdateSpotMarketFuel(update_spot_market_fuel::UpdateSpotMarketFuel),
    InitUserFuel(init_user_fuel::InitUserFuel),
    UpdateAdmin(update_admin::UpdateAdmin),
    UpdateWhitelistMint(update_whitelist_mint::UpdateWhitelistMint),
    UpdateDiscountMint(update_discount_mint::UpdateDiscountMint),
    UpdateExchangeStatus(update_exchange_status::UpdateExchangeStatus),
    UpdatePerpAuctionDuration(update_perp_auction_duration::UpdatePerpAuctionDuration),
    UpdateSpotAuctionDuration(update_spot_auction_duration::UpdateSpotAuctionDuration),
    InitializeProtocolIfSharesTransferConfig(initialize_protocol_if_shares_transfer_config::InitializeProtocolIfSharesTransferConfig),
    UpdateProtocolIfSharesTransferConfig(update_protocol_if_shares_transfer_config::UpdateProtocolIfSharesTransferConfig),
    InitializePrelaunchOracle(initialize_prelaunch_oracle::InitializePrelaunchOracle),
    UpdatePrelaunchOracleParams(update_prelaunch_oracle_params::UpdatePrelaunchOracleParams),
    DeletePrelaunchOracle(delete_prelaunch_oracle::DeletePrelaunchOracle),
    InitializePythPullOracle(initialize_pyth_pull_oracle::InitializePythPullOracle),
    InitializePythLazerOracle(initialize_pyth_lazer_oracle::InitializePythLazerOracle),
    PostPythLazerOracleUpdate(post_pyth_lazer_oracle_update::PostPythLazerOracleUpdate),
    InitializeHighLeverageModeConfig(initialize_high_leverage_mode_config::InitializeHighLeverageModeConfig),
    UpdateHighLeverageModeConfig(update_high_leverage_mode_config::UpdateHighLeverageModeConfig),
    InitializeProtectedMakerModeConfig(initialize_protected_maker_mode_config::InitializeProtectedMakerModeConfig),
    UpdateProtectedMakerModeConfig(update_protected_maker_mode_config::UpdateProtectedMakerModeConfig),
    NewUserRecordEvent(new_user_record_event::NewUserRecordEvent),
    DepositRecordEvent(deposit_record_event::DepositRecordEvent),
    SpotInterestRecordEvent(spot_interest_record_event::SpotInterestRecordEvent),
    FundingPaymentRecordEvent(funding_payment_record_event::FundingPaymentRecordEvent),
    FundingRateRecordEvent(funding_rate_record_event::FundingRateRecordEvent),
    CurveRecordEvent(curve_record_event::CurveRecordEvent),
    SignedMsgOrderRecordEvent(signed_msg_order_record_event::SignedMsgOrderRecordEvent),
    OrderRecordEvent(order_record_event::OrderRecordEvent),
    OrderActionRecordEvent(order_action_record_event::OrderActionRecordEvent),
    LpRecordEvent(lp_record_event::LpRecordEvent),
    LiquidationRecordEvent(liquidation_record_event::LiquidationRecordEvent),
    SettlePnlRecordEvent(settle_pnl_record_event::SettlePnlRecordEvent),
    InsuranceFundRecordEvent(insurance_fund_record_event::InsuranceFundRecordEvent),
    InsuranceFundStakeRecordEvent(insurance_fund_stake_record_event::InsuranceFundStakeRecordEvent),
    SwapRecordEvent(swap_record_event::SwapRecordEvent),
    SpotMarketVaultDepositRecordEvent(spot_market_vault_deposit_record_event::SpotMarketVaultDepositRecordEvent),
    DeleteUserRecordEvent(delete_user_record_event::DeleteUserRecordEvent),
    FuelSweepRecordEvent(fuel_sweep_record_event::FuelSweepRecordEvent),
    FuelSeasonRecordEvent(fuel_season_record_event::FuelSeasonRecordEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for DriftDecoder {
    type InstructionType = DriftInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            DriftInstruction::InitializeUser => initialize_user::InitializeUser,
            DriftInstruction::InitializeUserStats => initialize_user_stats::InitializeUserStats,
            DriftInstruction::InitializeSignedMsgUserOrders => initialize_signed_msg_user_orders::InitializeSignedMsgUserOrders,
            DriftInstruction::ResizeSignedMsgUserOrders => resize_signed_msg_user_orders::ResizeSignedMsgUserOrders,
            DriftInstruction::InitializeFuelOverflow => initialize_fuel_overflow::InitializeFuelOverflow,
            DriftInstruction::SweepFuel => sweep_fuel::SweepFuel,
            DriftInstruction::ResetFuelSeason => reset_fuel_season::ResetFuelSeason,
            DriftInstruction::InitializeReferrerName => initialize_referrer_name::InitializeReferrerName,
            DriftInstruction::Deposit => deposit::Deposit,
            DriftInstruction::Withdraw => withdraw::Withdraw,
            DriftInstruction::TransferDeposit => transfer_deposit::TransferDeposit,
            DriftInstruction::TransferPools => transfer_pools::TransferPools,
            DriftInstruction::PlacePerpOrder => place_perp_order::PlacePerpOrder,
            DriftInstruction::CancelOrder => cancel_order::CancelOrder,
            DriftInstruction::CancelOrderByUserId => cancel_order_by_user_id::CancelOrderByUserId,
            DriftInstruction::CancelOrders => cancel_orders::CancelOrders,
            DriftInstruction::CancelOrdersByIds => cancel_orders_by_ids::CancelOrdersByIds,
            DriftInstruction::ModifyOrder => modify_order::ModifyOrder,
            DriftInstruction::ModifyOrderByUserId => modify_order_by_user_id::ModifyOrderByUserId,
            DriftInstruction::PlaceAndTakePerpOrder => place_and_take_perp_order::PlaceAndTakePerpOrder,
            DriftInstruction::PlaceAndMakePerpOrder => place_and_make_perp_order::PlaceAndMakePerpOrder,
            DriftInstruction::PlaceAndMakeSignedMsgPerpOrder => place_and_make_signed_msg_perp_order::PlaceAndMakeSignedMsgPerpOrder,
            DriftInstruction::PlaceSignedMsgTakerOrder => place_signed_msg_taker_order::PlaceSignedMsgTakerOrder,
            DriftInstruction::PlaceSpotOrder => place_spot_order::PlaceSpotOrder,
            DriftInstruction::PlaceAndTakeSpotOrder => place_and_take_spot_order::PlaceAndTakeSpotOrder,
            DriftInstruction::PlaceAndMakeSpotOrder => place_and_make_spot_order::PlaceAndMakeSpotOrder,
            DriftInstruction::PlaceOrders => place_orders::PlaceOrders,
            DriftInstruction::BeginSwap => begin_swap::BeginSwap,
            DriftInstruction::EndSwap => end_swap::EndSwap,
            DriftInstruction::AddPerpLpShares => add_perp_lp_shares::AddPerpLpShares,
            DriftInstruction::RemovePerpLpShares => remove_perp_lp_shares::RemovePerpLpShares,
            DriftInstruction::RemovePerpLpSharesInExpiringMarket => remove_perp_lp_shares_in_expiring_market::RemovePerpLpSharesInExpiringMarket,
            DriftInstruction::UpdateUserName => update_user_name::UpdateUserName,
            DriftInstruction::UpdateUserCustomMarginRatio => update_user_custom_margin_ratio::UpdateUserCustomMarginRatio,
            DriftInstruction::UpdateUserMarginTradingEnabled => update_user_margin_trading_enabled::UpdateUserMarginTradingEnabled,
            DriftInstruction::UpdateUserPoolId => update_user_pool_id::UpdateUserPoolId,
            DriftInstruction::UpdateUserDelegate => update_user_delegate::UpdateUserDelegate,
            DriftInstruction::UpdateUserReduceOnly => update_user_reduce_only::UpdateUserReduceOnly,
            DriftInstruction::UpdateUserAdvancedLp => update_user_advanced_lp::UpdateUserAdvancedLp,
            DriftInstruction::UpdateUserProtectedMakerOrders => update_user_protected_maker_orders::UpdateUserProtectedMakerOrders,
            DriftInstruction::DeleteUser => delete_user::DeleteUser,
            DriftInstruction::ForceDeleteUser => force_delete_user::ForceDeleteUser,
            DriftInstruction::DeleteSignedMsgUserOrders => delete_signed_msg_user_orders::DeleteSignedMsgUserOrders,
            DriftInstruction::ReclaimRent => reclaim_rent::ReclaimRent,
            DriftInstruction::EnableUserHighLeverageMode => enable_user_high_leverage_mode::EnableUserHighLeverageMode,
            DriftInstruction::FillPerpOrder => fill_perp_order::FillPerpOrder,
            DriftInstruction::RevertFill => revert_fill::RevertFill,
            DriftInstruction::FillSpotOrder => fill_spot_order::FillSpotOrder,
            DriftInstruction::TriggerOrder => trigger_order::TriggerOrder,
            DriftInstruction::ForceCancelOrders => force_cancel_orders::ForceCancelOrders,
            DriftInstruction::UpdateUserIdle => update_user_idle::UpdateUserIdle,
            DriftInstruction::LogUserBalances => log_user_balances::LogUserBalances,
            DriftInstruction::DisableUserHighLeverageMode => disable_user_high_leverage_mode::DisableUserHighLeverageMode,
            DriftInstruction::UpdateUserFuelBonus => update_user_fuel_bonus::UpdateUserFuelBonus,
            DriftInstruction::UpdateUserStatsReferrerStatus => update_user_stats_referrer_status::UpdateUserStatsReferrerStatus,
            DriftInstruction::UpdateUserOpenOrdersCount => update_user_open_orders_count::UpdateUserOpenOrdersCount,
            DriftInstruction::AdminDisableUpdatePerpBidAskTwap => admin_disable_update_perp_bid_ask_twap::AdminDisableUpdatePerpBidAskTwap,
            DriftInstruction::SettlePnl => settle_pnl::SettlePnl,
            DriftInstruction::SettleMultiplePnls => settle_multiple_pnls::SettleMultiplePnls,
            DriftInstruction::SettleFundingPayment => settle_funding_payment::SettleFundingPayment,
            DriftInstruction::SettleLp => settle_lp::SettleLp,
            DriftInstruction::SettleExpiredMarket => settle_expired_market::SettleExpiredMarket,
            DriftInstruction::LiquidatePerp => liquidate_perp::LiquidatePerp,
            DriftInstruction::LiquidatePerpWithFill => liquidate_perp_with_fill::LiquidatePerpWithFill,
            DriftInstruction::LiquidateSpot => liquidate_spot::LiquidateSpot,
            DriftInstruction::LiquidateSpotWithSwapBegin => liquidate_spot_with_swap_begin::LiquidateSpotWithSwapBegin,
            DriftInstruction::LiquidateSpotWithSwapEnd => liquidate_spot_with_swap_end::LiquidateSpotWithSwapEnd,
            DriftInstruction::LiquidateBorrowForPerpPnl => liquidate_borrow_for_perp_pnl::LiquidateBorrowForPerpPnl,
            DriftInstruction::LiquidatePerpPnlForDeposit => liquidate_perp_pnl_for_deposit::LiquidatePerpPnlForDeposit,
            DriftInstruction::SetUserStatusToBeingLiquidated => set_user_status_to_being_liquidated::SetUserStatusToBeingLiquidated,
            DriftInstruction::ResolvePerpPnlDeficit => resolve_perp_pnl_deficit::ResolvePerpPnlDeficit,
            DriftInstruction::ResolvePerpBankruptcy => resolve_perp_bankruptcy::ResolvePerpBankruptcy,
            DriftInstruction::ResolveSpotBankruptcy => resolve_spot_bankruptcy::ResolveSpotBankruptcy,
            DriftInstruction::SettleRevenueToInsuranceFund => settle_revenue_to_insurance_fund::SettleRevenueToInsuranceFund,
            DriftInstruction::UpdateFundingRate => update_funding_rate::UpdateFundingRate,
            DriftInstruction::UpdatePrelaunchOracle => update_prelaunch_oracle::UpdatePrelaunchOracle,
            DriftInstruction::UpdatePerpBidAskTwap => update_perp_bid_ask_twap::UpdatePerpBidAskTwap,
            DriftInstruction::UpdateSpotMarketCumulativeInterest => update_spot_market_cumulative_interest::UpdateSpotMarketCumulativeInterest,
            DriftInstruction::UpdateAmms => update_amms::UpdateAmms,
            DriftInstruction::UpdateSpotMarketExpiry => update_spot_market_expiry::UpdateSpotMarketExpiry,
            DriftInstruction::UpdateUserQuoteAssetInsuranceStake => update_user_quote_asset_insurance_stake::UpdateUserQuoteAssetInsuranceStake,
            DriftInstruction::UpdateUserGovTokenInsuranceStake => update_user_gov_token_insurance_stake::UpdateUserGovTokenInsuranceStake,
            DriftInstruction::UpdateUserGovTokenInsuranceStakeDevnet => update_user_gov_token_insurance_stake_devnet::UpdateUserGovTokenInsuranceStakeDevnet,
            DriftInstruction::InitializeInsuranceFundStake => initialize_insurance_fund_stake::InitializeInsuranceFundStake,
            DriftInstruction::AddInsuranceFundStake => add_insurance_fund_stake::AddInsuranceFundStake,
            DriftInstruction::RequestRemoveInsuranceFundStake => request_remove_insurance_fund_stake::RequestRemoveInsuranceFundStake,
            DriftInstruction::CancelRequestRemoveInsuranceFundStake => cancel_request_remove_insurance_fund_stake::CancelRequestRemoveInsuranceFundStake,
            DriftInstruction::RemoveInsuranceFundStake => remove_insurance_fund_stake::RemoveInsuranceFundStake,
            DriftInstruction::TransferProtocolIfShares => transfer_protocol_if_shares::TransferProtocolIfShares,
            DriftInstruction::UpdatePythPullOracle => update_pyth_pull_oracle::UpdatePythPullOracle,
            DriftInstruction::PostPythPullOracleUpdateAtomic => post_pyth_pull_oracle_update_atomic::PostPythPullOracleUpdateAtomic,
            DriftInstruction::PostMultiPythPullOracleUpdatesAtomic => post_multi_pyth_pull_oracle_updates_atomic::PostMultiPythPullOracleUpdatesAtomic,
            DriftInstruction::PauseSpotMarketDepositWithdraw => pause_spot_market_deposit_withdraw::PauseSpotMarketDepositWithdraw,
            DriftInstruction::Initialize => initialize::Initialize,
            DriftInstruction::InitializeSpotMarket => initialize_spot_market::InitializeSpotMarket,
            DriftInstruction::DeleteInitializedSpotMarket => delete_initialized_spot_market::DeleteInitializedSpotMarket,
            DriftInstruction::InitializeSerumFulfillmentConfig => initialize_serum_fulfillment_config::InitializeSerumFulfillmentConfig,
            DriftInstruction::UpdateSerumFulfillmentConfigStatus => update_serum_fulfillment_config_status::UpdateSerumFulfillmentConfigStatus,
            DriftInstruction::InitializeOpenbookV2FulfillmentConfig => initialize_openbook_v2_fulfillment_config::InitializeOpenbookV2FulfillmentConfig,
            DriftInstruction::OpenbookV2FulfillmentConfigStatus => openbook_v2_fulfillment_config_status::OpenbookV2FulfillmentConfigStatus,
            DriftInstruction::InitializePhoenixFulfillmentConfig => initialize_phoenix_fulfillment_config::InitializePhoenixFulfillmentConfig,
            DriftInstruction::PhoenixFulfillmentConfigStatus => phoenix_fulfillment_config_status::PhoenixFulfillmentConfigStatus,
            DriftInstruction::UpdateSerumVault => update_serum_vault::UpdateSerumVault,
            DriftInstruction::InitializePerpMarket => initialize_perp_market::InitializePerpMarket,
            DriftInstruction::InitializePredictionMarket => initialize_prediction_market::InitializePredictionMarket,
            DriftInstruction::DeleteInitializedPerpMarket => delete_initialized_perp_market::DeleteInitializedPerpMarket,
            DriftInstruction::MoveAmmPrice => move_amm_price::MoveAmmPrice,
            DriftInstruction::RecenterPerpMarketAmm => recenter_perp_market_amm::RecenterPerpMarketAmm,
            DriftInstruction::UpdatePerpMarketAmmSummaryStats => update_perp_market_amm_summary_stats::UpdatePerpMarketAmmSummaryStats,
            DriftInstruction::UpdatePerpMarketExpiry => update_perp_market_expiry::UpdatePerpMarketExpiry,
            DriftInstruction::SettleExpiredMarketPoolsToRevenuePool => settle_expired_market_pools_to_revenue_pool::SettleExpiredMarketPoolsToRevenuePool,
            DriftInstruction::DepositIntoPerpMarketFeePool => deposit_into_perp_market_fee_pool::DepositIntoPerpMarketFeePool,
            DriftInstruction::DepositIntoSpotMarketVault => deposit_into_spot_market_vault::DepositIntoSpotMarketVault,
            DriftInstruction::DepositIntoSpotMarketRevenuePool => deposit_into_spot_market_revenue_pool::DepositIntoSpotMarketRevenuePool,
            DriftInstruction::RepegAmmCurve => repeg_amm_curve::RepegAmmCurve,
            DriftInstruction::UpdatePerpMarketAmmOracleTwap => update_perp_market_amm_oracle_twap::UpdatePerpMarketAmmOracleTwap,
            DriftInstruction::ResetPerpMarketAmmOracleTwap => reset_perp_market_amm_oracle_twap::ResetPerpMarketAmmOracleTwap,
            DriftInstruction::UpdateK => update_k::UpdateK,
            DriftInstruction::UpdatePerpMarketMarginRatio => update_perp_market_margin_ratio::UpdatePerpMarketMarginRatio,
            DriftInstruction::UpdatePerpMarketHighLeverageMarginRatio => update_perp_market_high_leverage_margin_ratio::UpdatePerpMarketHighLeverageMarginRatio,
            DriftInstruction::UpdatePerpMarketFundingPeriod => update_perp_market_funding_period::UpdatePerpMarketFundingPeriod,
            DriftInstruction::UpdatePerpMarketMaxImbalances => update_perp_market_max_imbalances::UpdatePerpMarketMaxImbalances,
            DriftInstruction::UpdatePerpMarketLiquidationFee => update_perp_market_liquidation_fee::UpdatePerpMarketLiquidationFee,
            DriftInstruction::UpdateInsuranceFundUnstakingPeriod => update_insurance_fund_unstaking_period::UpdateInsuranceFundUnstakingPeriod,
            DriftInstruction::UpdateSpotMarketPoolId => update_spot_market_pool_id::UpdateSpotMarketPoolId,
            DriftInstruction::UpdateSpotMarketLiquidationFee => update_spot_market_liquidation_fee::UpdateSpotMarketLiquidationFee,
            DriftInstruction::UpdateWithdrawGuardThreshold => update_withdraw_guard_threshold::UpdateWithdrawGuardThreshold,
            DriftInstruction::UpdateSpotMarketIfFactor => update_spot_market_if_factor::UpdateSpotMarketIfFactor,
            DriftInstruction::UpdateSpotMarketRevenueSettlePeriod => update_spot_market_revenue_settle_period::UpdateSpotMarketRevenueSettlePeriod,
            DriftInstruction::UpdateSpotMarketStatus => update_spot_market_status::UpdateSpotMarketStatus,
            DriftInstruction::UpdateSpotMarketPausedOperations => update_spot_market_paused_operations::UpdateSpotMarketPausedOperations,
            DriftInstruction::UpdateSpotMarketAssetTier => update_spot_market_asset_tier::UpdateSpotMarketAssetTier,
            DriftInstruction::UpdateSpotMarketMarginWeights => update_spot_market_margin_weights::UpdateSpotMarketMarginWeights,
            DriftInstruction::UpdateSpotMarketBorrowRate => update_spot_market_borrow_rate::UpdateSpotMarketBorrowRate,
            DriftInstruction::UpdateSpotMarketMaxTokenDeposits => update_spot_market_max_token_deposits::UpdateSpotMarketMaxTokenDeposits,
            DriftInstruction::UpdateSpotMarketMaxTokenBorrows => update_spot_market_max_token_borrows::UpdateSpotMarketMaxTokenBorrows,
            DriftInstruction::UpdateSpotMarketScaleInitialAssetWeightStart => update_spot_market_scale_initial_asset_weight_start::UpdateSpotMarketScaleInitialAssetWeightStart,
            DriftInstruction::UpdateSpotMarketOracle => update_spot_market_oracle::UpdateSpotMarketOracle,
            DriftInstruction::UpdateSpotMarketStepSizeAndTickSize => update_spot_market_step_size_and_tick_size::UpdateSpotMarketStepSizeAndTickSize,
            DriftInstruction::UpdateSpotMarketMinOrderSize => update_spot_market_min_order_size::UpdateSpotMarketMinOrderSize,
            DriftInstruction::UpdateSpotMarketOrdersEnabled => update_spot_market_orders_enabled::UpdateSpotMarketOrdersEnabled,
            DriftInstruction::UpdateSpotMarketIfPausedOperations => update_spot_market_if_paused_operations::UpdateSpotMarketIfPausedOperations,
            DriftInstruction::UpdateSpotMarketName => update_spot_market_name::UpdateSpotMarketName,
            DriftInstruction::UpdatePerpMarketStatus => update_perp_market_status::UpdatePerpMarketStatus,
            DriftInstruction::UpdatePerpMarketPausedOperations => update_perp_market_paused_operations::UpdatePerpMarketPausedOperations,
            DriftInstruction::UpdatePerpMarketContractTier => update_perp_market_contract_tier::UpdatePerpMarketContractTier,
            DriftInstruction::UpdatePerpMarketImfFactor => update_perp_market_imf_factor::UpdatePerpMarketImfFactor,
            DriftInstruction::UpdatePerpMarketUnrealizedAssetWeight => update_perp_market_unrealized_asset_weight::UpdatePerpMarketUnrealizedAssetWeight,
            DriftInstruction::UpdatePerpMarketConcentrationCoef => update_perp_market_concentration_coef::UpdatePerpMarketConcentrationCoef,
            DriftInstruction::UpdatePerpMarketCurveUpdateIntensity => update_perp_market_curve_update_intensity::UpdatePerpMarketCurveUpdateIntensity,
            DriftInstruction::UpdatePerpMarketTargetBaseAssetAmountPerLp => update_perp_market_target_base_asset_amount_per_lp::UpdatePerpMarketTargetBaseAssetAmountPerLp,
            DriftInstruction::UpdatePerpMarketPerLpBase => update_perp_market_per_lp_base::UpdatePerpMarketPerLpBase,
            DriftInstruction::UpdateLpCooldownTime => update_lp_cooldown_time::UpdateLpCooldownTime,
            DriftInstruction::UpdatePerpFeeStructure => update_perp_fee_structure::UpdatePerpFeeStructure,
            DriftInstruction::UpdateSpotFeeStructure => update_spot_fee_structure::UpdateSpotFeeStructure,
            DriftInstruction::UpdateInitialPctToLiquidate => update_initial_pct_to_liquidate::UpdateInitialPctToLiquidate,
            DriftInstruction::UpdateLiquidationDuration => update_liquidation_duration::UpdateLiquidationDuration,
            DriftInstruction::UpdateLiquidationMarginBufferRatio => update_liquidation_margin_buffer_ratio::UpdateLiquidationMarginBufferRatio,
            DriftInstruction::UpdateOracleGuardRails => update_oracle_guard_rails::UpdateOracleGuardRails,
            DriftInstruction::UpdateStateSettlementDuration => update_state_settlement_duration::UpdateStateSettlementDuration,
            DriftInstruction::UpdateStateMaxNumberOfSubAccounts => update_state_max_number_of_sub_accounts::UpdateStateMaxNumberOfSubAccounts,
            DriftInstruction::UpdateStateMaxInitializeUserFee => update_state_max_initialize_user_fee::UpdateStateMaxInitializeUserFee,
            DriftInstruction::UpdatePerpMarketOracle => update_perp_market_oracle::UpdatePerpMarketOracle,
            DriftInstruction::UpdatePerpMarketBaseSpread => update_perp_market_base_spread::UpdatePerpMarketBaseSpread,
            DriftInstruction::UpdateAmmJitIntensity => update_amm_jit_intensity::UpdateAmmJitIntensity,
            DriftInstruction::UpdatePerpMarketMaxSpread => update_perp_market_max_spread::UpdatePerpMarketMaxSpread,
            DriftInstruction::UpdatePerpMarketStepSizeAndTickSize => update_perp_market_step_size_and_tick_size::UpdatePerpMarketStepSizeAndTickSize,
            DriftInstruction::UpdatePerpMarketName => update_perp_market_name::UpdatePerpMarketName,
            DriftInstruction::UpdatePerpMarketMinOrderSize => update_perp_market_min_order_size::UpdatePerpMarketMinOrderSize,
            DriftInstruction::UpdatePerpMarketMaxSlippageRatio => update_perp_market_max_slippage_ratio::UpdatePerpMarketMaxSlippageRatio,
            DriftInstruction::UpdatePerpMarketMaxFillReserveFraction => update_perp_market_max_fill_reserve_fraction::UpdatePerpMarketMaxFillReserveFraction,
            DriftInstruction::UpdatePerpMarketMaxOpenInterest => update_perp_market_max_open_interest::UpdatePerpMarketMaxOpenInterest,
            DriftInstruction::UpdatePerpMarketNumberOfUsers => update_perp_market_number_of_users::UpdatePerpMarketNumberOfUsers,
            DriftInstruction::UpdatePerpMarketFeeAdjustment => update_perp_market_fee_adjustment::UpdatePerpMarketFeeAdjustment,
            DriftInstruction::UpdateSpotMarketFeeAdjustment => update_spot_market_fee_adjustment::UpdateSpotMarketFeeAdjustment,
            DriftInstruction::UpdatePerpMarketFuel => update_perp_market_fuel::UpdatePerpMarketFuel,
            DriftInstruction::UpdateSpotMarketFuel => update_spot_market_fuel::UpdateSpotMarketFuel,
            DriftInstruction::InitUserFuel => init_user_fuel::InitUserFuel,
            DriftInstruction::UpdateAdmin => update_admin::UpdateAdmin,
            DriftInstruction::UpdateWhitelistMint => update_whitelist_mint::UpdateWhitelistMint,
            DriftInstruction::UpdateDiscountMint => update_discount_mint::UpdateDiscountMint,
            DriftInstruction::UpdateExchangeStatus => update_exchange_status::UpdateExchangeStatus,
            DriftInstruction::UpdatePerpAuctionDuration => update_perp_auction_duration::UpdatePerpAuctionDuration,
            DriftInstruction::UpdateSpotAuctionDuration => update_spot_auction_duration::UpdateSpotAuctionDuration,
            DriftInstruction::InitializeProtocolIfSharesTransferConfig => initialize_protocol_if_shares_transfer_config::InitializeProtocolIfSharesTransferConfig,
            DriftInstruction::UpdateProtocolIfSharesTransferConfig => update_protocol_if_shares_transfer_config::UpdateProtocolIfSharesTransferConfig,
            DriftInstruction::InitializePrelaunchOracle => initialize_prelaunch_oracle::InitializePrelaunchOracle,
            DriftInstruction::UpdatePrelaunchOracleParams => update_prelaunch_oracle_params::UpdatePrelaunchOracleParams,
            DriftInstruction::DeletePrelaunchOracle => delete_prelaunch_oracle::DeletePrelaunchOracle,
            DriftInstruction::InitializePythPullOracle => initialize_pyth_pull_oracle::InitializePythPullOracle,
            DriftInstruction::InitializePythLazerOracle => initialize_pyth_lazer_oracle::InitializePythLazerOracle,
            DriftInstruction::PostPythLazerOracleUpdate => post_pyth_lazer_oracle_update::PostPythLazerOracleUpdate,
            DriftInstruction::InitializeHighLeverageModeConfig => initialize_high_leverage_mode_config::InitializeHighLeverageModeConfig,
            DriftInstruction::UpdateHighLeverageModeConfig => update_high_leverage_mode_config::UpdateHighLeverageModeConfig,
            DriftInstruction::InitializeProtectedMakerModeConfig => initialize_protected_maker_mode_config::InitializeProtectedMakerModeConfig,
            DriftInstruction::UpdateProtectedMakerModeConfig => update_protected_maker_mode_config::UpdateProtectedMakerModeConfig,
            DriftInstruction::NewUserRecordEvent => new_user_record_event::NewUserRecordEvent,
            DriftInstruction::DepositRecordEvent => deposit_record_event::DepositRecordEvent,
            DriftInstruction::SpotInterestRecordEvent => spot_interest_record_event::SpotInterestRecordEvent,
            DriftInstruction::FundingPaymentRecordEvent => funding_payment_record_event::FundingPaymentRecordEvent,
            DriftInstruction::FundingRateRecordEvent => funding_rate_record_event::FundingRateRecordEvent,
            DriftInstruction::CurveRecordEvent => curve_record_event::CurveRecordEvent,
            DriftInstruction::SignedMsgOrderRecordEvent => signed_msg_order_record_event::SignedMsgOrderRecordEvent,
            DriftInstruction::OrderRecordEvent => order_record_event::OrderRecordEvent,
            DriftInstruction::OrderActionRecordEvent => order_action_record_event::OrderActionRecordEvent,
            DriftInstruction::LpRecordEvent => lp_record_event::LpRecordEvent,
            DriftInstruction::LiquidationRecordEvent => liquidation_record_event::LiquidationRecordEvent,
            DriftInstruction::SettlePnlRecordEvent => settle_pnl_record_event::SettlePnlRecordEvent,
            DriftInstruction::InsuranceFundRecordEvent => insurance_fund_record_event::InsuranceFundRecordEvent,
            DriftInstruction::InsuranceFundStakeRecordEvent => insurance_fund_stake_record_event::InsuranceFundStakeRecordEvent,
            DriftInstruction::SwapRecordEvent => swap_record_event::SwapRecordEvent,
            DriftInstruction::SpotMarketVaultDepositRecordEvent => spot_market_vault_deposit_record_event::SpotMarketVaultDepositRecordEvent,
            DriftInstruction::DeleteUserRecordEvent => delete_user_record_event::DeleteUserRecordEvent,
            DriftInstruction::FuelSweepRecordEvent => fuel_sweep_record_event::FuelSweepRecordEvent,
            DriftInstruction::FuelSeasonRecordEvent => fuel_season_record_event::FuelSeasonRecordEvent,
        )
    }
}
