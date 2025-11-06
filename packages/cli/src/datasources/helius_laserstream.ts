import type { DatasourceArtifact, DecoderMeta } from './index';

function rustJoin(items: string[]): string {
    return items.map(s => `${s},`).join('\n                    ');
}

export function buildHeliusLaserstream(decoders: DecoderMeta[]): DatasourceArtifact {
    const programIds = decoders.map(d => `${d.name.toUpperCase()}_PROGRAM_ID.to_string().clone()`);

    const imports = [
        'std::collections::{HashMap, HashSet}',
        'tokio::sync::RwLock',
        'yellowstone_grpc_proto::geyser::{CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions}',
        'carbon_helius_laserstream_datasource::LaserStreamClientConfig',
    ];

    const init = `
    carbon_helius_laserstream_datasource::LaserStreamGeyserClient::new(
        std::env::var("GEYSER_URL").unwrap_or_default(),
        std::env::var("X_TOKEN").ok(),
        Some(CommitmentLevel::Confirmed),
        {
            let mut account_filters: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();
            account_filters.insert(
                "account_filter".to_string(),
                SubscribeRequestFilterAccounts {
                    account: vec![],
                    owner: vec![
                        ${rustJoin(programIds)}
                    ],
                    filters: vec![],
                    nonempty_txn_signature: None,
                },
            );
            account_filters
        },
        {
            let transaction_filter = SubscribeRequestFilterTransactions {
                vote: Some(false),
                failed: Some(false),
                account_include: vec![],
                account_exclude: vec![],
                account_required: vec![
                    ${rustJoin(programIds)}
                ],
                signature: None,
            };
            let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> = HashMap::new();
            transaction_filters.insert("transaction_filter".to_string(), transaction_filter);
            transaction_filters
        },
        Default::default(),
        std::sync::Arc::new(RwLock::new(HashSet::new())),
        LaserStreamClientConfig::default(),
    )`.trim();

    return {
        id: 'helius_laserstream',
        imports,
        init,
        env: {
            required: ['GEYSER_URL'],
            validate: () => {
                const v = process.env.GEYSER_URL;
                if (!v || !v.trim()) throw new Error('Missing required environment variable: GEYSER_URL');
            },
        },
    };
}
