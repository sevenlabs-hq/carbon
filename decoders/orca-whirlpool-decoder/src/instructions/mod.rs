use crate::PROGRAM_ID;

use super::OrcaWhirlpoolDecoder;
pub mod close_bundled_position;
pub mod close_position;
pub mod collect_fees;
pub mod collect_fees_v2;
pub mod collect_protocol_fees;
pub mod collect_protocol_fees_v2;
pub mod collect_reward;
pub mod collect_reward_v2;
pub mod decrease_liquidity;
pub mod decrease_liquidity_v2;
pub mod delete_position_bundle;
pub mod delete_token_badge;
pub mod increase_liquidity;
pub mod increase_liquidity_v2;
pub mod initialize_config;
pub mod initialize_config_extension;
pub mod initialize_fee_tier;
pub mod initialize_pool;
pub mod initialize_pool_v2;
pub mod initialize_position_bundle;
pub mod initialize_position_bundle_with_metadata;
pub mod initialize_reward;
pub mod initialize_reward_v2;
pub mod initialize_tick_array;
pub mod initialize_token_badge;
pub mod open_bundled_position;
pub mod open_position;
pub mod open_position_with_metadata;
pub mod set_collect_protocol_fees_authority;
pub mod set_config_extension_authority;
pub mod set_default_fee_rate;
pub mod set_default_protocol_fee_rate;
pub mod set_fee_authority;
pub mod set_fee_rate;
pub mod set_protocol_fee_rate;
pub mod set_reward_authority;
pub mod set_reward_authority_by_super_authority;
pub mod set_reward_emissions;
pub mod set_reward_emissions_super_authority;
pub mod set_reward_emissions_v2;
pub mod set_token_badge_authority;
pub mod swap;
pub mod swap_v2;
pub mod two_hop_swap;
pub mod two_hop_swap_v2;
pub mod update_fees_and_rewards;

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
pub enum OrcaWhirlpoolInstruction {
    InitializeConfig(initialize_config::InitializeConfig),
    InitializePool(initialize_pool::InitializePool),
    InitializeTickArray(initialize_tick_array::InitializeTickArray),
    InitializeFeeTier(initialize_fee_tier::InitializeFeeTier),
    InitializeReward(initialize_reward::InitializeReward),
    SetRewardEmissions(set_reward_emissions::SetRewardEmissions),
    OpenPosition(open_position::OpenPosition),
    OpenPositionWithMetadata(open_position_with_metadata::OpenPositionWithMetadata),
    IncreaseLiquidity(increase_liquidity::IncreaseLiquidity),
    DecreaseLiquidity(decrease_liquidity::DecreaseLiquidity),
    UpdateFeesAndRewards(update_fees_and_rewards::UpdateFeesAndRewards),
    CollectFees(collect_fees::CollectFees),
    CollectReward(collect_reward::CollectReward),
    CollectProtocolFees(collect_protocol_fees::CollectProtocolFees),
    Swap(swap::Swap),
    ClosePosition(close_position::ClosePosition),
    SetDefaultFeeRate(set_default_fee_rate::SetDefaultFeeRate),
    SetDefaultProtocolFeeRate(set_default_protocol_fee_rate::SetDefaultProtocolFeeRate),
    SetFeeRate(set_fee_rate::SetFeeRate),
    SetProtocolFeeRate(set_protocol_fee_rate::SetProtocolFeeRate),
    SetFeeAuthority(set_fee_authority::SetFeeAuthority),
    SetCollectProtocolFeesAuthority(
        set_collect_protocol_fees_authority::SetCollectProtocolFeesAuthority,
    ),
    SetRewardAuthority(set_reward_authority::SetRewardAuthority),
    SetRewardAuthorityBySuperAuthority(
        set_reward_authority_by_super_authority::SetRewardAuthorityBySuperAuthority,
    ),
    SetRewardEmissionsSuperAuthority(
        set_reward_emissions_super_authority::SetRewardEmissionsSuperAuthority,
    ),
    TwoHopSwap(two_hop_swap::TwoHopSwap),
    InitializePositionBundle(initialize_position_bundle::InitializePositionBundle),
    InitializePositionBundleWithMetadata(
        initialize_position_bundle_with_metadata::InitializePositionBundleWithMetadata,
    ),
    DeletePositionBundle(delete_position_bundle::DeletePositionBundle),
    OpenBundledPosition(open_bundled_position::OpenBundledPosition),
    CloseBundledPosition(close_bundled_position::CloseBundledPosition),
    CollectFeesV2(collect_fees_v2::CollectFeesV2),
    CollectProtocolFeesV2(collect_protocol_fees_v2::CollectProtocolFeesV2),
    CollectRewardV2(collect_reward_v2::CollectRewardV2),
    DecreaseLiquidityV2(decrease_liquidity_v2::DecreaseLiquidityV2),
    IncreaseLiquidityV2(increase_liquidity_v2::IncreaseLiquidityV2),
    InitializePoolV2(initialize_pool_v2::InitializePoolV2),
    InitializeRewardV2(initialize_reward_v2::InitializeRewardV2),
    SetRewardEmissionsV2(set_reward_emissions_v2::SetRewardEmissionsV2),
    SwapV2(swap_v2::SwapV2),
    TwoHopSwapV2(two_hop_swap_v2::TwoHopSwapV2),
    InitializeConfigExtension(initialize_config_extension::InitializeConfigExtension),
    SetConfigExtensionAuthority(set_config_extension_authority::SetConfigExtensionAuthority),
    SetTokenBadgeAuthority(set_token_badge_authority::SetTokenBadgeAuthority),
    InitializeTokenBadge(initialize_token_badge::InitializeTokenBadge),
    DeleteTokenBadge(delete_token_badge::DeleteTokenBadge),
}

impl carbon_core::instruction::InstructionDecoder<'_> for OrcaWhirlpoolDecoder {
    type InstructionType = OrcaWhirlpoolInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            OrcaWhirlpoolInstruction::InitializeConfig => initialize_config::InitializeConfig,
            OrcaWhirlpoolInstruction::InitializePool => initialize_pool::InitializePool,
            OrcaWhirlpoolInstruction::InitializeTickArray => initialize_tick_array::InitializeTickArray,
            OrcaWhirlpoolInstruction::InitializeFeeTier => initialize_fee_tier::InitializeFeeTier,
            OrcaWhirlpoolInstruction::InitializeReward => initialize_reward::InitializeReward,
            OrcaWhirlpoolInstruction::SetRewardEmissions => set_reward_emissions::SetRewardEmissions,
            OrcaWhirlpoolInstruction::OpenPosition => open_position::OpenPosition,
            OrcaWhirlpoolInstruction::OpenPositionWithMetadata => open_position_with_metadata::OpenPositionWithMetadata,
            OrcaWhirlpoolInstruction::IncreaseLiquidity => increase_liquidity::IncreaseLiquidity,
            OrcaWhirlpoolInstruction::DecreaseLiquidity => decrease_liquidity::DecreaseLiquidity,
            OrcaWhirlpoolInstruction::UpdateFeesAndRewards => update_fees_and_rewards::UpdateFeesAndRewards,
            OrcaWhirlpoolInstruction::CollectFees => collect_fees::CollectFees,
            OrcaWhirlpoolInstruction::CollectReward => collect_reward::CollectReward,
            OrcaWhirlpoolInstruction::CollectProtocolFees => collect_protocol_fees::CollectProtocolFees,
            OrcaWhirlpoolInstruction::Swap => swap::Swap,
            OrcaWhirlpoolInstruction::ClosePosition => close_position::ClosePosition,
            OrcaWhirlpoolInstruction::SetDefaultFeeRate => set_default_fee_rate::SetDefaultFeeRate,
            OrcaWhirlpoolInstruction::SetDefaultProtocolFeeRate => set_default_protocol_fee_rate::SetDefaultProtocolFeeRate,
            OrcaWhirlpoolInstruction::SetFeeRate => set_fee_rate::SetFeeRate,
            OrcaWhirlpoolInstruction::SetProtocolFeeRate => set_protocol_fee_rate::SetProtocolFeeRate,
            OrcaWhirlpoolInstruction::SetFeeAuthority => set_fee_authority::SetFeeAuthority,
            OrcaWhirlpoolInstruction::SetCollectProtocolFeesAuthority => set_collect_protocol_fees_authority::SetCollectProtocolFeesAuthority,
            OrcaWhirlpoolInstruction::SetRewardAuthority => set_reward_authority::SetRewardAuthority,
            OrcaWhirlpoolInstruction::SetRewardAuthorityBySuperAuthority => set_reward_authority_by_super_authority::SetRewardAuthorityBySuperAuthority,
            OrcaWhirlpoolInstruction::SetRewardEmissionsSuperAuthority => set_reward_emissions_super_authority::SetRewardEmissionsSuperAuthority,
            OrcaWhirlpoolInstruction::TwoHopSwap => two_hop_swap::TwoHopSwap,
            OrcaWhirlpoolInstruction::InitializePositionBundle => initialize_position_bundle::InitializePositionBundle,
            OrcaWhirlpoolInstruction::InitializePositionBundleWithMetadata => initialize_position_bundle_with_metadata::InitializePositionBundleWithMetadata,
            OrcaWhirlpoolInstruction::DeletePositionBundle => delete_position_bundle::DeletePositionBundle,
            OrcaWhirlpoolInstruction::OpenBundledPosition => open_bundled_position::OpenBundledPosition,
            OrcaWhirlpoolInstruction::CloseBundledPosition => close_bundled_position::CloseBundledPosition,
            OrcaWhirlpoolInstruction::CollectFeesV2 => collect_fees_v2::CollectFeesV2,
            OrcaWhirlpoolInstruction::CollectProtocolFeesV2 => collect_protocol_fees_v2::CollectProtocolFeesV2,
            OrcaWhirlpoolInstruction::CollectRewardV2 => collect_reward_v2::CollectRewardV2,
            OrcaWhirlpoolInstruction::DecreaseLiquidityV2 => decrease_liquidity_v2::DecreaseLiquidityV2,
            OrcaWhirlpoolInstruction::IncreaseLiquidityV2 => increase_liquidity_v2::IncreaseLiquidityV2,
            OrcaWhirlpoolInstruction::InitializePoolV2 => initialize_pool_v2::InitializePoolV2,
            OrcaWhirlpoolInstruction::InitializeRewardV2 => initialize_reward_v2::InitializeRewardV2,
            OrcaWhirlpoolInstruction::SetRewardEmissionsV2 => set_reward_emissions_v2::SetRewardEmissionsV2,
            OrcaWhirlpoolInstruction::SwapV2 => swap_v2::SwapV2,
            OrcaWhirlpoolInstruction::TwoHopSwapV2 => two_hop_swap_v2::TwoHopSwapV2,
            OrcaWhirlpoolInstruction::InitializeConfigExtension => initialize_config_extension::InitializeConfigExtension,
            OrcaWhirlpoolInstruction::SetConfigExtensionAuthority => set_config_extension_authority::SetConfigExtensionAuthority,
            OrcaWhirlpoolInstruction::SetTokenBadgeAuthority => set_token_badge_authority::SetTokenBadgeAuthority,
            OrcaWhirlpoolInstruction::InitializeTokenBadge => initialize_token_badge::InitializeTokenBadge,
            OrcaWhirlpoolInstruction::DeleteTokenBadge => delete_token_badge::DeleteTokenBadge,
        )
    }
}

