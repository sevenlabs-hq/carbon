import type { DatasourceArtifact, DecoderMeta } from './index';

export function buildRpcBlockSubscribe(_decoders: DecoderMeta[]): DatasourceArtifact {
    const imports = [
        'carbon_rpc_block_subscribe_datasource::{Filters, RpcBlockSubscribe}',
        'solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter}',
    ];

    const init = `
    {
        let filters = Filters::new(
            RpcBlockSubscribeFilter::All,
            Some(RpcBlockSubscribeConfig {
                max_supported_transaction_version: Some(0),
                ..RpcBlockSubscribeConfig::default()
            }),
        );

        let rpc_ws_url = std::env::var("RPC_WS_URL").unwrap_or("wss://api.mainnet-beta.solana.com/".to_string());
        log::info!("Starting with RPC: {}", rpc_ws_url);
        RpcBlockSubscribe::new(rpc_ws_url, filters)
    }`.trim();

    return {
        id: 'rpc_block_subscribe',
        imports,
        init,
        env: {
            required: ['RPC_WS_URL'],
            validate: () => {
                /* optional for this source */
            },
        },
    };
}
