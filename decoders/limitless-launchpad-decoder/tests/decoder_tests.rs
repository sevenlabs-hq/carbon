use base64::Engine;
use carbon_core::{account::AccountDecoder, instruction::InstructionDecoder};
use carbon_limitless_launchpad_decoder::{
    accounts::LimitlessLaunchpadAccount,
    events::LimitlessLaunchpadEvent,
    instructions::LimitlessLaunchpadInstruction,
    types::{GraduationVenue, LaunchType},
    LimitlessLaunchpadDecoder, PROGRAM_ID,
};
use serde_json::Value;
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use std::str::FromStr;

fn fixture_json(name: &str) -> Value {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    let raw = std::fs::read_to_string(&path).expect("fixture readable");
    serde_json::from_str(&raw).expect("valid fixture json")
}

fn load_account(name: &str) -> Account {
    let v = fixture_json(name);
    let data_b64 = v["account"]["data"][0].as_str().expect("base64 data");
    let data = base64::engine::general_purpose::STANDARD
        .decode(data_b64)
        .expect("decode base64");
    let owner = Pubkey::from_str(v["account"]["owner"].as_str().expect("owner")).expect("pubkey");
    let lamports = v["account"]["lamports"].as_u64().expect("lamports");
    let executable = v["account"]["executable"].as_bool().unwrap_or(false);
    Account {
        lamports,
        data,
        owner,
        executable,
        rent_epoch: 0,
    }
}

fn extract_launchpad_instructions(name: &str) -> (Vec<Instruction>, Vec<LimitlessLaunchpadEvent>) {
    let v = fixture_json(name);
    let tx = &v["transaction"];
    let msg = &tx["transaction"]["message"];
    let mut keys: Vec<Pubkey> = msg["accountKeys"]
        .as_array()
        .expect("accountKeys")
        .iter()
        .map(|k| Pubkey::from_str(k.as_str().expect("key str")).expect("pubkey"))
        .collect();
    if let Some(loaded) = tx["meta"]["loadedAddresses"].as_object() {
        for field in ["writable", "readonly"] {
            if let Some(arr) = loaded.get(field).and_then(|x| x.as_array()) {
                for k in arr {
                    keys.push(Pubkey::from_str(k.as_str().expect("key")).expect("pubkey"));
                }
            }
        }
    }

    let mut ixs = Vec::new();
    let parse_ix = |ix: &Value| -> Option<Instruction> {
        let pid_idx = ix["programIdIndex"].as_u64()? as usize;
        let program_id = *keys.get(pid_idx)?;
        if program_id != PROGRAM_ID {
            return None;
        }
        let data = bs58::decode(ix["data"].as_str()?).into_vec().ok()?;
        let accounts: Vec<AccountMeta> = ix["accounts"]
            .as_array()?
            .iter()
            .map(|idx| {
                let pubkey = keys[idx.as_u64().expect("acc idx") as usize];
                AccountMeta::new_readonly(pubkey, false)
            })
            .collect();
        Some(Instruction {
            program_id,
            accounts,
            data,
        })
    };

    for ix in msg["instructions"].as_array().expect("instructions") {
        if let Some(parsed) = parse_ix(ix) {
            ixs.push(parsed);
        }
    }
    if let Some(groups) = tx["meta"]["innerInstructions"].as_array() {
        for group in groups {
            if let Some(inner) = group["instructions"].as_array() {
                for ix in inner {
                    if let Some(parsed) = parse_ix(ix) {
                        ixs.push(parsed);
                    }
                }
            }
        }
    }

    let mut events = Vec::new();
    if let Some(logs) = tx["meta"]["logMessages"].as_array() {
        for line in logs.iter().filter_map(|l| l.as_str()) {
            if let Some(b64) = line.strip_prefix("Program data: ") {
                if let Ok(raw) = base64::engine::general_purpose::STANDARD.decode(b64.trim()) {
                    if let Some(ev) = LimitlessLaunchpadEvent::decode(&raw) {
                        events.push(ev);
                    }
                }
            }
        }
    }

    (ixs, events)
}

#[test]
fn decodes_mainnet_accounts() {
    let decoder = LimitlessLaunchpadDecoder;

    let cfg_acc = load_account("acc_config.json");
    let decoded_cfg = decoder.decode_account(&cfg_acc).expect("decode Config");
    match decoded_cfg.data {
        LimitlessLaunchpadAccount::Config(cfg) => {
            assert_eq!(cfg.bonding_curve_fee_bps, 125);
            assert_eq!(cfg.graduation_venue_default, GraduationVenue::MeteoraDammV2);
            assert!(cfg.meteora_graduation_enabled);
            assert_eq!(cfg.effective_meteora_pool_fee_tenths_of_bps(), 1_250);
        }
        other => panic!("unexpected account variant: {other:?}"),
    }

    let active_acc = load_account("acc_launched_coin_active.json");
    let decoded_active = decoder
        .decode_account(&active_acc)
        .expect("decode active LaunchedCoin");
    match decoded_active.data {
        LimitlessLaunchpadAccount::LaunchedCoin(coin) => {
            assert_eq!(
                coin.mint.to_string(),
                "ChqhroEewbABabezzCNSqA6pmqGng7D7n1QzaPuUbuzz"
            );
            assert_eq!(coin.launch_type, LaunchType::Standard);
            assert!(!coin.graduated);
            assert_eq!(coin.virtual_sol_lamports(), Some(30_001_975_000));
            assert_eq!(coin.virtual_token_units(), Some(1_072_929_365_483_440));
        }
        other => panic!("unexpected account variant: {other:?}"),
    }

    let grad_acc = load_account("acc_launched_coin_graduated.json");
    let decoded_grad = decoder
        .decode_account(&grad_acc)
        .expect("decode graduated LaunchedCoin");
    let expected_pool = "9FsfMfizwNaqbCGFnFAdZzkAzTfc7PWwM9foUBrCF5Af";
    match decoded_grad.data {
        LimitlessLaunchpadAccount::LaunchedCoin(coin) => {
            assert!(coin.graduated);
            assert!(coin.curve_complete());
            assert_eq!(coin.preferred_swap_venue.to_string(), expected_pool);
        }
        other => panic!("unexpected account variant: {other:?}"),
    }

    let receipt_acc = load_account("acc_meteora_graduation.json");
    let decoded_receipt = decoder
        .decode_account(&receipt_acc)
        .expect("decode MeteoraGraduation");
    match decoded_receipt.data {
        LimitlessLaunchpadAccount::MeteoraGraduation(r) => {
            assert_eq!(r.pool.to_string(), expected_pool);
            assert_eq!(r.pool_fee_bps, 1_250);
            assert_eq!(r.sol_deposited, 971_150_680);
        }
        other => panic!("unexpected account variant: {other:?}"),
    }
}

