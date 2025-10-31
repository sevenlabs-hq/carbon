import type { DatasourceArtifact, DecoderMeta } from './index';

function rustJoin(items: string[]): string {
    return items.map(s => `${s},`).join('\n                            ');
}

export function buildHeliusAtlasWs(decoders: DecoderMeta[]): DatasourceArtifact {
    const programIds = decoders.map(d => `${d.name.toUpperCase()}_PROGRAM_ID.to_string().clone()`);

    const imports = [
        'std::collections::HashSet',
        'tokio::sync::RwLock',
        'helius::types::{Cluster, RpcTransactionsConfig, TransactionSubscribeFilter, TransactionSubscribeOptions, UiEnhancedTransactionEncoding, TransactionDetails, TransactionCommitment}',
        'carbon_helius_atlas_ws_datasource::{HeliusWebsocket, Filters}',
    ];

    const init = `
HeliusWebsocket::new(
    std::env::var("HELIUS_API_KEY").unwrap(),
    Filters::new(
        vec![],
        Some(RpcTransactionsConfig {
            filter: TransactionSubscribeFilter {
                account_include: Some(vec![
                    ${rustJoin(programIds)}
                ]),
                account_exclude: None,
                account_required: None,
                vote: None,
                failed: None,
                signature: None,
            },
            options: TransactionSubscribeOptions {
                commitment: Some(TransactionCommitment::Confirmed),
                encoding: Some(UiEnhancedTransactionEncoding::Base64),
                transaction_details: Some(TransactionDetails::Full),
                show_rewards: None,
                max_supported_transaction_version: Some(0),
            },
        })
    ).unwrap(),
    std::sync::Arc::new(RwLock::new(HashSet::new())),
    Cluster::MainnetBeta,
)`.trim();

    return {
        id: 'helius_atlas_ws',
        imports,
        init,
        env: {
            required: ['HELIUS_API_KEY'],
            validate: () => {},
        },
    };
}



