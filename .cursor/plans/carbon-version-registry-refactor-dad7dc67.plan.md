<!-- dad7dc67-2cd1-469e-9196-dee235867d2b bdea05c2-50ca-46f9-9552-bfc9daa5d968 -->

# Carbon Version Registry Refactor

## Objective

Create a centralized version registry package `@sevenlabs-hq/carbon-versions@0.11.0` that stores all Rust crate versions (Carbon, Solana, utilities) and refactor both `@sevenlabs-hq/carbon-cli` and `@sevenlabs-hq/carbon-codama-renderer` to consume these versions instead of hardcoding them.

## Implementation Plan

### 1. Create `packages/versions` package structure

- **Location**: `packages/versions/`
- **Package name**: `@sevenlabs-hq/carbon-versions`
- **Version**: `0.11.0` (matching Rust workspace version)
- **Structure**:
- `package.json` with proper exports
- `src/index.ts` exporting a single `VERSIONS` constant object
- `tsconfig.json` and `tsup.config.ts` for build setup

### 2. Define version constants

- **File**: `packages/versions/src/index.ts`
- Export a single constant object `VERSIONS` containing:
- `carbon`: `"0.11.0"` (Carbon crate version)
- `solana`: Object with all Solana crate versions:
- `pubkey`: `"^2.3.6"`
- `client`: `"^2.3.6"`
- `instruction`: `"^2.3.0"`
- `account`: `"~2.2"`
- `commitmentConfig`: `"^2.2.1"`
- `accountDecoder`: `"^2.3.6"`
- `utils`: Object with utility crate versions:
- `yellowstoneGrpcClient`: `"9.0.0"`
- `yellowstoneGrpcProto`: `"9.0.0"`
- `sqlx`: `"0.8.6"`
- `sqlxMigrator`: `"0.17.0"`
- `juniper`: `"0.15"`
- `axum`: `"0.8.4"`
- `rustls`: `"0.23"`
- `helius`: `"0.3.2"`

### 3. Update CLI package (`packages/cli`)

- **Add dependency**: Add `@sevenlabs-hq/carbon-versions` to `packages/cli/package.json`
- **Update scaffold.ts**:
- Import `VERSIONS` from `@sevenlabs-hq/carbon-versions`
- Replace hardcoded versions in `grpcDeps` (lines 142-143)
- Replace hardcoded versions in `pgDeps` (lines 146-147)
- Replace hardcoded versions in `gqlDeps` (line 150)
- Replace hardcoded version in `rustlsDep` (line 152)
- Replace hardcoded version in `atlasDeps` (line 153)
- Replace hardcoded versions in `crawlerDeps` (line 159)
- Replace hardcoded version in `programDeps` (line 160)
- **Update template context**: Pass version constants to Nunjucks templates in `renderScaffold()`
- **Update indexer-cargo.njk template**: Replace hardcoded Solana versions (lines 25-27) with template variables from context

### 4. Update Renderer package (`packages/renderer`)

- **Add dependency**: Add `@sevenlabs-hq/carbon-versions` to `packages/renderer/package.json`
- **Update cargo.njk template**: Replace hardcoded Solana versions (lines 30-32) with template variables
- **Update template rendering**: Modify template context to include version constants where `cargo.njk` is rendered

### 5. Update workspace configuration

- **pnpm-workspace.yaml**: Already includes `packages/*`, so new package will be automatically included
- Ensure versions package builds as part of monorepo

## Files to Create

- `packages/versions/package.json`
- `packages/versions/src/index.ts`
- `packages/versions/tsconfig.json`
- `packages/versions/tsup.config.ts`
- `packages/versions/README.md` (optional, for documentation)

## Files to Modify

- `packages/cli/package.json` (add dependency)
- `packages/cli/src/lib/scaffold.ts` (replace hardcoded versions)
- `packages/cli/templates/indexer-cargo.njk` (use template variables)
- `packages/renderer/package.json` (add dependency)
- `packages/renderer/templates/cargo.njk` (use template variables)
- `packages/renderer/src/getRenderMapVisitor.ts` or wherever templates are rendered (add versions to context)

## Key Design Decisions

- Single constant export (`VERSIONS`) for simplicity
- Nested structure (solana._, utils._) for organization
- Versions package matches Rust workspace version (0.11.0)
- Template variables passed via Nunjucks context (not imported directly in templates)

### To-dos

- [ ] Create packages/versions directory structure with package.json, src/index.ts, tsconfig.json, and tsup.config.ts
- [ ] Implement VERSIONS constant in src/index.ts with all Carbon, Solana, and utility crate versions
- [ ] Add @sevenlabs-hq/carbon-versions dependency to packages/cli/package.json
- [ ] Replace hardcoded versions in packages/cli/src/lib/scaffold.ts with VERSIONS constants
- [ ] Update packages/cli/templates/indexer-cargo.njk to use version template variables instead of hardcoded values
- [ ] Add @sevenlabs-hq/carbon-versions dependency to packages/renderer/package.json
- [ ] Update packages/renderer/templates/cargo.njk to use version template variables and ensure context includes versions