#[test]
fn decodes_mainnet_instructions_and_events() {
    let decoder = LimitlessLaunchpadDecoder;

    // 1. create_or_buy_pda + inner create_and_first_buy
    let (ixs, evs) = extract_launchpad_instructions("tx_create_or_buy_pda.json");
    assert_eq!(ixs.len(), 2);
    assert!(matches!(
        decoder.decode_instruction(&ixs[0]),
        Some(LimitlessLaunchpadInstruction::CreateOrBuyPda { .. })
    ));
    assert!(matches!(
        decoder.decode_instruction(&ixs[1]),
        Some(LimitlessLaunchpadInstruction::CreateAndFirstBuy { .. })
    ));
    assert!(evs
        .iter()
        .any(|e| matches!(e, LimitlessLaunchpadEvent::CoinLaunched(_))));
    assert!(evs
        .iter()
        .any(|e| matches!(e, LimitlessLaunchpadEvent::Bought(_))));

    // 2. launch_coin
    let (ixs, evs) = extract_launchpad_instructions("tx_launch_coin_and_splitter.json");
    assert_eq!(ixs.len(), 1);
    assert!(matches!(
        decoder.decode_instruction(&ixs[0]),
        Some(LimitlessLaunchpadInstruction::LaunchCoin { .. })
    ));
    assert_eq!(evs.len(), 2);

    // 3. buy
    let (ixs, evs) = extract_launchpad_instructions("tx_buy.json");
    assert_eq!(ixs.len(), 1);
    match decoder.decode_instruction(&ixs[0]) {
        Some(LimitlessLaunchpadInstruction::Buy { data, .. }) => {
            assert_eq!(data.params.sol_in, 4_812_227);
        }
        other => panic!("expected Buy, got {other:?}"),
    }
    assert!(matches!(
        evs.as_slice(),
        [LimitlessLaunchpadEvent::Bought(_)]
    ));

    // 4. sell
    let (ixs, evs) = extract_launchpad_instructions("tx_sell.json");
    assert_eq!(ixs.len(), 1);
    match decoder.decode_instruction(&ixs[0]) {
        Some(LimitlessLaunchpadInstruction::Sell { data, .. }) => {
            assert_eq!(data.params.tokens_in, 1_010_450_202_972);
        }
        other => panic!("expected Sell, got {other:?}"),
    }
    assert!(matches!(evs.as_slice(), [LimitlessLaunchpadEvent::Sold(_)]));

    // 5. prepare_meteora_graduation
    let (ixs, _) = extract_launchpad_instructions("tx_prepare_meteora_graduation.json");
    assert_eq!(ixs.len(), 1);
    assert!(matches!(
        decoder.decode_instruction(&ixs[0]),
        Some(LimitlessLaunchpadInstruction::PrepareMeteoraGraduation { .. })
    ));

    // 6. graduate_meteora + MeteoraGraduated event
    let (ixs, evs) = extract_launchpad_instructions("tx_graduate_meteora.json");
    assert_eq!(ixs.len(), 1);
    assert!(matches!(
        decoder.decode_instruction(&ixs[0]),
        Some(LimitlessLaunchpadInstruction::GraduateMeteora { .. })
    ));
    match evs.as_slice() {
        [LimitlessLaunchpadEvent::MeteoraGraduated(ev)] => {
            assert_eq!(
                ev.mint.to_string(),
                "BYhPKXoMaQXnGhhMaatDRwWCLdKnEG5LfAtYJ8knEgFn"
            );
            assert_eq!(
                ev.pool.to_string(),
                "9FsfMfizwNaqbCGFnFAdZzkAzTfc7PWwM9foUBrCF5Af"
            );
        }
        other => panic!("expected MeteoraGraduated, got {other:?}"),
    }

    // 7. claim_meteora_fees
    let (ixs, _) = extract_launchpad_instructions("tx_claim_meteora_fees.json");
    assert_eq!(ixs.len(), 1);
    assert!(matches!(
        decoder.decode_instruction(&ixs[0]),
        Some(LimitlessLaunchpadInstruction::ClaimMeteoraFees { .. })
    ));
}