#[cfg(test)]
mod tests {
    use carbon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
    use solana_instruction::AccountMeta;
    use solana_pubkey::pubkey;

    use crate::{
        instructions::{
            close_bundled_position::{
                CloseBundledPosition, CloseBundledPositionInstructionAccounts,
            },
            close_position::{ClosePosition, ClosePositionInstructionAccounts},
            collect_fees::{CollectFees, CollectFeesInstructionAccounts},
            collect_fees_v2::{CollectFeesV2, CollectFeesV2InstructionAccounts},
            collect_protocol_fees::{CollectProtocolFees, CollectProtocolFeesInstructionAccounts},
            collect_protocol_fees_v2::{
                CollectProtocolFeesV2, CollectProtocolFeesV2InstructionAccounts,
            },
            collect_reward::{CollectReward, CollectRewardInstructionAccounts},
            collect_reward_v2::{CollectRewardV2, CollectRewardV2InstructionAccounts},
            decrease_liquidity::{DecreaseLiquidity, DecreaseLiquidityInstructionAccounts},
            decrease_liquidity_v2::{DecreaseLiquidityV2, DecreaseLiquidityV2InstructionAccounts},
            delete_position_bundle::{
                DeletePositionBundle, DeletePositionBundleInstructionAccounts,
            },
            increase_liquidity::{IncreaseLiquidity, IncreaseLiquidityInstructionAccounts},
            increase_liquidity_v2::{IncreaseLiquidityV2, IncreaseLiquidityV2InstructionAccounts},
            initialize_config_extension::{
                InitializeConfigExtension, InitializeConfigExtensionInstructionAccounts,
            },
            initialize_fee_tier::{InitializeFeeTier, InitializeFeeTierInstructionAccounts},
            initialize_pool::{InitializePool, InitializePoolInstructionAccounts},
            initialize_pool_v2::{InitializePoolV2, InitializePoolV2InstructionAccounts},
            initialize_position_bundle::{
                InitializePositionBundle, InitializePositionBundleInstructionAccounts,
            },
            initialize_position_bundle_with_metadata::{
                InitializePositionBundleWithMetadata,
                InitializePositionBundleWithMetadataInstructionAccounts,
            },
            initialize_reward_v2::{InitializeRewardV2, InitializeRewardV2InstructionAccounts},
            initialize_tick_array::{InitializeTickArray, InitializeTickArrayInstructionAccounts},
            initialize_token_badge::{
                InitializeTokenBadge, InitializeTokenBadgeInstructionAccounts,
            },
            open_bundled_position::{OpenBundledPosition, OpenBundledPositionInstructionAccounts},
            open_position::{OpenPosition, OpenPositionInstructionAccounts},
            open_position_with_metadata::{
                OpenPositionWithMetadata, OpenPositionWithMetadataInstructionAccounts,
            },
            set_collect_protocol_fees_authority::{
                SetCollectProtocolFeesAuthority, SetCollectProtocolFeesAuthorityInstructionAccounts,
            },
            set_reward_emissions_v2::{
                SetRewardEmissionsV2, SetRewardEmissionsV2InstructionAccounts,
            },
            swap::{Swap, SwapInstructionAccounts},
            swap_v2::{SwapV2, SwapV2InstructionAccounts},
            two_hop_swap::{TwoHopSwap, TwoHopSwapInstructionAccounts},
            two_hop_swap_v2::{TwoHopSwapV2, TwoHopSwapV2InstructionAccounts},
            update_fees_and_rewards::{
                UpdateFeesAndRewards, UpdateFeesAndRewardsInstructionAccounts,
            },
        },
        types::{OpenPositionBumps, OpenPositionWithMetadataBumps, WhirlpoolBumps},
    };

    use super::*;

