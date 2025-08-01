use crate::PROGRAM_ID;

use super::PumpfunDecoder;

pub mod admin_set_creator;
pub mod admin_set_creator_event;
pub mod admin_update_token_incentives;
pub mod admin_update_token_incentives_event;
pub mod buy;
pub mod claim_token_incentives;
pub mod claim_token_incentives_event;
pub mod collect_creator_fee;
pub mod collect_creator_fee_event;
pub mod complete_event;
pub mod complete_pump_amm_migration_event;
pub mod create;
pub mod create_event;
pub mod extend_account;
pub mod extend_account_event;
pub mod initialize;
pub mod migrate;
pub mod sell;
pub mod set_creator;
pub mod set_creator_event;
pub mod set_metaplex_creator;
pub mod set_metaplex_creator_event;
pub mod set_params;
pub mod set_params_event;
pub mod sync_user_volume_accumulator;
pub mod sync_user_volume_accumulator_event;
pub mod trade_event;
pub mod update_global_authority;
pub mod update_global_authority_event;

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
pub enum PumpfunInstruction {
    AdminSetCreator(admin_set_creator::AdminSetCreator),
    AdminUpdateTokenIncentives(admin_update_token_incentives::AdminUpdateTokenIncentives),
    Buy(buy::Buy),
    ClaimTokenIncentives(claim_token_incentives::ClaimTokenIncentives),
    CollectCreatorFee(collect_creator_fee::CollectCreatorFee),
    Create(create::Create),
    ExtendAccount(extend_account::ExtendAccount),
    Initialize(initialize::Initialize),
    Migrate(migrate::Migrate),
    Sell(sell::Sell),
    SetCreator(set_creator::SetCreator),
    SetMetaplexCreator(set_metaplex_creator::SetMetaplexCreator),
    SetParams(set_params::SetParams),
    SyncUserVolumeAccumulator(sync_user_volume_accumulator::SyncUserVolumeAccumulator),
    UpdateGlobalAuthority(update_global_authority::UpdateGlobalAuthority),
    AdminSetCreatorEvent(admin_set_creator_event::AdminSetCreatorEvent),
    AdminUpdateTokenIncentivesEvent(
        admin_update_token_incentives_event::AdminUpdateTokenIncentivesEvent,
    ),
    ClaimTokenIncentivesEvent(claim_token_incentives_event::ClaimTokenIncentivesEvent),
    CollectCreatorFeeEvent(collect_creator_fee_event::CollectCreatorFeeEvent),
    CompleteEvent(complete_event::CompleteEvent),
    CompletePumpAmmMigrationEvent(complete_pump_amm_migration_event::CompletePumpAmmMigrationEvent),
    CreateEvent(create_event::CreateEvent),
    ExtendAccountEvent(extend_account_event::ExtendAccountEvent),
    SetCreatorEvent(set_creator_event::SetCreatorEvent),
    SetMetaplexCreatorEvent(set_metaplex_creator_event::SetMetaplexCreatorEvent),
    SetParamsEvent(set_params_event::SetParamsEvent),
    SyncUserVolumeAccumulatorEvent(
        sync_user_volume_accumulator_event::SyncUserVolumeAccumulatorEvent,
    ),
    TradeEvent(trade_event::TradeEvent),
    UpdateGlobalAuthorityEvent(update_global_authority_event::UpdateGlobalAuthorityEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for PumpfunDecoder {
    type InstructionType = PumpfunInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }
        carbon_core::try_decode_instructions!(instruction,
            PumpfunInstruction::AdminSetCreator => admin_set_creator::AdminSetCreator,
            PumpfunInstruction::AdminUpdateTokenIncentives => admin_update_token_incentives::AdminUpdateTokenIncentives,
            PumpfunInstruction::Buy => buy::Buy,
            PumpfunInstruction::ClaimTokenIncentives => claim_token_incentives::ClaimTokenIncentives,
            PumpfunInstruction::CollectCreatorFee => collect_creator_fee::CollectCreatorFee,
            PumpfunInstruction::Create => create::Create,
            PumpfunInstruction::ExtendAccount => extend_account::ExtendAccount,
            PumpfunInstruction::Initialize => initialize::Initialize,
            PumpfunInstruction::Migrate => migrate::Migrate,
            PumpfunInstruction::Sell => sell::Sell,
            PumpfunInstruction::SetCreator => set_creator::SetCreator,
            PumpfunInstruction::SetMetaplexCreator => set_metaplex_creator::SetMetaplexCreator,
            PumpfunInstruction::SetParams => set_params::SetParams,
            PumpfunInstruction::SyncUserVolumeAccumulator => sync_user_volume_accumulator::SyncUserVolumeAccumulator,
            PumpfunInstruction::UpdateGlobalAuthority => update_global_authority::UpdateGlobalAuthority,
            PumpfunInstruction::AdminSetCreatorEvent => admin_set_creator_event::AdminSetCreatorEvent,
            PumpfunInstruction::AdminUpdateTokenIncentivesEvent => admin_update_token_incentives_event::AdminUpdateTokenIncentivesEvent,
            PumpfunInstruction::ClaimTokenIncentivesEvent => claim_token_incentives_event::ClaimTokenIncentivesEvent,
            PumpfunInstruction::CollectCreatorFeeEvent => collect_creator_fee_event::CollectCreatorFeeEvent,
            PumpfunInstruction::CompleteEvent => complete_event::CompleteEvent,
            PumpfunInstruction::CompletePumpAmmMigrationEvent => complete_pump_amm_migration_event::CompletePumpAmmMigrationEvent,
            PumpfunInstruction::CreateEvent => create_event::CreateEvent,
            PumpfunInstruction::ExtendAccountEvent => extend_account_event::ExtendAccountEvent,
            PumpfunInstruction::SetCreatorEvent => set_creator_event::SetCreatorEvent,
            PumpfunInstruction::SetMetaplexCreatorEvent => set_metaplex_creator_event::SetMetaplexCreatorEvent,
            PumpfunInstruction::SetParamsEvent => set_params_event::SetParamsEvent,
            PumpfunInstruction::SyncUserVolumeAccumulatorEvent => sync_user_volume_accumulator_event::SyncUserVolumeAccumulatorEvent,
            PumpfunInstruction::TradeEvent => trade_event::TradeEvent,
            PumpfunInstruction::UpdateGlobalAuthorityEvent => update_global_authority_event::UpdateGlobalAuthorityEvent,
        )
    }
}

