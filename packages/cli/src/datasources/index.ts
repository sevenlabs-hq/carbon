export type DecoderMeta = {
    name: string;
    module_name: string;
};

export type DatasourceArtifact = {
    id: string;
    imports: string[];
    init: string;
    env?: { required: string[]; validate: () => void };
};

export type DatasourceBuilder = (decoders: DecoderMeta[]) => DatasourceArtifact;

import { buildHeliusLaserstream } from './helius_laserstream';
import { buildRpcBlockSubscribe } from './rpc_block_subscribe';
import { buildYellowstoneGrpc } from './yellowstone_grpc';
import { buildRpcTransactionCrawler } from './rpc_transaction_crawler';
import { buildRpcProgramSubscribe } from './rpc_program_subscribe';

const REGISTRY: Record<string, DatasourceBuilder> = {
    helius_laserstream: buildHeliusLaserstream,
    rpc_block_subscribe: buildRpcBlockSubscribe,
    yellowstone_grpc: buildYellowstoneGrpc,
    rpc_transaction_crawler: buildRpcTransactionCrawler,
    rpc_program_subscribe: buildRpcProgramSubscribe,
};

export function getDatasourceBuilder(moduleName: string): DatasourceBuilder | undefined {
    return REGISTRY[moduleName];
}
