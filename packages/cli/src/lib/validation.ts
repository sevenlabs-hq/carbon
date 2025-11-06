import { exitWithError } from './utils';

export const VALID_DATASOURCES = [
    'helius-laserstream',
    'rpc-block-subscribe',
    'rpc-program-subscribe',
    'rpc-transaction-crawler',
    'yellowstone-grpc',
] as const;

export type DataSource = (typeof VALID_DATASOURCES)[number];
export type MetricsType = 'log' | 'prometheus';

export function validateDataSource(dataSource: string): asserts dataSource is DataSource {
    if (!VALID_DATASOURCES.includes(dataSource as DataSource)) {
        exitWithError(`Invalid data source. Must be one of: ${VALID_DATASOURCES.join(', ')}`);
    }
}

export function validateMetrics(metrics: string): asserts metrics is MetricsType {
    if (metrics !== 'log' && metrics !== 'prometheus') {
        exitWithError("Metrics must be 'log' or 'prometheus'");
    }
}
