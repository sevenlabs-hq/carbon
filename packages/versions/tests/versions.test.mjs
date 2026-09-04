import assert from 'node:assert/strict';
import test from 'node:test';

import { CARBON_MSRV, CARBON_VERSION, VERSIONS } from '../dist/index.mjs';

const V2_DATASOURCES = [
    'carbon-helius-atlas-ws-datasource',
    'carbon-helius-gpa-v2-datasource',
    'carbon-helius-gtfa-datasource',
    'carbon-helius-laserstream-datasource',
    'carbon-rpc-block-crawler-datasource',
    'carbon-rpc-block-subscribe-datasource',
    'carbon-rpc-gpa-datasource',
    'carbon-rpc-program-subscribe-datasource',
    'carbon-rpc-transaction-crawler-datasource',
    'carbon-stream-message-datasource',
    'carbon-validator-snapshot-datasource',
    'carbon-yellowstone-grpc-datasource',
];

function versionOf(dependency) {
    return typeof dependency === 'string' ? dependency : dependency?.version;
}

test('pins the supported Carbon release line consistently', () => {
    assert.equal(CARBON_VERSION, '2.0.0');
    assert.equal(CARBON_MSRV, '1.96.1');

    for (const [crate, dependency] of Object.entries(VERSIONS).filter(([crate]) => crate.startsWith('carbon-'))) {
        assert.equal(versionOf(dependency), CARBON_VERSION, `${crate} must track Carbon ${CARBON_VERSION}`);
    }
});

test('lists all and only the supported v2 datasource crates', () => {
    const datasourceCrates = Object.keys(VERSIONS)
        .filter(crate => crate.startsWith('carbon-') && crate.endsWith('-datasource'))
        .sort();

    assert.deepEqual(datasourceCrates, [...V2_DATASOURCES].sort());
    assert.equal(VERSIONS['carbon-jetstreamer-datasource'], undefined);
    assert.equal(VERSIONS['carbon-jito-shredstream-grpc-datasource'], undefined);
});
