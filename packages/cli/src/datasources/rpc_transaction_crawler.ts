import type { DatasourceArtifact, DecoderMeta } from './index';

export function buildRpcTransactionCrawler(decoders: DecoderMeta[]): DatasourceArtifact {
    const firstProgram = decoders[0]?.name.toUpperCase() ?? 'PROGRAM_ID';

    const imports = [
        'carbon_rpc_transaction_crawler_datasource::{ConnectionConfig, Filters, RetryConfig, RpcTransactionCrawler}',
        'solana_commitment_config::CommitmentConfig',
        'std::time::Duration',
    ];

    const init = `
    {
        let connection_config = ConnectionConfig::new(
            100,
            Duration::from_secs(5),
            5,
            RetryConfig::default(),
            None,
            None,
            false,
        );

        RpcTransactionCrawler::new(
            std::env::var("RPC_URL").unwrap_or_default(),
            ${firstProgram}_PROGRAM_ID,
            connection_config,
            Filters::new(None, None, None),
            Some(CommitmentConfig::finalized()),
        )
    }`.trim();

    return {
        id: 'rpc_transaction_crawler',
        imports,
        init,
        env: {
            required: ['RPC_URL'],
            validate: () => {},
        },
    };
}
