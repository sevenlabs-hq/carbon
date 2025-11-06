<!-- 29c62c57-ce53-4613-805a-a7a07baf602a db872d73-61ea-4adf-bb63-7711e4e6c25d -->

# Phase 1: Remove Path-Based Crates

Remove all path-based crate dependencies from the VERSIONS registry and replace them with version-only dependencies using "0.11.0". Also remove the unused `dsPathDir` variable from scaffold.ts.

## Changes Required

### 1. Update VERSIONS Registry (`packages/versions/src/index.ts`)

Remove `path` properties from all Carbon crates and keep only version "0.11.0":

- `carbon-core`: Remove `path: "../../../crates/core"`, keep `version: "0.11.0"` and `defaultFeatures: false`
- `carbon-test-utils`: Remove `path: "../../../crates/test-utils"`, keep `version: "0.11.0"`
- `carbon-log-metrics`: Remove `path: "../../../metrics/log-metrics"`, keep `version: "0.11.0"`
- All datasource crates (helius-atlas-ws, helius-laserstream, jito-shredstream-grpc, rpc-block-crawler, rpc-block-subscribe, rpc-program-subscribe, rpc-transaction-crawler, stream-message, yellowstone-grpc): Remove `path` properties, keep `version: "0.11.0"`

Note: Keep git-based dependencies (yellowstone-grpc-client, yellowstone-grpc-proto, helius) as-is since they're external.

### 2. Remove dsPathDir from scaffold.ts (`packages/cli/src/lib/scaffold.ts`)

Remove the `dsPathDir` variable computation (lines 124-134) since it's not used anywhere in the codebase. The `datasourceCrateName` is already computed correctly on line 135.

## Implementation Notes

- All Carbon crates will use version "0.11.0" instead of path dependencies
- The `getCrateDependencyString` function already handles version-only dependencies correctly
- No changes needed to templates - they will automatically use version-based dependencies
- The local decoder path reference in `indexer-cargo.njk` (`path = "../decoder"`) should remain as-is since it's for local decoders in scaffolded projects, not published crates

### To-dos

- [ ] Remove all path properties from Carbon crates in packages/versions/src/index.ts, keeping only version '0.11.0'
- [ ] Remove unused dsPathDir variable computation from packages/cli/src/lib/scaffold.ts
