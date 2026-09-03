use {
    carbon_token_2022_decoder::types::Extension,
    solana_pubkey::Pubkey,
    spl_token_2022::{
        extension::{
            interest_bearing_mint::InterestBearingConfig, mint_close_authority::MintCloseAuthority,
            permanent_delegate::PermanentDelegate, permissioned_burn::PermissionedBurnConfig,
            scaled_ui_amount::ScaledUiAmountConfig, transfer_fee::TransferFeeConfig,
            transfer_hook::TransferHook, BaseStateWithExtensionsMut, ExtensionType,
            StateWithExtensions, StateWithExtensionsMut,
        },
        state::Mint,
    },
};

const NULLABLE_EXTENSION_TYPES: [ExtensionType; 7] = [
    ExtensionType::TransferFeeConfig,
    ExtensionType::MintCloseAuthority,
    ExtensionType::InterestBearingConfig,
    ExtensionType::PermanentDelegate,
    ExtensionType::TransferHook,
    ExtensionType::ScaledUiAmount,
    ExtensionType::PermissionedBurn,
];

fn nullable_extension_mint(
    configure: impl FnOnce(&mut StateWithExtensionsMut<'_, Mint>),
) -> Vec<u8> {
    let mint_len =
        ExtensionType::try_calculate_account_len::<Mint>(&NULLABLE_EXTENSION_TYPES).unwrap();
    let mut data = vec![0; mint_len];

    {
        let mut mint = StateWithExtensionsMut::<Mint>::unpack_uninitialized(&mut data).unwrap();
        mint.init_extension::<TransferFeeConfig>(true).unwrap();
        mint.init_extension::<MintCloseAuthority>(true).unwrap();
        mint.init_extension::<InterestBearingConfig>(true).unwrap();
        mint.init_extension::<PermanentDelegate>(true).unwrap();
        mint.init_extension::<TransferHook>(true).unwrap();
        mint.init_extension::<ScaledUiAmountConfig>(true).unwrap();
        mint.init_extension::<PermissionedBurnConfig>(true).unwrap();

        configure(&mut mint);
        mint.base.is_initialized = true;
        mint.pack_base();
        mint.init_account_type().unwrap();
    }

    data
}

fn decode_nullable_extensions(data: &[u8]) -> Vec<Extension> {
    let mint = StateWithExtensions::<Mint>::unpack(data).unwrap();
    NULLABLE_EXTENSION_TYPES
        .iter()
        .filter_map(|extension_type| Extension::from_mint_and_type(&mint, extension_type))
        .collect()
}

#[test]
fn nullable_extension_pubkeys_preserve_none() {
    let extensions = decode_nullable_extensions(&nullable_extension_mint(|_| {}));

    assert_eq!(extensions.len(), NULLABLE_EXTENSION_TYPES.len());
    assert!(extensions.iter().any(|extension| matches!(
        extension,
        Extension::TransferFeeConfig {
            transfer_fee_config_authority: None,
            withdraw_withheld_authority: None,
            ..
        }
    )));
    assert!(extensions.iter().any(|extension| matches!(
        extension,
        Extension::MintCloseAuthority {
            close_authority: None
        }
    )));
    assert!(extensions.iter().any(|extension| matches!(
        extension,
        Extension::InterestBearingConfig {
            rate_authority: None,
            ..
        }
    )));
    assert!(extensions
        .iter()
        .any(|extension| matches!(extension, Extension::PermanentDelegate { delegate: None })));
    assert!(extensions.iter().any(|extension| matches!(
        extension,
        Extension::TransferHook {
            authority: None,
            program_id: None
        }
    )));
    assert!(extensions.iter().any(|extension| matches!(
        extension,
        Extension::ScaledUiAmountConfig {
            authority: None,
            ..
        }
    )));
    assert!(extensions
        .iter()
        .any(|extension| matches!(extension, Extension::PermissionedBurn { authority: None })));
}

#[test]
fn nullable_extension_pubkeys_preserve_some() {
    let program_id = Pubkey::new_from_array([7; 32]);
    let data = nullable_extension_mint(|mint| {
        mint.get_extension_mut::<TransferHook>().unwrap().program_id =
            Some(program_id).try_into().unwrap();
    });
    let extensions = decode_nullable_extensions(&data);

    assert!(extensions.iter().any(|extension| matches!(
        extension,
        Extension::TransferHook {
            program_id: Some(value),
            ..
        } if *value == program_id
    )));
}
