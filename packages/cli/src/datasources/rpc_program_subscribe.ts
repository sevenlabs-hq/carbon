import type { DatasourceArtifact, DecoderMeta } from './index';

export function buildRpcProgramSubscribe(decoders: DecoderMeta[]): DatasourceArtifact {
    const firstProgram = decoders[0]?.name.toUpperCase() ?? 'PROGRAM_ID';

    const imports = [
        'carbon_rpc_program_subscribe_datasource::{Filters, RpcProgramSubscribe}',
        'solana_client::rpc_config::RpcProgramAccountsConfig',
    ];

    const init = `
{
    let filters = Filters::new(
        ${firstProgram}_PROGRAM_ID,
        None::<RpcProgramAccountsConfig>,
    );

    carbon_rpc_program_subscribe_datasource::RpcProgramSubscribe::new(
        std::env::var("RPC_WS_URL").unwrap_or("wss://api.mainnet-beta.solana.com/".to_string()),
        filters,
    )
}`.trim();

    return {
        id: 'rpc_program_subscribe',
        imports,
        init,
        env: {
            required: ['RPC_WS_URL'],
            validate: () => {},
        },
    };
}