    #[test]
    fn test_decode_close_bundled_position_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::CloseBundledPosition(CloseBundledPosition {
            bundle_index: 0,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("FSXUZE8ZiRmwBJqghEjKmENqU1JaSnVcxaYc7mDL2YVc"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Ek7JsLsfsWLspYKF6Rx9xrez16kYrRfo59kfWg6mu6ye"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("GpqwMcGqb8zc7CUUYWtGyDKeCwmHXnSj9z2KE9wWM9fF"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6Pg1AsSovedLHBgtAzm4i3A7AoZDTo28qkV9U46Zj8iA"),
                true,
            ),
            AccountMeta::new(
                pubkey!("6Pg1AsSovedLHBgtAzm4i3A7AoZDTo28qkV9U46Zj8iA"),
                true,
            ),
        ];
        let expected_arranged_accounts = CloseBundledPositionInstructionAccounts {
            bundled_position: pubkey!("FSXUZE8ZiRmwBJqghEjKmENqU1JaSnVcxaYc7mDL2YVc"),
            position_bundle: pubkey!("Ek7JsLsfsWLspYKF6Rx9xrez16kYrRfo59kfWg6mu6ye"),
            position_bundle_token_account: pubkey!("GpqwMcGqb8zc7CUUYWtGyDKeCwmHXnSj9z2KE9wWM9fF"),
            position_bundle_authority: pubkey!("6Pg1AsSovedLHBgtAzm4i3A7AoZDTo28qkV9U46Zj8iA"),
            receiver: pubkey!("6Pg1AsSovedLHBgtAzm4i3A7AoZDTo28qkV9U46Zj8iA"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/close_bundled_position_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            CloseBundledPosition::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_close_position_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::ClosePosition(ClosePosition {});
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("8YbKYJyh4Xe7PWoVdCgZapW6vHwf4saKNaMBfZhuHFUs"),
                false,
            ),
            AccountMeta::new(pubkey!("dche7M2764e8AxNihBdn7uffVzZvTBNeL8x4LZg5E2c"), true),
            AccountMeta::new(
                pubkey!("3c5GDpDJHTjrK4sVWAVMfwoN17V6oBZEpQD4Z8vg5Ea5"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7VyJ7CuxP7CZAipZgHQ4dnDuRmkzCZtr7BgxHKmhV14f"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GsgUebzMur2DakcY4sN2RM4E1LWLB4x7SGB5dhdnii6R"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
        ];
        let expected_arranged_accounts = ClosePositionInstructionAccounts {
            position_authority: pubkey!("8YbKYJyh4Xe7PWoVdCgZapW6vHwf4saKNaMBfZhuHFUs"),
            receiver: pubkey!("dche7M2764e8AxNihBdn7uffVzZvTBNeL8x4LZg5E2c"),
            position: pubkey!("3c5GDpDJHTjrK4sVWAVMfwoN17V6oBZEpQD4Z8vg5Ea5"),
            position_mint: pubkey!("7VyJ7CuxP7CZAipZgHQ4dnDuRmkzCZtr7BgxHKmhV14f"),
            position_token_account: pubkey!("GsgUebzMur2DakcY4sN2RM4E1LWLB4x7SGB5dhdnii6R"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/close_position_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            ClosePosition::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_collect_fees_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::CollectFees(CollectFees {});
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("GDL9697QALucFPpMt2q9xJKYQsqYYsXmNt2jSii8g3Fa"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EjGi7MvfXYeumngY4makPAkKdrshZ98aXZPbGyf2maM6"),
                true,
            ),
            AccountMeta::new(
                pubkey!("6nToW8ErR2K3op2tyjcLG2bEKWsBFW3W2HLVdiEs3T9p"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("2NaKgz8WPtxtzDwHPZ2gnRmNQd3qBx51Uyji1hqTc72U"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EzwAfPbWcTR2WTPEdfzpsDLWY3hbeLFqiWBnhk64AKDd"),
                true,
            ),
            AccountMeta::new(
                pubkey!("BrNQmGP4FJ88LMA4PE5ecR75L2gxRi3JcvdtiBd58nnn"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AeEHBAYv9g4iBNyipSqB4PmpXCnXUbieNw9pVN5J4oZu"),
                false,
            ),
            AccountMeta::new(
                pubkey!("71hf61PcLXRBQQ2jn8C7nFM1FrRk6eYmtCNkqrL2my5M"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
        ];
        let expected_arranged_accounts = CollectFeesInstructionAccounts {
            whirlpool: pubkey!("GDL9697QALucFPpMt2q9xJKYQsqYYsXmNt2jSii8g3Fa"),
            position_authority: pubkey!("EjGi7MvfXYeumngY4makPAkKdrshZ98aXZPbGyf2maM6"),
            position: pubkey!("6nToW8ErR2K3op2tyjcLG2bEKWsBFW3W2HLVdiEs3T9p"),
            position_token_account: pubkey!("2NaKgz8WPtxtzDwHPZ2gnRmNQd3qBx51Uyji1hqTc72U"),
            token_owner_account_a: pubkey!("EzwAfPbWcTR2WTPEdfzpsDLWY3hbeLFqiWBnhk64AKDd"),
            token_vault_a: pubkey!("BrNQmGP4FJ88LMA4PE5ecR75L2gxRi3JcvdtiBd58nnn"),
            token_owner_account_b: pubkey!("AeEHBAYv9g4iBNyipSqB4PmpXCnXUbieNw9pVN5J4oZu"),
            token_vault_b: pubkey!("71hf61PcLXRBQQ2jn8C7nFM1FrRk6eYmtCNkqrL2my5M"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/collect_fees_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            CollectFees::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_collect_fees_v2_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::CollectFeesV2(CollectFeesV2 {
            remaining_accounts_info: Default::default(),
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("C9U2Ksk6KKWvLEeo5yUQ7Xu46X7NzeBJtd9PBfuXaUSM"),
                false,
            ),
            AccountMeta::new(
                pubkey!("G5DKvpnqv2cJnVp9eMWNB2214Ty8K5zLFCobLDcHd3QP"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AozvWPVaRK4J8RK3m4uQ1CZuMmrmCNy24ZQaHU6kWbig"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("77qeyXbUnrTApvGfQV9b5dJgzQkKsWhgSKYjQ8HnSLmk"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("9BB6NFEcjBCtnNLFko2FqVQBq8HHM13kCyYcdQbgpump"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3DiHij3ENtrxR319CiDTHXwBWs4oN4aSZ6uxTHgeJds6"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2z2dj8yyRmXP8bwP8LqmxeZGVtuAGv4g2wgzp2Rz1QmJ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("oQy5Q2gnmNjsM1Fy86wMsaeMyVj7A8NU7RtR7Ai7j97"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5ex8weR6VjuwTrE4aP7juXcGWPTiD9PJeMZiwZZderqc"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
        ];
        let expected_arranged_accounts = CollectFeesV2InstructionAccounts {
            whirlpool: pubkey!("C9U2Ksk6KKWvLEeo5yUQ7Xu46X7NzeBJtd9PBfuXaUSM"),
            position_authority: pubkey!("G5DKvpnqv2cJnVp9eMWNB2214Ty8K5zLFCobLDcHd3QP"),
            position: pubkey!("AozvWPVaRK4J8RK3m4uQ1CZuMmrmCNy24ZQaHU6kWbig"),
            position_token_account: pubkey!("77qeyXbUnrTApvGfQV9b5dJgzQkKsWhgSKYjQ8HnSLmk"),
            token_mint_a: pubkey!("So11111111111111111111111111111111111111112"),
            token_mint_b: pubkey!("9BB6NFEcjBCtnNLFko2FqVQBq8HHM13kCyYcdQbgpump"),
            token_owner_account_a: pubkey!("3DiHij3ENtrxR319CiDTHXwBWs4oN4aSZ6uxTHgeJds6"),
            token_vault_a: pubkey!("2z2dj8yyRmXP8bwP8LqmxeZGVtuAGv4g2wgzp2Rz1QmJ"),
            token_owner_account_b: pubkey!("oQy5Q2gnmNjsM1Fy86wMsaeMyVj7A8NU7RtR7Ai7j97"),
            token_vault_b: pubkey!("5ex8weR6VjuwTrE4aP7juXcGWPTiD9PJeMZiwZZderqc"),
            token_program_a: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program_b: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/collect_fees_v2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            CollectFeesV2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_collect_protocol_fees_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::CollectProtocolFees(CollectProtocolFees {});
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("HDE78Mg7ukJht34qd9yifnDGkdZoAFu5K5dSGQiHDDRq"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EEEEBfiPZbNDNnzwAgJJSHqsWz7u4iuDfTWfpzfZqgzT"),
                false,
            ),
            AccountMeta::new(
                pubkey!("89VB5UmvopuCFmp5Mf8YPX28fGvvqn79afCgouQuPyhY"),
                true,
            ),
            AccountMeta::new(
                pubkey!("4YRsbqv8PL7XtJspEHswDtm7YYyvmpvCQ4EjtijmmQk9"),
                false,
            ),
            AccountMeta::new(
                pubkey!("39XJsPJStL3H6QVsCqvENFJ4QPXSKmoSvcuTbhC4nnyk"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5K9KFLPwkskr3V5mrHfENE8EGYa85heKfW9YpCMowNJC"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3RCQzb4yyWPb99L99C16uokdV8ktmeEQnLsTF2S7wABz"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
        ];
        let expected_arranged_accounts = CollectProtocolFeesInstructionAccounts {
            whirlpools_config: pubkey!("HDE78Mg7ukJht34qd9yifnDGkdZoAFu5K5dSGQiHDDRq"),
            whirlpool: pubkey!("EEEEBfiPZbNDNnzwAgJJSHqsWz7u4iuDfTWfpzfZqgzT"),
            collect_protocol_fees_authority: pubkey!(
                "89VB5UmvopuCFmp5Mf8YPX28fGvvqn79afCgouQuPyhY"
            ),
            token_vault_a: pubkey!("4YRsbqv8PL7XtJspEHswDtm7YYyvmpvCQ4EjtijmmQk9"),
            token_vault_b: pubkey!("39XJsPJStL3H6QVsCqvENFJ4QPXSKmoSvcuTbhC4nnyk"),
            token_destination_a: pubkey!("5K9KFLPwkskr3V5mrHfENE8EGYa85heKfW9YpCMowNJC"),
            token_destination_b: pubkey!("3RCQzb4yyWPb99L99C16uokdV8ktmeEQnLsTF2S7wABz"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/collect_protocol_fees_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            CollectProtocolFees::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_collect_protocol_fees_v2_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::CollectProtocolFeesV2(CollectProtocolFeesV2 {
            remaining_accounts_info: Default::default(),
        });
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9gPd7Khu5bX3SyrXm5qTVrhGxuvMNzDKexsdGRfiwUMt"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CTgympSD3hkBhH2XYWfGCigrFh3ZHAvnxwAfm7nCbExY"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("WENWENvqqNya429ubCdR81ZmD69brwQaaBYY6p3LCpk"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("GENZexWRRGNS2Ko5rEgGG1snRXpaa3CDDGYnhTSmE3kd"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HXY8ATP7bDY7yWuGgc9dsRArCVan34PkmTszzEZ8YUNN"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6uRrG8etue7oR9UX5Vm65DSR1xF1Znm9eq4LjNMvnyhg"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4ATUUYSoVR9uPmhWtuhzst3RCoi7hauvQPEXm3oMULdB"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EVX1HdMb1tofjgKKn6qayjs7SywURQLXgrci4NTdbepA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
        ];
        let expected_arranged_accounts = CollectProtocolFeesV2InstructionAccounts {
            whirlpools_config: pubkey!("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ"),
            whirlpool: pubkey!("9gPd7Khu5bX3SyrXm5qTVrhGxuvMNzDKexsdGRfiwUMt"),
            collect_protocol_fees_authority: pubkey!(
                "CTgympSD3hkBhH2XYWfGCigrFh3ZHAvnxwAfm7nCbExY"
            ),
            token_mint_a: pubkey!("WENWENvqqNya429ubCdR81ZmD69brwQaaBYY6p3LCpk"),
            token_mint_b: pubkey!("GENZexWRRGNS2Ko5rEgGG1snRXpaa3CDDGYnhTSmE3kd"),
            token_vault_a: pubkey!("HXY8ATP7bDY7yWuGgc9dsRArCVan34PkmTszzEZ8YUNN"),
            token_vault_b: pubkey!("6uRrG8etue7oR9UX5Vm65DSR1xF1Znm9eq4LjNMvnyhg"),
            token_destination_a: pubkey!("4ATUUYSoVR9uPmhWtuhzst3RCoi7hauvQPEXm3oMULdB"),
            token_destination_b: pubkey!("EVX1HdMb1tofjgKKn6qayjs7SywURQLXgrci4NTdbepA"),
            token_program_a: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program_b: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/collect_protocol_fees_v2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            CollectProtocolFeesV2::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_collect_reward_ix() {
        // Arrange
        let expected_ix =
            OrcaWhirlpoolInstruction::CollectReward(CollectReward { reward_index: 0 });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE"),
                false,
            ),
            AccountMeta::new(
                pubkey!("D5g9NYqa8Pzu8FRHDMkAzKotai7NdVTG51xLL1Esq8z4"),
                true,
            ),
            AccountMeta::new(
                pubkey!("AvjzPjG3XfjAEKDHc3GtmvyspuUq1bQ3EuMGzmjspVag"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("F8DpS1w93ZtSN1PBMTrA7G1UEvrabcYYix8GKCEDuhyJ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9RWazJxyz1bLC3Au9T4Kd9pqD6NsRfjLxKEapNUKLaCD"),
                false,
            ),
            AccountMeta::new(
                pubkey!("21yVeQh9dMDFxcgAxo7pSKBiBUeb2UmHgrf9fobEX4x9"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
        ];
        let expected_arranged_accounts = CollectRewardInstructionAccounts {
            whirlpool: pubkey!("Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE"),
            position_authority: pubkey!("D5g9NYqa8Pzu8FRHDMkAzKotai7NdVTG51xLL1Esq8z4"),
            position: pubkey!("AvjzPjG3XfjAEKDHc3GtmvyspuUq1bQ3EuMGzmjspVag"),
            position_token_account: pubkey!("F8DpS1w93ZtSN1PBMTrA7G1UEvrabcYYix8GKCEDuhyJ"),
            reward_owner_account: pubkey!("9RWazJxyz1bLC3Au9T4Kd9pqD6NsRfjLxKEapNUKLaCD"),
            reward_vault: pubkey!("21yVeQh9dMDFxcgAxo7pSKBiBUeb2UmHgrf9fobEX4x9"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/collect_reward_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            CollectReward::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_collect_reward_v2_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::CollectRewardV2(CollectRewardV2 {
            remaining_accounts_info: Default::default(),
            reward_index: 0,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("5zpyutJu9ee6jFymDGoK7F6S5Kczqtc9FomP3ueKuyA9"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HJ8tXpJA62BHbEQyoXnmLZz1Tifek9WVM3GTrzd4NyUa"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4aQCMsowSNTqWQq5sbZbF3GBBUhzPkyLYsbdKUHXVory"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("D7TpS9Tj7mva9hK71y9MQfGow2WFRkcroDw6t1zfbUpk"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HxPt8A4GDq7SBRnp4Seyau1Pf17KiH2633Wi9svoRRbi"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AfHnr4bRa7jgNnq9PfkwdMDKeiwFwSWqvJTnqqN6T3kq"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
        ];
        let expected_arranged_accounts = CollectRewardV2InstructionAccounts {
            whirlpool: pubkey!("5zpyutJu9ee6jFymDGoK7F6S5Kczqtc9FomP3ueKuyA9"),
            position_authority: pubkey!("HJ8tXpJA62BHbEQyoXnmLZz1Tifek9WVM3GTrzd4NyUa"),
            position: pubkey!("4aQCMsowSNTqWQq5sbZbF3GBBUhzPkyLYsbdKUHXVory"),
            position_token_account: pubkey!("D7TpS9Tj7mva9hK71y9MQfGow2WFRkcroDw6t1zfbUpk"),
            reward_owner_account: pubkey!("HxPt8A4GDq7SBRnp4Seyau1Pf17KiH2633Wi9svoRRbi"),
            reward_mint: pubkey!("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"),
            reward_vault: pubkey!("AfHnr4bRa7jgNnq9PfkwdMDKeiwFwSWqvJTnqqN6T3kq"),
            reward_token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/collect_reward_v2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            CollectRewardV2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_decrease_liquidity_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::DecreaseLiquidity(DecreaseLiquidity {
            liquidity_amount: 6728487368,
            token_min_a: 859639789,
            token_min_b: 742325562,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("57mP5WoNrg3uiGFUdoeYr2CPUZak1L2ZgFtyFwoT7K6G"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CjssRwVNrR6zfzbgiLJEBDHj31ejLuikkZUmQdTqr1KZ"),
                true,
            ),
            AccountMeta::new(
                pubkey!("5yQL7jFFbteXJ7SyTow2MZ96garFk5JqrxMJci5yUTBU"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7bk8YqiE5dK46AxTNfGmaLNhnrCBgSwqneE6xYKj38P9"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2tG779cf9WopoqukfGYbGcAUWDmQGZj2u5o5ApV53bpk"),
                true,
            ),
            AccountMeta::new(
                pubkey!("4AWhobuZKUeoNg738vJaR8VxnATWSF8faNXDKsBg2STA"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CcwLMXxRLaaf1biHSaXCckQB85xyq3U7GRo3iiqCV74H"),
                false,
            ),
            AccountMeta::new(
                pubkey!("79Lv5tG6n74sRJFLjrXxwqBdNmFv8ERYQZ1WiSUbCDU4"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GJaRuVms5122pL7pncdSNxoKPVNckHjupU9EjjLCSeCg"),
                false,
            ),
            AccountMeta::new(
                pubkey!("93NMWh9DJuR9S3ys8VRgAjZtqWubtuPVR94NTCXYwp7m"),
                false,
            ),
        ];
        let expected_arranged_accounts = DecreaseLiquidityInstructionAccounts {
            whirlpool: pubkey!("57mP5WoNrg3uiGFUdoeYr2CPUZak1L2ZgFtyFwoT7K6G"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            position_authority: pubkey!("CjssRwVNrR6zfzbgiLJEBDHj31ejLuikkZUmQdTqr1KZ"),
            position: pubkey!("5yQL7jFFbteXJ7SyTow2MZ96garFk5JqrxMJci5yUTBU"),
            position_token_account: pubkey!("7bk8YqiE5dK46AxTNfGmaLNhnrCBgSwqneE6xYKj38P9"),
            token_owner_account_a: pubkey!("2tG779cf9WopoqukfGYbGcAUWDmQGZj2u5o5ApV53bpk"),
            token_owner_account_b: pubkey!("4AWhobuZKUeoNg738vJaR8VxnATWSF8faNXDKsBg2STA"),
            token_vault_a: pubkey!("CcwLMXxRLaaf1biHSaXCckQB85xyq3U7GRo3iiqCV74H"),
            token_vault_b: pubkey!("79Lv5tG6n74sRJFLjrXxwqBdNmFv8ERYQZ1WiSUbCDU4"),
            tick_array_lower: pubkey!("GJaRuVms5122pL7pncdSNxoKPVNckHjupU9EjjLCSeCg"),
            tick_array_upper: pubkey!("93NMWh9DJuR9S3ys8VRgAjZtqWubtuPVR94NTCXYwp7m"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/decrease_liquidity_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            DecreaseLiquidity::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_decrease_liquidity_v2_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::DecreaseLiquidityV2(DecreaseLiquidityV2 {
            liquidity_amount: 33429762919,
            remaining_accounts_info: None,
            token_min_a: 0,
            token_min_b: 341312282,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Gs3WwVr2pHL3z2C2ruD2UEL1Vd6rCs8tMd7PTveYVcfT"),
                true,
            ),
            AccountMeta::new(
                pubkey!("6VKUNDe19Gsg896kd7TFfYdnMBMWhQM6PcoCL1wRhid7"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4xYVtnApUf9FPUjxcGXPDKL2umfdaJehuYTKegspXeVr"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FY1k7XwkR6Hn6bkpnKooNVAK51abJYf2QZycradGjoD8"),
                true,
            ),
            AccountMeta::new(
                pubkey!("8fZK6UNp6MNeqW8EBx8zdibEMRD9HJ9FqKUMtzPE38ad"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EUuUbDcafPrmVTD5M6qoJAoyyNbihBhugADAxRMn5he9"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2WLWEuKDgkDUccTpbwYp1GToYktiSB1cXvreHUwiSUVP"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DAJjGxPKTEBcDdQqojLuN1nEpKNcAiowoqguHb2AA7dy"),
                false,
            ),
            AccountMeta::new(
                pubkey!("ANkk7h9Ak8o5tkYhc5tZzw8TCxfvGrnpJ6ht26roh49a"),
                false,
            ),
        ];
        let expected_arranged_accounts = DecreaseLiquidityV2InstructionAccounts {
            whirlpool: pubkey!("Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE"),
            token_program_a: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program_b: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
            position_authority: pubkey!("Gs3WwVr2pHL3z2C2ruD2UEL1Vd6rCs8tMd7PTveYVcfT"),
            position: pubkey!("6VKUNDe19Gsg896kd7TFfYdnMBMWhQM6PcoCL1wRhid7"),
            position_token_account: pubkey!("4xYVtnApUf9FPUjxcGXPDKL2umfdaJehuYTKegspXeVr"),
            token_mint_a: pubkey!("So11111111111111111111111111111111111111112"),
            token_mint_b: pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
            token_owner_account_a: pubkey!("FY1k7XwkR6Hn6bkpnKooNVAK51abJYf2QZycradGjoD8"),
            token_owner_account_b: pubkey!("8fZK6UNp6MNeqW8EBx8zdibEMRD9HJ9FqKUMtzPE38ad"),
            token_vault_a: pubkey!("EUuUbDcafPrmVTD5M6qoJAoyyNbihBhugADAxRMn5he9"),
            token_vault_b: pubkey!("2WLWEuKDgkDUccTpbwYp1GToYktiSB1cXvreHUwiSUVP"),
            tick_array_lower: pubkey!("DAJjGxPKTEBcDdQqojLuN1nEpKNcAiowoqguHb2AA7dy"),
            tick_array_upper: pubkey!("ANkk7h9Ak8o5tkYhc5tZzw8TCxfvGrnpJ6ht26roh49a"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/decrease_liquidity_v2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            DecreaseLiquidityV2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_delete_position_bundle_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::DeletePositionBundle(DeletePositionBundle {});
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("Hx4EKx2VJH4WPtkafQNRR6x9rLWvCmpf6SGVkLKg9wqi"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Fr9kvQC7nPPQuVdWdUWMNf2SQJn6DMNK28K4KajCe7T6"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3pLvy7CmzMrZdxJfvTV8CgUcJgC5AH4KKcLJYEgMuVjf"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GNB1zCnnmXEC5Vz22PyyGzkp2nPjDwDAfKBctBwjCLDD"),
                true,
            ),
            AccountMeta::new(
                pubkey!("GNB1zCnnmXEC5Vz22PyyGzkp2nPjDwDAfKBctBwjCLDD"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
        ];
        let expected_arranged_accounts = DeletePositionBundleInstructionAccounts {
            position_bundle: pubkey!("Hx4EKx2VJH4WPtkafQNRR6x9rLWvCmpf6SGVkLKg9wqi"),
            position_bundle_mint: pubkey!("Fr9kvQC7nPPQuVdWdUWMNf2SQJn6DMNK28K4KajCe7T6"),
            position_bundle_token_account: pubkey!("3pLvy7CmzMrZdxJfvTV8CgUcJgC5AH4KKcLJYEgMuVjf"),
            position_bundle_owner: pubkey!("GNB1zCnnmXEC5Vz22PyyGzkp2nPjDwDAfKBctBwjCLDD"),
            receiver: pubkey!("GNB1zCnnmXEC5Vz22PyyGzkp2nPjDwDAfKBctBwjCLDD"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/delete_position_bundle_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            DeletePositionBundle::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_increase_liquidity_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::IncreaseLiquidity(IncreaseLiquidity {
            liquidity_amount: 22561146,
            token_max_a: 7709,
            token_max_b: 16093721672,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("Q63pRd5cCFkYLkze6nSJhoRSd3ZFtHWra7a2HfGijQo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FVgatbrhoA1soBNy7icnEMsNoYuzk3Uarp6qTp74mokC"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BePKmc9QmueEpZfxg7rezgrJPAJb47wkUPGnY2KT3Zxb"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6tmiEFgYiWegUzwNAma3PB66fFXYphhypoD8mZmDZdVQ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DBLgWRZNnMTbYAj6Y3tKhMvAVSEUqkbensZN3EQae3uj"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4eC12KAx2nBW6eq89iCd2WaQobxaJjbKZiVuiFLxTkV7"),
                false,
            ),
            AccountMeta::new(
                pubkey!("uRBAwb7w2Qdu4iLvwxbzFX6KciEuunxVmHwaQPCVRU8"),
                false,
            ),
            AccountMeta::new(
                pubkey!("66GvEgxNeQYuUVErTrKd3dUUUMNB57kKHJG2H5obB7Xm"),
                false,
            ),
            AccountMeta::new(
                pubkey!("99MCmD4KWtUgayDUfahJiCd3uyYytFT81EtHEE398MhY"),
                false,
            ),
            AccountMeta::new(
                pubkey!("47t1m6uFFwx6a7S9Yvb9j29b7yAv9eKeNk65YUeHiKwt"),
                false,
            ),
        ];
        let expected_arranged_accounts = IncreaseLiquidityInstructionAccounts {
            whirlpool: pubkey!("Q63pRd5cCFkYLkze6nSJhoRSd3ZFtHWra7a2HfGijQo"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            position_authority: pubkey!("FVgatbrhoA1soBNy7icnEMsNoYuzk3Uarp6qTp74mokC"),
            position: pubkey!("BePKmc9QmueEpZfxg7rezgrJPAJb47wkUPGnY2KT3Zxb"),
            position_token_account: pubkey!("6tmiEFgYiWegUzwNAma3PB66fFXYphhypoD8mZmDZdVQ"),
            token_owner_account_a: pubkey!("DBLgWRZNnMTbYAj6Y3tKhMvAVSEUqkbensZN3EQae3uj"),
            token_owner_account_b: pubkey!("4eC12KAx2nBW6eq89iCd2WaQobxaJjbKZiVuiFLxTkV7"),
            token_vault_a: pubkey!("uRBAwb7w2Qdu4iLvwxbzFX6KciEuunxVmHwaQPCVRU8"),
            token_vault_b: pubkey!("66GvEgxNeQYuUVErTrKd3dUUUMNB57kKHJG2H5obB7Xm"),
            tick_array_lower: pubkey!("99MCmD4KWtUgayDUfahJiCd3uyYytFT81EtHEE398MhY"),
            tick_array_upper: pubkey!("47t1m6uFFwx6a7S9Yvb9j29b7yAv9eKeNk65YUeHiKwt"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/increase_liquidity_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            IncreaseLiquidity::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_increase_liquidity_v2_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::IncreaseLiquidityV2(IncreaseLiquidityV2 {
            liquidity_amount: 119303511311,
            remaining_accounts_info: None,
            token_max_a: 18446744073709551615,
            token_max_b: 18446744073709551615,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("AHTTzwf3GmVMJdxWM8v2MSxyjZj8rQR6hyAC3g9477Yj"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
            AccountMeta::new(
                pubkey!("33niXfecHme6jgZ18MDfiuAEfGbV797c5vKLsVboFqUB"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7mCEF32rYgGVZTiP4SNyeGk8uYq2SpxdJU8WN6EEeY6M"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("DubWoqvxsB7J6A9ZjqVRm74BS6CWWAAre4rzLZJZpYjG"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Bkym9Buc5UZ3Y3NEohxnxCbAbLMzjeUMc3ENu4h4PG6Q"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7KQP7YXe51Uw3bxYyLzAca8xSh4DRiuCFMJ4NdiFn6Cy"),
                false,
            ),
            AccountMeta::new(
                pubkey!("ATUMydDvNcELzNk9GP1Ky7i2Mgx2t2ej5aNPMhA6F2VH"),
                false,
            ),
            AccountMeta::new(
                pubkey!("ChcWkmUbWDbBspDjPX6ZXi7Hb9kZ7VTbNUf6nMtWF1YH"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9hcRuY6mg7EhCDzbgKoU8oz7xXTJNgY6dsgNF8fVxnqp"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CczSTD94CGWLTKGNyzKVTpiYft1ZL6eQxVi32CwhaCJc"),
                false,
            ),
        ];
        let expected_arranged_accounts = IncreaseLiquidityV2InstructionAccounts {
            whirlpool: pubkey!("AHTTzwf3GmVMJdxWM8v2MSxyjZj8rQR6hyAC3g9477Yj"),
            token_program_a: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program_b: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
            position_authority: pubkey!("33niXfecHme6jgZ18MDfiuAEfGbV797c5vKLsVboFqUB"),
            position: pubkey!("7mCEF32rYgGVZTiP4SNyeGk8uYq2SpxdJU8WN6EEeY6M"),
            position_token_account: pubkey!("DubWoqvxsB7J6A9ZjqVRm74BS6CWWAAre4rzLZJZpYjG"),
            token_mint_a: pubkey!("So11111111111111111111111111111111111111112"),
            token_mint_b: pubkey!("7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr"),
            token_owner_account_a: pubkey!("Bkym9Buc5UZ3Y3NEohxnxCbAbLMzjeUMc3ENu4h4PG6Q"),
            token_owner_account_b: pubkey!("7KQP7YXe51Uw3bxYyLzAca8xSh4DRiuCFMJ4NdiFn6Cy"),
            token_vault_a: pubkey!("ATUMydDvNcELzNk9GP1Ky7i2Mgx2t2ej5aNPMhA6F2VH"),
            token_vault_b: pubkey!("ChcWkmUbWDbBspDjPX6ZXi7Hb9kZ7VTbNUf6nMtWF1YH"),
            tick_array_lower: pubkey!("9hcRuY6mg7EhCDzbgKoU8oz7xXTJNgY6dsgNF8fVxnqp"),
            tick_array_upper: pubkey!("CczSTD94CGWLTKGNyzKVTpiYft1ZL6eQxVi32CwhaCJc"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/increase_liquidity_v2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            IncreaseLiquidityV2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_config_extension_ix() {
        // Arrange
        let expected_ix =
            OrcaWhirlpoolInstruction::InitializeConfigExtension(InitializeConfigExtension {});
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("ET8gW61Ru2Fgi4Nyvm4uQHeah7v8QMhnXnrgXxN2Y2s9"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4nvE42kKB3VRLzRxCw3ikjCk6WC6WHy7vQ5EJG3JmrXd"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6Pg1AsSovedLHBgtAzm4i3A7AoZDTo28qkV9U46Zj8iA"),
                true,
            ),
            AccountMeta::new(
                pubkey!("6Pg1AsSovedLHBgtAzm4i3A7AoZDTo28qkV9U46Zj8iA"),
                true,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
        ];
        let expected_arranged_accounts = InitializeConfigExtensionInstructionAccounts {
            config: pubkey!("ET8gW61Ru2Fgi4Nyvm4uQHeah7v8QMhnXnrgXxN2Y2s9"),
            config_extension: pubkey!("4nvE42kKB3VRLzRxCw3ikjCk6WC6WHy7vQ5EJG3JmrXd"),
            funder: pubkey!("6Pg1AsSovedLHBgtAzm4i3A7AoZDTo28qkV9U46Zj8iA"),
            fee_authority: pubkey!("6Pg1AsSovedLHBgtAzm4i3A7AoZDTo28qkV9U46Zj8iA"),
            system_program: pubkey!("11111111111111111111111111111111"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/initialize_config_extension_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializeConfigExtension::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_fee_tier_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::InitializeFeeTier(InitializeFeeTier {
            default_fee_rate: 0,
            tick_spacing: 256,
        });
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("DUAyeevUnXv1ePfyRJby7fdBqhezEdvXZonFnQa6j6fM"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2qX12HjWr4b4LYc7oAQfmst5drcmNCUZxs5c3jz2e3Tv"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DDGbigvGH49guY7PaaSdaNd3FiF5fyMzywzkF35bjZz7"),
                true,
            ),
            AccountMeta::new(
                pubkey!("DDGbigvGH49guY7PaaSdaNd3FiF5fyMzywzkF35bjZz7"),
                true,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
        ];
        let expected_arranged_accounts = InitializeFeeTierInstructionAccounts {
            config: pubkey!("DUAyeevUnXv1ePfyRJby7fdBqhezEdvXZonFnQa6j6fM"),
            fee_tier: pubkey!("2qX12HjWr4b4LYc7oAQfmst5drcmNCUZxs5c3jz2e3Tv"),
            funder: pubkey!("DDGbigvGH49guY7PaaSdaNd3FiF5fyMzywzkF35bjZz7"),
            fee_authority: pubkey!("DDGbigvGH49guY7PaaSdaNd3FiF5fyMzywzkF35bjZz7"),
            system_program: pubkey!("11111111111111111111111111111111"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/initialize_fee_tier_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializeFeeTier::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_pool_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::InitializePool(InitializePool {
            bumps: WhirlpoolBumps {
                whirlpool_bump: 246,
            },
            initial_sqrt_price: 1304381782533278221200000,
            tick_spacing: 32896,
        });
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("DzkKGBw4njGApBJ4Bd2csCpm4rsdgEmkgivR7XZvENPv"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EDttsTWsed4QS7Qum19PvJvB7om9N321tk3dhnYxKb94"),
                true,
            ),
            AccountMeta::new(
                pubkey!("CGGNcohZdLdeDBdhmQRGmUH1Viv1p4d1ds2aPLoiVWaR"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6cS7s6TZuQofs1TPYSpZ51qGPQKnrxk6xWhW131MysXh"),
                true,
            ),
            AccountMeta::new(
                pubkey!("25tSGzR5NcRpuoWKzWFTWfMaB1ijiaHP6HkqjxZNH72m"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("zVmMsL5qGh7txhTHFgGZcFQpSsxSx6DBLJ3u113PBer"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
        ];
        let expected_arranged_accounts = InitializePoolInstructionAccounts {
            whirlpools_config: pubkey!("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ"),
            token_mint_a: pubkey!("So11111111111111111111111111111111111111112"),
            token_mint_b: pubkey!("DzkKGBw4njGApBJ4Bd2csCpm4rsdgEmkgivR7XZvENPv"),
            funder: pubkey!("EDttsTWsed4QS7Qum19PvJvB7om9N321tk3dhnYxKb94"),
            whirlpool: pubkey!("CGGNcohZdLdeDBdhmQRGmUH1Viv1p4d1ds2aPLoiVWaR"),
            token_vault_a: pubkey!("6cS7s6TZuQofs1TPYSpZ51qGPQKnrxk6xWhW131MysXh"),
            token_vault_b: pubkey!("25tSGzR5NcRpuoWKzWFTWfMaB1ijiaHP6HkqjxZNH72m"),
            fee_tier: pubkey!("zVmMsL5qGh7txhTHFgGZcFQpSsxSx6DBLJ3u113PBer"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/initialize_pool_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializePool::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_pool_v2_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::InitializePoolV2(InitializePoolV2 {
            initial_sqrt_price: 313696632313979986050,
            tick_spacing: 32896,
        });
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("3z2ruynV5UfjqHecqHcm81LseA4Bz5pVszSrYzeanqKf"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Ey59PH7Z4BFU4HjyKnyMdWt5GGN76KazTAwQihoUXRnk"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("GATEWkJdo18j4mWYvzhidadAWSuA9RLkwARcnofH5W4B"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("GaDyFDz4rWd7fwbrM1mBGSPQC1RmUtXmA9aZeFhMu49b"),
                false,
            ),
            AccountMeta::new(pubkey!("4zDe2QNMcGU8vyyRwnzkxLH4rrhe9VBNNGNz9yaYAL3"), true),
            AccountMeta::new(
                pubkey!("3xerTvpyKck7FB5drkikkn7cCdafuoe5qgMMtPVsGs5K"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8qEAygr3uFPh5Va4hAjCzu12X31ohtiWVNbGk2Vfr57M"),
                true,
            ),
            AccountMeta::new(
                pubkey!("4F8Aroc6aXhPauqpzYd36fgim99sgJYmr3SszgKMzC5R"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("zVmMsL5qGh7txhTHFgGZcFQpSsxSx6DBLJ3u113PBer"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
        ];
        let expected_arranged_accounts = InitializePoolV2InstructionAccounts {
            whirlpools_config: pubkey!("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ"),
            token_mint_a: pubkey!("3z2ruynV5UfjqHecqHcm81LseA4Bz5pVszSrYzeanqKf"),
            token_mint_b: pubkey!("Ey59PH7Z4BFU4HjyKnyMdWt5GGN76KazTAwQihoUXRnk"),
            token_badge_a: pubkey!("GATEWkJdo18j4mWYvzhidadAWSuA9RLkwARcnofH5W4B"),
            token_badge_b: pubkey!("GaDyFDz4rWd7fwbrM1mBGSPQC1RmUtXmA9aZeFhMu49b"),
            funder: pubkey!("4zDe2QNMcGU8vyyRwnzkxLH4rrhe9VBNNGNz9yaYAL3"),
            whirlpool: pubkey!("3xerTvpyKck7FB5drkikkn7cCdafuoe5qgMMtPVsGs5K"),
            token_vault_a: pubkey!("8qEAygr3uFPh5Va4hAjCzu12X31ohtiWVNbGk2Vfr57M"),
            token_vault_b: pubkey!("4F8Aroc6aXhPauqpzYd36fgim99sgJYmr3SszgKMzC5R"),
            fee_tier: pubkey!("zVmMsL5qGh7txhTHFgGZcFQpSsxSx6DBLJ3u113PBer"),
            token_program_a: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            token_program_b: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/initialize_pool_v2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializePoolV2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_position_bundle_ix() {
        // Arrange
        let expected_ix =
            OrcaWhirlpoolInstruction::InitializePositionBundle(InitializePositionBundle {});
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("2ZChq6FdKxY3Vgqg8PfqByWq4fVBvY9Rbnq5cJ8u2wPo"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EMY7ftt18nnjQSLd8v9QvJ91hZbeRFQRjXbkb5pEJWpF"),
                true,
            ),
            AccountMeta::new(
                pubkey!("F12n9B7iE2a5PKVXs4xs3bs3xtr2QVs1onYAWyxcYRuH"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FUzsmdqHtLhYD4Xsg6a8ekgtefLCKbaMvBRUQiWHuodt"),
                true,
            ),
            AccountMeta::new(
                pubkey!("FUzsmdqHtLhYD4Xsg6a8ekgtefLCKbaMvBRUQiWHuodt"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
        ];
        let expected_arranged_accounts = InitializePositionBundleInstructionAccounts {
            position_bundle: pubkey!("2ZChq6FdKxY3Vgqg8PfqByWq4fVBvY9Rbnq5cJ8u2wPo"),
            position_bundle_mint: pubkey!("EMY7ftt18nnjQSLd8v9QvJ91hZbeRFQRjXbkb5pEJWpF"),
            position_bundle_token_account: pubkey!("F12n9B7iE2a5PKVXs4xs3bs3xtr2QVs1onYAWyxcYRuH"),
            position_bundle_owner: pubkey!("FUzsmdqHtLhYD4Xsg6a8ekgtefLCKbaMvBRUQiWHuodt"),
            funder: pubkey!("FUzsmdqHtLhYD4Xsg6a8ekgtefLCKbaMvBRUQiWHuodt"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/initialize_position_bundle_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializePositionBundle::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_position_bundle_with_metadata_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::InitializePositionBundleWithMetadata(
            InitializePositionBundleWithMetadata {},
        );
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("Hi1C58UFPsDjFicyLqneMGhnjnrCv6mqPZhgZAsPqua9"),
                false,
            ),
            AccountMeta::new(pubkey!("M8LwWr6hsDD1xtkjkV5dF6nb3hco1TCMGAQNaHgVjSC"), true),
            AccountMeta::new(
                pubkey!("DRme9v6GokUZfwT73crz74WEAyQjp9ZueyoaD4u4NqaB"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BpGAcZ4i7myVqkduQUmCc3Waxy5tEFXcj76bstnQFNAY"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9wbfz1APnCTSDxxfWaGLUTwP3P7pLU1a3ASuo7UsBZsJ"),
                true,
            ),
            AccountMeta::new(
                pubkey!("9wbfz1APnCTSDxxfWaGLUTwP3P7pLU1a3ASuo7UsBZsJ"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("3axbTs2z5GBy6usVbNVoqEgZMng3vZvMnAoX29BFfwhr"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                false,
            ),
        ];
        let expected_arranged_accounts = InitializePositionBundleWithMetadataInstructionAccounts {
            position_bundle: pubkey!("Hi1C58UFPsDjFicyLqneMGhnjnrCv6mqPZhgZAsPqua9"),
            position_bundle_mint: pubkey!("M8LwWr6hsDD1xtkjkV5dF6nb3hco1TCMGAQNaHgVjSC"),
            position_bundle_metadata: pubkey!("DRme9v6GokUZfwT73crz74WEAyQjp9ZueyoaD4u4NqaB"),
            position_bundle_token_account: pubkey!("BpGAcZ4i7myVqkduQUmCc3Waxy5tEFXcj76bstnQFNAY"),
            position_bundle_owner: pubkey!("9wbfz1APnCTSDxxfWaGLUTwP3P7pLU1a3ASuo7UsBZsJ"),
            funder: pubkey!("9wbfz1APnCTSDxxfWaGLUTwP3P7pLU1a3ASuo7UsBZsJ"),
            metadata_update_auth: pubkey!("3axbTs2z5GBy6usVbNVoqEgZMng3vZvMnAoX29BFfwhr"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            metadata_program: pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/initialize_position_bundle_with_metadata_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializePositionBundleWithMetadata::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_reward_v2_ix() {
        // Arrange
        let expected_ix =
            OrcaWhirlpoolInstruction::InitializeRewardV2(InitializeRewardV2 { reward_index: 1 });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("DjDsi34mSB66p2nhBL6YvhbcLtZbkGfNybFeLDjJqxJW"),
                true,
            ),
            AccountMeta::new(
                pubkey!("DjDsi34mSB66p2nhBL6YvhbcLtZbkGfNybFeLDjJqxJW"),
                true,
            ),
            AccountMeta::new(
                pubkey!("FWDvLqKFt5H514ov22XXGLopGVsf4w34SD1CEnhfMyJW"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("2emiLJvgkofybvvADYrJJwAaTUpCTU9YJcdhpGGMbGtb"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3jxf1SxwM76sHp65NUMdwphYHoT1b1WKSofZiLou6SyK"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
        ];
        let expected_arranged_accounts = InitializeRewardV2InstructionAccounts {
            reward_authority: pubkey!("DjDsi34mSB66p2nhBL6YvhbcLtZbkGfNybFeLDjJqxJW"),
            funder: pubkey!("DjDsi34mSB66p2nhBL6YvhbcLtZbkGfNybFeLDjJqxJW"),
            whirlpool: pubkey!("FWDvLqKFt5H514ov22XXGLopGVsf4w34SD1CEnhfMyJW"),
            reward_mint: pubkey!("orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE"),
            reward_token_badge: pubkey!("2emiLJvgkofybvvADYrJJwAaTUpCTU9YJcdhpGGMbGtb"),
            reward_vault: pubkey!("3jxf1SxwM76sHp65NUMdwphYHoT1b1WKSofZiLou6SyK"),
            reward_token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/initialize_reward_v2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializeRewardV2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_tick_array_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::InitializeTickArray(InitializeTickArray {
            start_tick_index: 0,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("3xerTvpyKck7FB5drkikkn7cCdafuoe5qgMMtPVsGs5K"),
                false,
            ),
            AccountMeta::new(pubkey!("4zDe2QNMcGU8vyyRwnzkxLH4rrhe9VBNNGNz9yaYAL3"), true),
            AccountMeta::new(
                pubkey!("Fgmcorxso1tXtEGWYjwTsEttZBdNqcA9jx2HccqSwGGc"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
        ];
        let expected_arranged_accounts = InitializeTickArrayInstructionAccounts {
            whirlpool: pubkey!("3xerTvpyKck7FB5drkikkn7cCdafuoe5qgMMtPVsGs5K"),
            funder: pubkey!("4zDe2QNMcGU8vyyRwnzkxLH4rrhe9VBNNGNz9yaYAL3"),
            tick_array: pubkey!("Fgmcorxso1tXtEGWYjwTsEttZBdNqcA9jx2HccqSwGGc"),
            system_program: pubkey!("11111111111111111111111111111111"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/initialize_tick_array_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializeTickArray::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_initialize_token_badge_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::InitializeTokenBadge(InitializeTokenBadge {});
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("777H5H3Tp9U11uRVRzFwM8BinfiakbaLT8vQpeuhvEiH"),
                false,
            ),
            AccountMeta::new(pubkey!("r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6"), true),
            AccountMeta::new_readonly(
                pubkey!("mzeroXDoBpRVhnEXBra27qzAMdxgpWVY3DzQW7xMVJp"),
                false,
            ),
            AccountMeta::new(
                pubkey!("G4bXf8kkJ9Dq9cXNZV5jfUSe21nsu8712mUrDZSXysQM"),
                false,
            ),
            AccountMeta::new(pubkey!("r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6"), true),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
        ];
        let expected_arranged_accounts = InitializeTokenBadgeInstructionAccounts {
            whirlpools_config: pubkey!("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ"),
            whirlpools_config_extension: pubkey!("777H5H3Tp9U11uRVRzFwM8BinfiakbaLT8vQpeuhvEiH"),
            token_badge_authority: pubkey!("r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6"),
            token_mint: pubkey!("mzeroXDoBpRVhnEXBra27qzAMdxgpWVY3DzQW7xMVJp"),
            token_badge: pubkey!("G4bXf8kkJ9Dq9cXNZV5jfUSe21nsu8712mUrDZSXysQM"),
            funder: pubkey!("r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6"),
            system_program: pubkey!("11111111111111111111111111111111"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/initialize_token_badge_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            InitializeTokenBadge::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_open_bundled_position_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::OpenBundledPosition(OpenBundledPosition {
            bundle_index: 0,
            tick_lower_index: 78272,
            tick_upper_index: 79872,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("FWim6XUdmPkSdgRgNRKPqerTNzN378WJwQX5STaL1pem"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BgrZQ84QfmAZRRR11iMBQVFq1RcDUPZ5P74po8MugqfC"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("7brwbZhAm5gksZWjC54zMcxQRXpgqZZrH6ManPqXauiR"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8pFnziupLUjUtbHopmxcpGohhf6h6z9QNMHWRvXRfJsn"),
                true,
            ),
            AccountMeta::new(
                pubkey!("4L1v2B8wpag2JrM1DXJuxYPNDNGBaGbZRAygYJNxyFoz"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8pFnziupLUjUtbHopmxcpGohhf6h6z9QNMHWRvXRfJsn"),
                true,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
        ];
        let expected_arranged_accounts = OpenBundledPositionInstructionAccounts {
            bundled_position: pubkey!("FWim6XUdmPkSdgRgNRKPqerTNzN378WJwQX5STaL1pem"),
            position_bundle: pubkey!("BgrZQ84QfmAZRRR11iMBQVFq1RcDUPZ5P74po8MugqfC"),
            position_bundle_token_account: pubkey!("7brwbZhAm5gksZWjC54zMcxQRXpgqZZrH6ManPqXauiR"),
            position_bundle_authority: pubkey!("8pFnziupLUjUtbHopmxcpGohhf6h6z9QNMHWRvXRfJsn"),
            whirlpool: pubkey!("4L1v2B8wpag2JrM1DXJuxYPNDNGBaGbZRAygYJNxyFoz"),
            funder: pubkey!("8pFnziupLUjUtbHopmxcpGohhf6h6z9QNMHWRvXRfJsn"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/open_bundled_position_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            OpenBundledPosition::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_open_position_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::OpenPosition(OpenPosition {
            bumps: OpenPositionBumps { position_bump: 253 },
            tick_lower_index: -21264,
            tick_upper_index: -19424,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("DqK18n46EmUpetfGKcdLVx2uWZWUTCx6BvcwqTDqLuAy"),
                true,
            ),
            AccountMeta::new(
                pubkey!("EzNTbBxqXHxFp1S6UsoVYH9ynfrKke6Spq5dYV86LWKw"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8sGvh6GTNxToZNd9fttMvxRN1GEuXVMxwZbqKmhHPhUc"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4t8CquCQLazH6AZHtCdqfNowTvGztBRFdQ5ewimu8bpB"),
                true,
            ),
            AccountMeta::new(
                pubkey!("Fxnq2PT3z5zicLpiypV19NHrDTppmdXSfSdaVcrH7vwS"),
                false,
            ),
            AccountMeta::new(
                pubkey!("C9U2Ksk6KKWvLEeo5yUQ7Xu46X7NzeBJtd9PBfuXaUSM"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
        ];
        let expected_arranged_accounts = OpenPositionInstructionAccounts {
            funder: pubkey!("DqK18n46EmUpetfGKcdLVx2uWZWUTCx6BvcwqTDqLuAy"),
            owner: pubkey!("EzNTbBxqXHxFp1S6UsoVYH9ynfrKke6Spq5dYV86LWKw"),
            position: pubkey!("8sGvh6GTNxToZNd9fttMvxRN1GEuXVMxwZbqKmhHPhUc"),
            position_mint: pubkey!("4t8CquCQLazH6AZHtCdqfNowTvGztBRFdQ5ewimu8bpB"),
            position_token_account: pubkey!("Fxnq2PT3z5zicLpiypV19NHrDTppmdXSfSdaVcrH7vwS"),
            whirlpool: pubkey!("C9U2Ksk6KKWvLEeo5yUQ7Xu46X7NzeBJtd9PBfuXaUSM"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/open_position_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            OpenPosition::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_open_position_with_metadata_ix() {
        // Arrange
        let expected_ix =
            OrcaWhirlpoolInstruction::OpenPositionWithMetadata(OpenPositionWithMetadata {
                bumps: OpenPositionWithMetadataBumps {
                    metadata_bump: 255,
                    position_bump: 254,
                },
                tick_lower_index: -18776,
                tick_upper_index: -18576,
            });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("8VsaMPYx4DNPspdVtWtyzpRoCjCnWcX6U9rWjqRc4pF9"),
                true,
            ),
            AccountMeta::new(
                pubkey!("8VsaMPYx4DNPspdVtWtyzpRoCjCnWcX6U9rWjqRc4pF9"),
                true,
            ),
            AccountMeta::new(
                pubkey!("7DpkP4FYhrarfrKNzPcheRvcWxjK4XGC9uHuTikhNY3W"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8Y6STDa3Q1JdoC4eSUtpEpnV33bJgBwGxBdDHuoZyQ3c"),
                true,
            ),
            AccountMeta::new(
                pubkey!("D6CvMbP2ZGMqfj8mgBA3rgt6VbGtfPT3AqLfFYjKy6NK"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Bcc8P3bTCFHvnNezQ1mdYwVtPr8evceUi7LMEqUHcun6"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("3axbTs2z5GBy6usVbNVoqEgZMng3vZvMnAoX29BFfwhr"),
                false,
            ),
        ];
        let expected_arranged_accounts = OpenPositionWithMetadataInstructionAccounts {
            funder: pubkey!("8VsaMPYx4DNPspdVtWtyzpRoCjCnWcX6U9rWjqRc4pF9"),
            owner: pubkey!("8VsaMPYx4DNPspdVtWtyzpRoCjCnWcX6U9rWjqRc4pF9"),
            position: pubkey!("7DpkP4FYhrarfrKNzPcheRvcWxjK4XGC9uHuTikhNY3W"),
            position_mint: pubkey!("8Y6STDa3Q1JdoC4eSUtpEpnV33bJgBwGxBdDHuoZyQ3c"),
            position_metadata_account: pubkey!("D6CvMbP2ZGMqfj8mgBA3rgt6VbGtfPT3AqLfFYjKy6NK"),
            position_token_account: pubkey!("Bcc8P3bTCFHvnNezQ1mdYwVtPr8evceUi7LMEqUHcun6"),
            whirlpool: pubkey!("Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            system_program: pubkey!("11111111111111111111111111111111"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            metadata_program: pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
            metadata_update_auth: pubkey!("3axbTs2z5GBy6usVbNVoqEgZMng3vZvMnAoX29BFfwhr"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/open_position_with_metadata_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            OpenPositionWithMetadata::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_set_collect_protocol_fees_authority_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::SetCollectProtocolFeesAuthority(
            SetCollectProtocolFeesAuthority {},
        );
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AMxS2z98KhS3ZJAtMG7PkapJvP6sJTCMhmcFPZiUovMu"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("CTgympSD3hkBhH2XYWfGCigrFh3ZHAvnxwAfm7nCbExY"),
                false,
            ),
        ];
        let expected_arranged_accounts = SetCollectProtocolFeesAuthorityInstructionAccounts {
            whirlpools_config: pubkey!("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ"),
            collect_protocol_fees_authority: pubkey!(
                "AMxS2z98KhS3ZJAtMG7PkapJvP6sJTCMhmcFPZiUovMu"
            ),
            new_collect_protocol_fees_authority: pubkey!(
                "CTgympSD3hkBhH2XYWfGCigrFh3ZHAvnxwAfm7nCbExY"
            ),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction = carbon_test_utils::read_instruction(
            "tests/fixtures/set_collect_protocol_fees_authority_ix.json",
        )
        .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            SetCollectProtocolFeesAuthority::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_set_reward_emissions_v2_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::SetRewardEmissionsV2(SetRewardEmissionsV2 {
            emissions_per_second_x64: 610011378098860833862433862,
            reward_index: 0,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("FWDvLqKFt5H514ov22XXGLopGVsf4w34SD1CEnhfMyJW"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DjDsi34mSB66p2nhBL6YvhbcLtZbkGfNybFeLDjJqxJW"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("Dm5a5zgHGCbrjKufQJbJMoFQspQGV6kWk6PXD2iqgwx2"),
                false,
            ),
        ];
        let expected_arranged_accounts = SetRewardEmissionsV2InstructionAccounts {
            whirlpool: pubkey!("FWDvLqKFt5H514ov22XXGLopGVsf4w34SD1CEnhfMyJW"),
            reward_authority: pubkey!("DjDsi34mSB66p2nhBL6YvhbcLtZbkGfNybFeLDjJqxJW"),
            reward_vault: pubkey!("Dm5a5zgHGCbrjKufQJbJMoFQspQGV6kWk6PXD2iqgwx2"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/set_reward_emissions_v2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            SetRewardEmissionsV2::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_swap_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::Swap(Swap {
            a_to_b: false,
            amount: 85373619,
            amount_specified_is_input: true,
            other_amount_threshold: 0,
            sqrt_price_limit: 79226673515401279992447579055,
        });
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6sZGtMnbfZYrma9bKnTkLwjRYXBoTpzcP72cCAWPMhrk"),
                true,
            ),
            AccountMeta::new(
                pubkey!("2nJjE2ba3iGtefN4UNM1KN5FdYCwssU4Bzhc8URceqh6"),
                false,
            ),
            AccountMeta::new(
                pubkey!("51zQuBTBwidrpuYb55h3QNo1mgazmui6YQSg8V9APbHV"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5KzNJsS2CBTyEYBJXJFibWPike9mBW6ZBVcEsxASRowX"),
                false,
            ),
            AccountMeta::new(
                pubkey!("2YJnKoCuGn5HfSokHWd7ExMrjnQHmWB4j4ro5fo4gKaa"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6VsutsdwfRoJFkMwjxKNyAi9dsD5u37kUQefWRU9tuyM"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EL6zbUeYQVcrBerBsSwASH9LguiwhSJXv5H2hj4js4ED"),
                false,
            ),
            AccountMeta::new(
                pubkey!("29vLNbM9kA5AAKRKsphY6A5JDxAFYprkcJTVLLE7wZSJ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6crpbm8wwA9FTEheV1JmLLecCynzitrqGFxPondv7XNW"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("HQdJ9Fuk3RLBJ3EFBEKMC7hhJ3L9QKcWGhwSbQoJA3QT"),
                false,
            ),
        ];
        let expected_arranged_accounts = SwapInstructionAccounts {
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_authority: pubkey!("6sZGtMnbfZYrma9bKnTkLwjRYXBoTpzcP72cCAWPMhrk"),
            whirlpool: pubkey!("2nJjE2ba3iGtefN4UNM1KN5FdYCwssU4Bzhc8URceqh6"),
            token_owner_account_a: pubkey!("51zQuBTBwidrpuYb55h3QNo1mgazmui6YQSg8V9APbHV"),
            token_vault_a: pubkey!("5KzNJsS2CBTyEYBJXJFibWPike9mBW6ZBVcEsxASRowX"),
            token_owner_account_b: pubkey!("2YJnKoCuGn5HfSokHWd7ExMrjnQHmWB4j4ro5fo4gKaa"),
            token_vault_b: pubkey!("6VsutsdwfRoJFkMwjxKNyAi9dsD5u37kUQefWRU9tuyM"),
            tick_array0: pubkey!("EL6zbUeYQVcrBerBsSwASH9LguiwhSJXv5H2hj4js4ED"),
            tick_array1: pubkey!("29vLNbM9kA5AAKRKsphY6A5JDxAFYprkcJTVLLE7wZSJ"),
            tick_array2: pubkey!("6crpbm8wwA9FTEheV1JmLLecCynzitrqGFxPondv7XNW"),
            oracle: pubkey!("HQdJ9Fuk3RLBJ3EFBEKMC7hhJ3L9QKcWGhwSbQoJA3QT"),
            remaining_accounts: vec![],
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/swap_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            Swap::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_swap_v2_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::SwapV2(SwapV2 {
            a_to_b: true,
            amount: 367897999,
            amount_specified_is_input: true,
            other_amount_threshold: 1,
            remaining_accounts_info: None,
            sqrt_price_limit: 4295048016,
        });
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("6TYDxGmVxkBPBmEfnmLXx6jVff9LknsjRHqdTjVyZmG8"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AHTTzwf3GmVMJdxWM8v2MSxyjZj8rQR6hyAC3g9477Yj"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr"),
                false,
            ),
            AccountMeta::new(
                pubkey!("13Ma7KdfbBrF8NmDBnVTkwny7tjrZxYHCfn6a3FFbRUa"),
                false,
            ),
            AccountMeta::new(
                pubkey!("ATUMydDvNcELzNk9GP1Ky7i2Mgx2t2ej5aNPMhA6F2VH"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CX9yaoQw5YfpLH7Lamm5p2395ssKH2k3Gv8JDHL1vKM9"),
                false,
            ),
            AccountMeta::new(
                pubkey!("ChcWkmUbWDbBspDjPX6ZXi7Hb9kZ7VTbNUf6nMtWF1YH"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AiEsfEZ1mNTRuT4kaovStK8aVabrV8gJ5dJojkgFynaX"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9hcRuY6mg7EhCDzbgKoU8oz7xXTJNgY6dsgNF8fVxnqp"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CNRe6eKbdbSvmgY5BicihPXWQaUqF33rc352bB17crgS"),
                false,
            ),
            AccountMeta::new(
                pubkey!("7T9uvDmpdKVzxsWkEQvpnZKnEsaK6LQ7tnDpd6pNJfZH"),
                false,
            ),
        ];
        let expected_arranged_accounts = SwapV2InstructionAccounts {
            token_program_a: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program_b: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
            token_authority: pubkey!("6TYDxGmVxkBPBmEfnmLXx6jVff9LknsjRHqdTjVyZmG8"),
            whirlpool: pubkey!("AHTTzwf3GmVMJdxWM8v2MSxyjZj8rQR6hyAC3g9477Yj"),
            token_mint_a: pubkey!("So11111111111111111111111111111111111111112"),
            token_mint_b: pubkey!("7GCihgDB8fe6KNjn2MYtkzZcRjQy3t9GHdC8uHYmW2hr"),
            token_owner_account_a: pubkey!("13Ma7KdfbBrF8NmDBnVTkwny7tjrZxYHCfn6a3FFbRUa"),
            token_vault_a: pubkey!("ATUMydDvNcELzNk9GP1Ky7i2Mgx2t2ej5aNPMhA6F2VH"),
            token_owner_account_b: pubkey!("CX9yaoQw5YfpLH7Lamm5p2395ssKH2k3Gv8JDHL1vKM9"),
            token_vault_b: pubkey!("ChcWkmUbWDbBspDjPX6ZXi7Hb9kZ7VTbNUf6nMtWF1YH"),
            tick_array0: pubkey!("AiEsfEZ1mNTRuT4kaovStK8aVabrV8gJ5dJojkgFynaX"),
            tick_array1: pubkey!("9hcRuY6mg7EhCDzbgKoU8oz7xXTJNgY6dsgNF8fVxnqp"),
            tick_array2: pubkey!("CNRe6eKbdbSvmgY5BicihPXWQaUqF33rc352bB17crgS"),
            oracle: pubkey!("7T9uvDmpdKVzxsWkEQvpnZKnEsaK6LQ7tnDpd6pNJfZH"),
            remaining_accounts: vec![],
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/swap_v2_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            SwapV2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_two_hop_swap_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::TwoHopSwap(TwoHopSwap {
            a_to_b_one: true,
            a_to_b_two: false,
            amount: 110000,
            amount_specified_is_input: true,
            other_amount_threshold: 194634640,
            sqrt_price_limit_one: 4295048016,
            sqrt_price_limit_two: 79226673515401279992447579055,
        });
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GdTFjcR2iTnix7LQV417XkjmjnfGfBdxAWvjtujkoYfc"),
                true,
            ),
            AccountMeta::new(
                pubkey!("2ioUDDXppDSzAbvD1QRX6njJFRAj2jbTCmUz5KLn255e"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FRtf39f3n1ovCB61Us6upTNyEki4hGSvKKRZavoJ8Q5y"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3aQVx4SNRBV1SYbQmLbCD1CyL8zAEPBQCjxbLms11Jh1"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HB9B4oay2r22vcmP8uo9TaLAPv6DM8drPqZ1yS6KznN4"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4aEgQGaNkm5Q9hhdWqumLWw2r94PUS8seysT3SR8S7Lb"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AxKw6GkLHzUbx6gkZjFottX1Hk86XMxTccVtfsPtKxbA"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DTVGJCJJNCg7L29KNC8dK6r8H4VVGtLVgk5QdX1jQQe5"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CvzKsy2yNGuX7XwKby46LQsiJRdTXkZgkNJzwjS4LgN7"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4aEgQGaNkm5Q9hhdWqumLWw2r94PUS8seysT3SR8S7Lb"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9NGF4oRUXh2TE9vjajDZpz8id1T4EhYZVgNyiLpLyzaW"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HRHgmkgFWSpLnQ2cUATYkxD5zLHVNQXNqmtXcjaB4UNh"),
                false,
            ),
            AccountMeta::new(
                pubkey!("79SjfB67pJAzVqQ3pcQLZsb9FfZMrW5LSrQ3oSC2E1nh"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9LE77fqKDdX2auzRFTe6saJ7feZKt4TWoYstPh94KDbf"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HbMGtYegzq48UvKbirK995vqndCtaTzLiL4V2ZL6YvSN"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5ycPirRR9fTkjuyVqGNjHWSnJmPjhWUiztZ2jVbA1ng8"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EMjkXR59cxzFWHvgo9aQCo2eQ4fLgheCYzk3Vghc68r4"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5117rFaD8y1pdVG6bzAU2x2qJjfUTcRZVwNa8g2ogY6U"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BZVEkYH9xPFnnSdt6nNfcHn7wCV9h5X7nxXXTTnEt9u1"),
                false,
            ),
        ];
        let expected_arranged_accounts = TwoHopSwapInstructionAccounts {
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_authority: pubkey!("GdTFjcR2iTnix7LQV417XkjmjnfGfBdxAWvjtujkoYfc"),
            whirlpool_one: pubkey!("2ioUDDXppDSzAbvD1QRX6njJFRAj2jbTCmUz5KLn255e"),
            whirlpool_two: pubkey!("FRtf39f3n1ovCB61Us6upTNyEki4hGSvKKRZavoJ8Q5y"),
            token_owner_account_one_a: pubkey!("3aQVx4SNRBV1SYbQmLbCD1CyL8zAEPBQCjxbLms11Jh1"),
            token_vault_one_a: pubkey!("HB9B4oay2r22vcmP8uo9TaLAPv6DM8drPqZ1yS6KznN4"),
            token_owner_account_one_b: pubkey!("4aEgQGaNkm5Q9hhdWqumLWw2r94PUS8seysT3SR8S7Lb"),
            token_vault_one_b: pubkey!("AxKw6GkLHzUbx6gkZjFottX1Hk86XMxTccVtfsPtKxbA"),
            token_owner_account_two_a: pubkey!("DTVGJCJJNCg7L29KNC8dK6r8H4VVGtLVgk5QdX1jQQe5"),
            token_vault_two_a: pubkey!("CvzKsy2yNGuX7XwKby46LQsiJRdTXkZgkNJzwjS4LgN7"),
            token_owner_account_two_b: pubkey!("4aEgQGaNkm5Q9hhdWqumLWw2r94PUS8seysT3SR8S7Lb"),
            token_vault_two_b: pubkey!("9NGF4oRUXh2TE9vjajDZpz8id1T4EhYZVgNyiLpLyzaW"),
            tick_array_one0: pubkey!("HRHgmkgFWSpLnQ2cUATYkxD5zLHVNQXNqmtXcjaB4UNh"),
            tick_array_one1: pubkey!("79SjfB67pJAzVqQ3pcQLZsb9FfZMrW5LSrQ3oSC2E1nh"),
            tick_array_one2: pubkey!("9LE77fqKDdX2auzRFTe6saJ7feZKt4TWoYstPh94KDbf"),
            tick_array_two0: pubkey!("HbMGtYegzq48UvKbirK995vqndCtaTzLiL4V2ZL6YvSN"),
            tick_array_two1: pubkey!("5ycPirRR9fTkjuyVqGNjHWSnJmPjhWUiztZ2jVbA1ng8"),
            tick_array_two2: pubkey!("EMjkXR59cxzFWHvgo9aQCo2eQ4fLgheCYzk3Vghc68r4"),
            oracle_one: pubkey!("5117rFaD8y1pdVG6bzAU2x2qJjfUTcRZVwNa8g2ogY6U"),
            oracle_two: pubkey!("BZVEkYH9xPFnnSdt6nNfcHn7wCV9h5X7nxXXTTnEt9u1"),
            remaining_accounts: vec![],
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/two_hop_swap_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            TwoHopSwap::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_two_hop_swap_v2_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::TwoHopSwapV2(TwoHopSwapV2 {
            a_to_b_one: false,
            a_to_b_two: true,
            amount: 40139172,
            amount_specified_is_input: true,
            other_amount_threshold: 0,
            remaining_accounts_info: None,
            sqrt_price_limit_one: 79226673515401279992447579055,
            sqrt_price_limit_two: 4295048016,
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("EYMwW3Y7k37G9e3Hfks7KCFv8r5Sict7fBCYLz3vvusQ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("9tXiuRRw7kbejLhZXtxDxYs2REe43uH2e7k1kocgdM9B"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4Ht7H3BYgFtzsDBybgenETHuiLu7WQauBFuRACVa1Uzx"),
                false,
            ),
            AccountMeta::new(
                pubkey!("AruiMAnx9rWfcyKMKS1qZXt9KsJksxGFUaxL2xvgQPdb"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HupYrHwSbCU95VH2Q8SnpaWxwUhPPdqezAX4pvGssn3X"),
                false,
            ),
            AccountMeta::new(
                pubkey!("EeF6oBy6AQiBJoRx5xiRNxa6cmpQE3ayVagj28QFZuyg"),
                false,
            ),
            AccountMeta::new(
                pubkey!("MvB8poDgpDPbRgx8MXeb7EPEsawGuiBTqpkpM9exeLi"),
                false,
            ),
            AccountMeta::new(
                pubkey!("Aqb9GRGQbSC4jgKaBEoJmMzMtmomeH52FTTExjdZ6Taz"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CTgympSD3hkBhH2XYWfGCigrFh3ZHAvnxwAfm7nCbExY"),
                true,
            ),
            AccountMeta::new(
                pubkey!("EJVjpVJr8UdqxEA1TPcyE6njiQEvis7U2r66bwRnepoY"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8VRxdsWcdkwDpHFCVCRt8ivuXHXehyYA1psNwXJYWfwe"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5ijVVXeaBEdBfReVstNksvNQRGEK7dkeF2aZL5jv9N7y"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8hXTpuvJQRar4Pf6BZiEWquFgtAtSf2RFDM6EL2FCcf1"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FoA8jguautnY4AU9qYRCobyukqsRCn4irFVQ1mTUXmXL"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8dEgczJsVK2YNdqKCETR2G6ijo8uq8vKq7qDpST2YeDM"),
                false,
            ),
            AccountMeta::new(
                pubkey!("H2xbhvvRxPGQhcYqGgf41yAeHewZ8iGsQgzuFXuJ7LNC"),
                false,
            ),
            AccountMeta::new(
                pubkey!("BogFEjFPy2dHaJTUANvKnNjymswDVhDbyeCCcmScaHRG"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
                false,
            ),
        ];
        let expected_arranged_accounts = TwoHopSwapV2InstructionAccounts {
            whirlpool_one: pubkey!("EYMwW3Y7k37G9e3Hfks7KCFv8r5Sict7fBCYLz3vvusQ"),
            whirlpool_two: pubkey!("9tXiuRRw7kbejLhZXtxDxYs2REe43uH2e7k1kocgdM9B"),
            token_mint_input: pubkey!("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn"),
            token_mint_intermediate: pubkey!("2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo"),
            token_mint_output: pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
            token_program_input: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_program_intermediate: pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
            token_program_output: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            token_owner_account_input: pubkey!("4Ht7H3BYgFtzsDBybgenETHuiLu7WQauBFuRACVa1Uzx"),
            token_vault_one_input: pubkey!("AruiMAnx9rWfcyKMKS1qZXt9KsJksxGFUaxL2xvgQPdb"),
            token_vault_one_intermediate: pubkey!("HupYrHwSbCU95VH2Q8SnpaWxwUhPPdqezAX4pvGssn3X"),
            token_vault_two_intermediate: pubkey!("EeF6oBy6AQiBJoRx5xiRNxa6cmpQE3ayVagj28QFZuyg"),
            token_vault_two_output: pubkey!("MvB8poDgpDPbRgx8MXeb7EPEsawGuiBTqpkpM9exeLi"),
            token_owner_account_output: pubkey!("Aqb9GRGQbSC4jgKaBEoJmMzMtmomeH52FTTExjdZ6Taz"),
            token_authority: pubkey!("CTgympSD3hkBhH2XYWfGCigrFh3ZHAvnxwAfm7nCbExY"),
            tick_array_one0: pubkey!("EJVjpVJr8UdqxEA1TPcyE6njiQEvis7U2r66bwRnepoY"),
            tick_array_one1: pubkey!("8VRxdsWcdkwDpHFCVCRt8ivuXHXehyYA1psNwXJYWfwe"),
            tick_array_one2: pubkey!("5ijVVXeaBEdBfReVstNksvNQRGEK7dkeF2aZL5jv9N7y"),
            tick_array_two0: pubkey!("8hXTpuvJQRar4Pf6BZiEWquFgtAtSf2RFDM6EL2FCcf1"),
            tick_array_two1: pubkey!("FoA8jguautnY4AU9qYRCobyukqsRCn4irFVQ1mTUXmXL"),
            tick_array_two2: pubkey!("8dEgczJsVK2YNdqKCETR2G6ijo8uq8vKq7qDpST2YeDM"),
            oracle_one: pubkey!("H2xbhvvRxPGQhcYqGgf41yAeHewZ8iGsQgzuFXuJ7LNC"),
            oracle_two: pubkey!("BogFEjFPy2dHaJTUANvKnNjymswDVhDbyeCCcmScaHRG"),
            memo_program: pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"),
            remaining_accounts: vec![],
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/two_hop_swap_v2_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            TwoHopSwapV2::arrange_accounts(&instruction.accounts).expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_update_fees_and_rewards_ix() {
        // Arrange
        let expected_ix = OrcaWhirlpoolInstruction::UpdateFeesAndRewards(UpdateFeesAndRewards {});
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("C9U2Ksk6KKWvLEeo5yUQ7Xu46X7NzeBJtd9PBfuXaUSM"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3QzmtmQZWK4kbCCrvCPgMwx3jzpPHEpiP4bqFGGfzfYZ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("8BMs46yZnuqiWa2WVdwZreYYsemRNS5Smjnw5x3Kyi75"),
                false,
            ),
            AccountMeta::new(
                pubkey!("GPhfHEJjRVx5hnSkh3jcqWdJo3LthY31BmPZ6QRaRzDb"),
                false,
            ),
        ];
        let expected_arranged_accounts = UpdateFeesAndRewardsInstructionAccounts {
            whirlpool: pubkey!("C9U2Ksk6KKWvLEeo5yUQ7Xu46X7NzeBJtd9PBfuXaUSM"),
            position: pubkey!("3QzmtmQZWK4kbCCrvCPgMwx3jzpPHEpiP4bqFGGfzfYZ"),
            tick_array_lower: pubkey!("8BMs46yZnuqiWa2WVdwZreYYsemRNS5Smjnw5x3Kyi75"),
            tick_array_upper: pubkey!("GPhfHEJjRVx5hnSkh3jcqWdJo3LthY31BmPZ6QRaRzDb"),
        };

        // Act
        let decoder = OrcaWhirlpoolDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/update_fees_and_rewards_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            UpdateFeesAndRewards::arrange_accounts(&instruction.accounts)
                .expect("arrange accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }
}