#[cfg(test)]
mod tests {
    use alloc::{borrow::ToOwned, vec};
    use carbon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
    use solana_instruction::AccountMeta;
    use solana_pubkey::pubkey;

    use super::*;

    #[test]
    fn test_decode_buy() {
        // Arrange
        let expected_ix = PumpfunInstruction::Buy(buy::Buy {
            amount: 1761808330141,
            max_sol_cost: 55000000,
        });
        let expected_accounts = vec![
            AccountMeta {
                pubkey: pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("68yFSZxzLWJXkxxRGydZ63C6mHx1NLEDWmwN9Lb5yySg"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("ANY6GmWTnMdcve9ouPYRfTUaJEzi5pFHxaXDRReMveMo"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("7BXEBYDN1RDb9gyp3ZXuYfk63rRrYsbwUjbVCTrM7kMq"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("6s8yqaYzxYgkaguisExzrTsZVUe5zGqk5yVeFh2XZvpo"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("8EDQQdDDJ7RuKgAAfubcWiUUakpSzRE9TBorhtqxM1Y2"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("B52TW9jWxJfe84ovSe8eHtw7s8zu4ZAJ2LSkWE3cP2pd"),
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("11111111111111111111111111111111"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("Av288QVHYUTtKQCyxFHMWPoF4gTLCowJpw6mQm2jjjPZ"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("Hq2wp8uJ9jCPsYgNHex8RtqdvMPfVGoYwjvF1ATiwn2Y"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("A5YYvy72JYi6PhwHRNm6VPH1mt17a7MjUp2CdZf6xsKQ"),
                is_signer: false,
                is_writable: false,
            },
        ];
        let expected_arranged_accounts = buy::BuyInstructionAccounts {
            global: pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            fee_recipient: pubkey!("68yFSZxzLWJXkxxRGydZ63C6mHx1NLEDWmwN9Lb5yySg"),
            mint: pubkey!("ANY6GmWTnMdcve9ouPYRfTUaJEzi5pFHxaXDRReMveMo"),
            bonding_curve: pubkey!("7BXEBYDN1RDb9gyp3ZXuYfk63rRrYsbwUjbVCTrM7kMq"),
            associated_bonding_curve: pubkey!("6s8yqaYzxYgkaguisExzrTsZVUe5zGqk5yVeFh2XZvpo"),
            associated_user: pubkey!("8EDQQdDDJ7RuKgAAfubcWiUUakpSzRE9TBorhtqxM1Y2"),
            user: pubkey!("B52TW9jWxJfe84ovSe8eHtw7s8zu4ZAJ2LSkWE3cP2pd"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            creator_vault: pubkey!("Av288QVHYUTtKQCyxFHMWPoF4gTLCowJpw6mQm2jjjPZ"),
            event_authority: pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
            global_volume_accumulator: pubkey!("Hq2wp8uJ9jCPsYgNHex8RtqdvMPfVGoYwjvF1ATiwn2Y"),
            user_volume_accumulator: pubkey!("A5YYvy72JYi6PhwHRNm6VPH1mt17a7MjUp2CdZf6xsKQ"),
        };

        // Act
        let decoder = PumpfunDecoder;
        // The buy fixture is from devnet
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/buy_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            buy::Buy::arrange_accounts(&instruction.accounts).expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_sell() {
        // Arrange
        let expected_ix = PumpfunInstruction::Sell(sell::Sell {
            amount: 88888000000,
            min_sol_output: 2361153,
        });
        let expected_accounts = vec![
            AccountMeta {
                pubkey: pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("AC69oJv1m7843mdRfoQDneZuyRxYrMq86i2mARMtpump"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("623TpUDcZjKdmd9wybMveLKSSbgRs2hvwFjygzi4g15B"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("5rQKu3z4SXShvQkNKSJu9mtsVmgM8AvLoeNbJGvTyQv6"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("BFgJqMUhraJvzERrt2BbPqbqDcLgbiHMdfdPQAqLtPcR"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("D8h8aUEaQnBRALrcTxLkaLLCQVCASnLVx17E3m6qfuPF"),
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("11111111111111111111111111111111"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("8rsczKQ9bVT6AcGoD4CqoKySbVErCoXwQH8h7ZjAsUqE"),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
                is_signer: false,
                is_writable: false,
            },
        ];

        let expected_arranged_accounts = sell::SellInstructionAccounts {
            global: pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            fee_recipient: pubkey!("62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV"),
            mint: pubkey!("AC69oJv1m7843mdRfoQDneZuyRxYrMq86i2mARMtpump"),
            bonding_curve: pubkey!("623TpUDcZjKdmd9wybMveLKSSbgRs2hvwFjygzi4g15B"),
            associated_bonding_curve: pubkey!("5rQKu3z4SXShvQkNKSJu9mtsVmgM8AvLoeNbJGvTyQv6"),
            associated_user: pubkey!("BFgJqMUhraJvzERrt2BbPqbqDcLgbiHMdfdPQAqLtPcR"),
            user: pubkey!("D8h8aUEaQnBRALrcTxLkaLLCQVCASnLVx17E3m6qfuPF"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            creator_vault: pubkey!("8rsczKQ9bVT6AcGoD4CqoKySbVErCoXwQH8h7ZjAsUqE"),
            event_authority: pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
        };

        // Act
        let decoder = PumpfunDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/sell_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            sell::Sell::arrange_accounts(&instruction.accounts).expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_create() {
        // Arrange
        let expected_ix = PumpfunInstruction::Create(create::Create {
            name: "Mystical Cat".to_owned(),
            symbol: "MC".to_owned(),
            uri: "https://ipfs.io/ipfs/QmeXR4TB9NZVK279DAVDKevSLzrWSi9papnaknVPJ8AvHe".to_owned(),
            creator: pubkey!("CkdtUhQdH2sHXJYTJTNFbF1K5W33WVgVHG7zffaMkEmv"),
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("AC69oJv1m7843mdRfoQDneZuyRxYrMq86i2mARMtpump"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM"),
                false,
            ),
            AccountMeta::new(
                pubkey!("623TpUDcZjKdmd9wybMveLKSSbgRs2hvwFjygzi4g15B"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5rQKu3z4SXShvQkNKSJu9mtsVmgM8AvLoeNbJGvTyQv6"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                false,
            ),
            AccountMeta::new(
                pubkey!("311RNHU1xDgJfYvQLszzgnBjQ1fDg5Ddr2yR3ymmUCed"),
                false,
            ),
            AccountMeta::new(
                pubkey!("CkdtUhQdH2sHXJYTJTNFbF1K5W33WVgVHG7zffaMkEmv"),
                true,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("SysvarRent111111111111111111111111111111111"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
                false,
            ),
        ];
        let expected_arranged_accounts = create::CreateInstructionAccounts {
            mint: pubkey!("AC69oJv1m7843mdRfoQDneZuyRxYrMq86i2mARMtpump"),
            mint_authority: pubkey!("TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM"),
            bonding_curve: pubkey!("623TpUDcZjKdmd9wybMveLKSSbgRs2hvwFjygzi4g15B"),
            associated_bonding_curve: pubkey!("5rQKu3z4SXShvQkNKSJu9mtsVmgM8AvLoeNbJGvTyQv6"),
            global: pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf"),
            mpl_token_metadata: pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
            metadata: pubkey!("311RNHU1xDgJfYvQLszzgnBjQ1fDg5Ddr2yR3ymmUCed"),
            user: pubkey!("CkdtUhQdH2sHXJYTJTNFbF1K5W33WVgVHG7zffaMkEmv"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            rent: pubkey!("SysvarRent111111111111111111111111111111111"),
            event_authority: pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1"),
            program: pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"),
        };

        // Act
        let decoder = PumpfunDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/create_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            create::Create::arrange_accounts(&instruction.accounts).expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }
}
