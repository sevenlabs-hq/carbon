# (1) Understanding Carbon CLI and Codama Renderer tasks

_Exported on 10/30/2025 at 19:47:22 GMT+5:30 from Cursor (1.7.46)_

---

**User**

Codama Renderer

1. Use ImportMap as much as possible instead of hardcoded imports

Carbon CLI

1. Use a dynamic import map instead of handling import code generation in Askama.

i want to understand what Carbon cli task meant? also whether its related codama rendere task only or not

@packages/

---

**Cursor**

I'll help you understand what the Carbon CLI task means and how it relates to the Codama Renderer task. Let me examine the packages to understand the current implementation and the proposed changes.

Now let me examine the ImportMap class and understand how it's currently being used:

Now let me look at the CLI package to understand how it currently handles import code generation:

Let me examine the decoder generation code more closely:

Now let me examine some template files to understand how imports are currently handled:

Now let me look at some individual account/instruction templates to see how imports are handled:

Now let me examine the current project template to understand how imports are handled there:

Based on my analysis of the codebase, I can now provide a comprehensive explanation of what these tasks mean and how they relate to each other.

## CLARITY INTERROGATION

Before I proceed, I need to understand:

1. **Current State**: How are imports currently being handled in both the Codama Renderer and Carbon CLI?
2. **Pain Points**: What specific issues exist with the current import handling approach?
3. **Scope**: Are these tasks independent or do they need to be coordinated?
4. **Implementation Details**: What would a "dynamic import map" look like in the CLI context?

## SELF-CHECK & INTERNAL ITERATION

After examining the codebase, I can see:

**Current Import Handling:**

- **Codama Renderer**: Uses `ImportMap` class to collect and manage imports, then renders them as `use` statements in templates
- **Carbon CLI**: Uses Nunjucks templates with hardcoded import logic in `project.njk` template

**Key Issues Identified:**

1. **Codama Renderer**: Import logic is scattered across multiple template files and visitor functions
2. **Carbon CLI**: Complex conditional import logic is embedded directly in Nunjucks templates, making them hard to maintain
3. **Duplication**: Similar import patterns are repeated across different templates
4. **Maintainability**: Changes to import requirements require updating multiple template files

## PRODUCTION-READY ANALYSIS

### **Codama Renderer Task: "Use ImportMap as much as possible instead of hardcoded imports"**

**What this means:**

- Currently, the renderer has some hardcoded imports scattered throughout templates
- The `ImportMap` class exists but isn't being used consistently
- The goal is to centralize all import management through the `ImportMap` system

**Current Issues:**

```12:14:packages/renderer/templates/accountsMod.njk
use crate::{PROGRAM_ID, {{ program.name | pascalCase }}Decoder};
```

```14:14:packages/renderer/templates/instructionsMod.njk
use crate::{PROGRAM_ID, {{ program.name | pascalCase }}Decoder};
```

These hardcoded imports should be managed through `ImportMap` instead.

### **Carbon CLI Task: "Use a dynamic import map instead of handling import code generation in Askama"**

**What this means:**

- Currently, the CLI uses Nunjucks templates (not Askama - this seems to be a terminology mix-up) to generate complex import statements
- The `project.njk` template contains extensive conditional logic for imports (lines 1-80)
- The goal is to move this logic out of templates and into a dynamic import management system

**Current Issues:**

```1:80:packages/cli/templates/project.njk
use {
    {%- if not withPostgres %}
    async_trait::async_trait,
    {%- endif %}
    carbon_core::{
        {%- if not withPostgres %}
        deserialize::ArrangeAccounts,
        {%- endif %}
        error::CarbonResult,
        // ... 70+ lines of conditional import logic
    },
    // ... more conditional imports
};
```

### **Relationship Between Tasks**

**YES, these tasks are related:**

1. **Shared Goal**: Both aim to centralize and improve import management
2. **Complementary Approach**:
    - Codama Renderer should use `ImportMap` for decoder generation
    - Carbon CLI should use a similar dynamic system for project scaffolding
3. **Consistency**: Both systems should use similar patterns for import management
4. **Maintainability**: Centralized import logic reduces duplication and errors

### **Proposed Solution Architecture**

```typescript
// Enhanced ImportMap for both systems
class ImportMap {
    private imports: Map<string, ImportInfo> = new Map();

    addImport(module: string, items: string[], conditions?: ImportCondition[]): this;
    addConditionalImport(condition: string, module: string, items: string[]): this;
    mergeWith(other: ImportMap): this;
    toString(context: RenderContext): string;
}

// For Carbon CLI - Project Import Manager
class ProjectImportManager {
    private baseImports: ImportMap;
    private conditionalImports: Map<string, ImportMap> = new Map();

    addDataSourceImports(dataSource: string): this;
    addPostgresImports(mode: 'generic' | 'typed'): this;
    addGraphQLImports(): this;
    addDecoderImports(decoders: DecoderInfo[]): this;

    generateImports(context: ProjectContext): string;
}
```

### **Implementation Strategy**

1. **Phase 1**: Enhance `ImportMap` class to support conditional imports
2. **Phase 2**: Refactor Codama Renderer to use `ImportMap` consistently
3. **Phase 3**: Create `ProjectImportManager` for Carbon CLI
4. **Phase 4**: Update CLI templates to use dynamic import generation
5. **Phase 5**: Remove hardcoded import logic from templates

## FINAL STATUS

**This is ready for implementation.** The tasks are clearly defined and related:

- **Codama Renderer**: Centralize import management using `ImportMap`
- **Carbon CLI**: Replace template-based import logic with dynamic import management
- **Coordination**: Both should use similar patterns for consistency

The solution will improve maintainability, reduce duplication, and make the codebase more robust by centralizing import logic instead of scattering it across templates.

```plaintext
use crate::{PROGRAM_ID, {{ program.name | pascalCase }}Decoder};
```

```plaintext
use crate::{PROGRAM_ID, {{ program.name | pascalCase }}Decoder};
```

```plaintext
use {
    {%- if not withPostgres %}
    async_trait::async_trait,
    {%- endif %}
    carbon_core::{
        {%- if not withPostgres %}
        deserialize::ArrangeAccounts,
        {%- endif %}
        error::CarbonResult,
        // ... 70+ lines of conditional import logic
    },
    // ... more conditional imports
};
```

```typescript
// Enhanced ImportMap for both systems
class ImportMap {
    private imports: Map<string, ImportInfo> = new Map();

    addImport(module: string, items: string[], conditions?: ImportCondition[]): this;
    addConditionalImport(condition: string, module: string, items: string[]): this;
    mergeWith(other: ImportMap): this;
    toString(context: RenderContext): string;
}

// For Carbon CLI - Project Import Manager
class ProjectImportManager {
    private baseImports: ImportMap;
    private conditionalImports: Map<string, ImportMap> = new Map();

    addDataSourceImports(dataSource: string): this;
    addPostgresImports(mode: 'generic' | 'typed'): this;
    addGraphQLImports(): this;
    addDecoderImports(decoders: DecoderInfo[]): this;

    generateImports(context: ProjectContext): string;
}
```

---

**User**

how much to be done on this task, can you go through all the files and write a file wise report

be consise

---

**Cursor**

Let me quickly scan renderer and CLI templates for hardcoded imports vs ImportMap usage to produce a concise file-wise report.

### CLARITY INTERROGATION

- Confirm scope: do you want only `packages/` reviewed (renderer + CLI), or also generated test/example outputs under `test/` and `examples/`?
- Is the goal to remove all hardcoded `use ...` blocks where feasible, or only those that depend on program/feature flags?

### FILE-WISE REPORT (concise)

Codama Renderer (packages/renderer)

- src/ImportMap.ts
    - Status: Good. Central utility already used widely. No change required unless adding conditional import helpers.

- src/getRenderMapVisitor.ts
    - Status: Good use of `ImportMap` for accounts, instructions, Postgres, GraphQL, and mod files via template contexts.
    - Action: None, except if you want to move mod-level imports into ImportMap (see templates below).

- src/getPostgresTypeManifestVisitor.ts
    - Status: Good. Returns `imports` via `ImportMap` for SQLx types.
    - Action: None.

- src/getGraphQLTypeManifestVisitor.ts
    - Status: Good. Returns `imports` via `ImportMap` for GraphQL scalars/types.
    - Action: None.

- src/renderVisitor.ts
    - Status: Good. Just wires the render visitor. No import logic here.
    - Action: None.

- templates/accountsPage.njk
    - Status: Uses `{{ imports }}`. OK.
    - Action: None.

- templates/instructionsPage.njk
    - Status: Uses `{{ imports }}`. OK.
    - Action: None.

- templates/postgresRowPage.njk
    - Status: Uses `{{ imports }}`. OK.
    - Action: None.

- templates/graphqlSchemaPage.njk
    - Status: Has hardcoded `use juniper::GraphQLObject;` plus `{{ imports }}`.
    - Action: Optional: move `juniper::GraphQLObject` into ImportMap to be consistent.

- templates/graphqlSchemaPageGeneric.njk
    - Status: Same as above; `use juniper::GraphQLObject;` hardcoded.
    - Action: Optional: move into ImportMap.

- templates/accountsMod.njk
    - Status: Hardcoded `use crate::{PROGRAM_ID, <Program>Decoder};`
    - Action: Optional: let visitor provide `imports` for mod files, then use `{{ imports }}` here.

- templates/instructionsMod.njk
    - Status: Hardcoded `use crate::{PROGRAM_ID, <Program>Decoder};`
    - Action: Optional: move to ImportMap via visitor.

- templates/instructionsPostgresMod.njk, accountsPostgresMod.njk
    - Status: Don’t contain explicit `use` blocks; no change.
    - Action: None.

- templates/graphqlQueryPage.njk / graphqlQueryPageGeneric.njk
    - Status: Likely template-specific, no `{{ imports }}` block. They bake in crate paths directly.
    - Action: Optional: if you want zero hardcoded imports, thread ImportMap for these too.

- templates/graphqlRootMod.njk, types*.njk, events*.njk
    - Status: Either no `use` or already rely on `{{ imports }}`.
    - Action: None.

Summary for Renderer: Mostly compliant. Only a few templates have hardcoded imports that could be moved into `ImportMap` for consistency: `graphqlSchemaPage*.njk`, `accountsMod.njk`, `instructionsMod.njk`, and optionally GraphQL root/query templates.

Carbon CLI (packages/cli)

- templates/project.njk
    - Status: Heavy hardcoded/conditional import block at top (data source, metrics, postgres mode, graphql).
    - Action: High priority. Replace with a single `{{ imports }}` line and compute imports in TS.

- src/lib/scaffold.ts
    - Status: Builds template context and renders `project.njk`. No import computation.
    - Action: Add a dynamic import builder in TS (derive imports based on `withPostgres`, `useGenericPostgres`, `withGraphql`, `data_source`, `decoders`, `metrics`), inject `imports: importMap.toString()` into `mainContext`, and simplify `project.njk`.

- src/lib/decoder.ts
    - Status: Delegates rendering to the renderer; no import generation for `main.rs`.
    - Action: None.

- src/cli.ts
    - Status: Orchestrates; no import generation.
    - Action: None.

### SELF-CHECK & INTERNAL ITERATION

- Alternatives: Keep a minimal set of stable hardcoded imports (e.g., juniper derive) in templates vs. move everything into ImportMap for full consistency. Moving everything reduces drift but adds minor visitor complexity for mod/root files.
- Risks: Introducing ImportMap for CLI scaffolding must mirror current conditional behavior exactly (Postgres generic/typed, data sources specifics, GraphQL presence). Ensure test scaffolds still compile across all matrix configs.

### PRODUCTION-READY DESIGN (concise)

- Renderer: Extend visitor to emit ImportMap for mod and GraphQL schema files; update those templates to use `{{ imports }}`. Optionally add `juniper::GraphQLObject` to the GraphQL manifests.
- CLI: Implement `buildProjectImports(opts): ImportMap` in `scaffold.ts`, compute imports dynamically, pass `imports` string to `project.njk`, and collapse the 70+ line `use { ... }` block to `{{ imports }}`.

### FINAL STATUS

- Renderer: Small edits across 3–5 templates; optional for full consistency.
- CLI: One significant change (dynamic import builder in `scaffold.ts`) and one template simplification (`project.njk`).

---

**User**

build a plan

---

**Cursor**

### CLARITY INTERROGATION

1. Do you want ImportMap applied to all renderer templates including mod files and GraphQL templates, or only where imports are already dynamic?
    - a) All renderer templates
    - b) Only where `{{ imports }}` already exists
2. For the CLI, should the dynamic import map cover the full matrix (all datasources, Postgres generic/typed, GraphQL on/off, metrics), matching current template behavior?
    - a) Yes, full parity
    - b) Start with a subset (specify)

### SELF-CHECK & INTERNAL ITERATION

- Alternative: keep a few stable hardcoded imports (e.g., juniper derives) in templates; but full centralization via ImportMap is cleaner and consistent.
- Risks: Missing an edge-condition in CLI dynamic import logic (e.g., specific datasource imports, GraphQL generics vs typed). We must mirror current conditions exactly to avoid compile errors.

### PRODUCTION-READY DESIGN

- Renderer: centralize imports via `ImportMap` everywhere (including `accountsMod.njk`, `instructionsMod.njk`, `graphqlSchemaPage*.njk`, optionally `graphqlQueryPage*.njk`), with the visitor providing `imports` for these pages. Add `juniper::GraphQLObject` to the GraphQL type manifest or a dedicated ImportMap step.
- CLI: add a TypeScript import builder in `src/lib/scaffold.ts` to compute imports based on `withPostgres`, `useGenericPostgres`, `withGraphql`, `dataSource`, `metrics`, and `decoders`. Pass `imports: importMap.toString()` into `project.njk`, and replace the large `use { ... }` block with `{{ imports }}`.

### FINAL STATUS

STOPPED: Awaiting your answers to the two questions above. I’ll produce a concise, actionable plan with exact file edits once confirmed.

---

**User**

1a 2a

make it two phases
1 renderer 2 cli

Implement the plan as specified. To-do's from the plan have already been created, do not create them again.

---

**Cursor**

I'll start by updating the renderer: add ImportMap-driven imports for mod and GraphQL templates, then modify the templates to consume {{ imports }}. After that, I'll update the CLI scaffold to compute imports dynamically and simplify project.njk to use {{ imports }}.

```typescript
import {
    camelCase,
    DefinedTypeNode,
    EnumTypeNode,
    getAllAccounts,
    getAllDefinedTypes,
    getAllInstructionsWithSubs,
    getAllPrograms,
    isNode,
    pascalCase,
    ProgramNode,
    snakeCase,
    SnakeCaseString,
    structFieldTypeNode,
    structTypeNode,
    TypeNode,
} from '@codama/nodes';
import { RenderMap } from '@codama/renderers-core';
import { extendVisitor, pipe, staticVisitor, visit } from '@codama/visitors-core';

import { DiscriminatorManifest, getDiscriminatorManifest, getTypeManifestVisitor } from './getTypeManifestVisitor';
import { getGraphQLTypeManifestVisitor } from './getGraphQLTypeManifestVisitor';
import { ImportMap } from './ImportMap';
import { partition, render } from './utils';
import { getPostgresTypeManifestVisitor, PostgresTypeManifest } from './getPostgresTypeManifestVisitor';
import { FlattenedGraphQLField, flattenTypeForGraphQL } from './utils/flattenGraphqlFields';

export type GetRenderMapOptions = {
    renderParentInstructions?: boolean;
    packageName?: string;
    anchorEvents?: {
        name: string;
        discriminator: number[];
    }[];
    postgresMode?: 'generic' | 'typed';
};

type FlattenedField = {
    column: string;
    rustPath: string;
    rowType: string;
    postgresColumnType: string;
    expr?: string;
    reverseExpr?: string;
    docs: string[];
    postgresManifest: PostgresTypeManifest;
};

export function getRenderMapVisitor(options: GetRenderMapOptions = {}) {
    const renderParentInstructions = options.renderParentInstructions ?? false;
    const typeManifestVisitor = getTypeManifestVisitor();
    const postgresTypeManifestVisitor = getPostgresTypeManifestVisitor();

    let currentProgram: ProgramNode | null = null;

    return pipe(
        staticVisitor(() => new RenderMap(), {
            keys: ['rootNode', 'programNode', 'instructionNode', 'accountNode', 'definedTypeNode'],
        }),
        v =>
            extendVisitor(v, {
                visitAccount(node) {
                    let discriminators = node.discriminators ?? [];

                    let newNode = node;

                    if (node.data.kind == 'structTypeNode') {
                        const [discriminatorArguments, regularArguments] = partition(
                            node.data.fields,
                            arg => arg.name == 'discriminator',
                        );

                        newNode = {
                            ...node,
                            data: {
                                ...node.data,
                                fields: regularArguments,
                            },
                        };

                        for (const discriminatorArgument of discriminatorArguments) {
                            if (discriminatorArgument.defaultValue) {
                                for (let i = 0; i < discriminators.length; i++) {
                                    const discriminator = discriminators[i];
                                    if (
                                        discriminator.kind === 'fieldDiscriminatorNode' &&
                                        discriminator.name === discriminatorArgument.name
                                    ) {
                                        discriminators[i] = {
                                            kind: 'constantDiscriminatorNode',
                                            offset: discriminator.offset,
                                            constant: {
                                                kind: 'constantValueNode',
                                                type: discriminatorArgument.type,
                                                value: discriminatorArgument.defaultValue as any,
                                            },
                                        };
                                    }
                                }
                            }
                        }
                    }

                    const typeManifest = visit(newNode.data, typeManifestVisitor);
                    const imports = new ImportMap()
                        .mergeWithManifest(typeManifest)
                        .add('carbon_core::borsh::{self, BorshDeserialize}');

                    const discriminatorManifest =
                        discriminators.length > 0 ? getDiscriminatorManifest(discriminators) : undefined;

                    // Postgres generation
                    const flatFields = flattenType(newNode.data, [], [], new Set());
                    const postgresImports = new ImportMap()
                        .add(`crate::accounts::${snakeCase(node.name)}::${pascalCase(node.name)}`)
                        .add('carbon_core::account::AccountMetadata')
                        .add('carbon_core::postgres::metadata::AccountRowMetadata');
                    flatFields.forEach(f => {
                        postgresImports.mergeWith(f.postgresManifest.imports);
                    });

                    let renderMap = new RenderMap().add(
                        `src/accounts/${snakeCase(node.name)}.rs`,
                        render('accountsPage.njk', {
                            account: newNode,
                            imports: imports.toString(),
                            program: currentProgram,
                            discriminatorManifest,
                            typeManifest,
                        }),
                    );

                    // Only generate postgres files if not in generic mode
                    if (options.postgresMode !== 'generic') {
                        renderMap.add(
                            `src/accounts/postgres/${snakeCase(node.name)}_row.rs`,
                            render('postgresRowPage.njk', {
                                entityDocs: node.docs,
                                entityName: node.name,
                                imports: postgresImports.toString(),
                                flatFields,
                                isAccount: true,
                            }),
                        );
                    }

                    // GraphQL generation
                    const graphqlFields = flattenTypeForGraphQL(newNode.data, [], [], new Set());
                    const graphqlImports = new ImportMap();
                    graphqlFields.forEach((f: FlattenedGraphQLField) => {
                        graphqlImports.mergeWith(f.graphqlManifest.imports);
                    });
                    // Ensure GraphQL derive is imported consistently via ImportMap
                    graphqlImports.add('juniper::GraphQLObject');

                    // GraphQLObject doesn't support empty structs
                    if (graphqlFields.length > 0) {
                        const schemaTemplate =
                            options.postgresMode === 'generic'
                                ? 'graphqlSchemaPageGeneric.njk'
                                : 'graphqlSchemaPage.njk';
                        renderMap.add(
                            `src/accounts/graphql/${snakeCase(node.name)}_schema.rs`,
                            render(schemaTemplate, {
                                entityDocs: node.docs,
                                entityName: node.name,
                                imports: graphqlImports.toString(),
                                graphqlFields,
                                isAccount: true,
                            }),
                        );
                    }

                    return renderMap;
                },

                visitDefinedType(node) {
                    const typeManifest = visit(node.type, typeManifestVisitor);
                    const imports = new ImportMap().mergeWithManifest(typeManifest);
                    // Only import borsh if the type is a struct or enum, to have clippy not complain
                    if (node.type.kind === 'structTypeNode' || node.type.kind === 'enumTypeNode') {
                        imports.add('carbon_core::borsh');
                    }

                    let renderMap = new RenderMap().add(
                        `src/types/${snakeCase(node.name)}.rs`,
                        render('typesPage.njk', {
                            definedType: node,
                            imports: imports.toString(),
                            typeManifest,
                        }),
                    );

                    for (let event of options.anchorEvents ?? []) {
                        imports.add('carbon_core::borsh::BorshDeserialize');

                        if (camelCase(event.name) == node.name) {
                            let discriminatorManifest: DiscriminatorManifest = {
                                bytes: `[${event.discriminator.join(', ')}]`,
                                size: event.discriminator.length,
                                checkCode: `
                                    if data.len() < ${event.discriminator.length} {
                                        return None;
                                    }
                                    let discriminator = &data[0..${event.discriminator.length}];
                                    if discriminator != &[${event.discriminator.join(', ')}] {
                                        return None;
                                    }
                                `,
                            };

                            renderMap.add(
                                `src/events/${snakeCase(node.name)}.rs`,
                                render('eventsPage.njk', {
                                    event: node,
                                    imports: imports.toString(),
                                    typeManifest,
                                    discriminatorManifest,
                                }),
                            );
                        }
                    }

                    // GraphQL generation for structs and enums
                    if (node.type.kind === 'structTypeNode') {
                        if (node.type.fields.length > 0) {
                            const graphqlFields = flattenTypeForGraphQL(node.type, [], [], new Set());
                            const graphqlImports = new ImportMap();
                            graphqlFields.forEach((f: FlattenedGraphQLField) => {
                                graphqlImports.mergeWith(f.graphqlManifest.imports);
                            });

                            renderMap.add(
                                `src/types/graphql/${snakeCase(node.name)}_schema.rs`,
                                render('graphqlTypeSchemaPage.njk', {
                                    entityDocs: node.docs,
                                    entityName: node.name,
                                    imports: graphqlImports.toString(),
                                    graphqlFields,
                                    isAccount: false,
                                }),
                            );
                        } else {
                            renderMap.add(
                                `src/types/graphql/${snakeCase(node.name)}_schema.rs`,
                                render('graphqlEmptyStructSchemaPage.njk', {
                                    entityDocs: node.docs,
                                    entityName: node.name,
                                }),
                            );
                        }
                    } else if (node.type.kind === 'enumTypeNode') {
                        const isFieldless = node.type.variants.every(v => v.kind === 'enumEmptyVariantTypeNode');
                        const imports = new ImportMap();
                        if (!isFieldless) {
                            imports.add('serde_json');
                            imports.add('carbon_core::graphql::primitives::Json');
                        }
                        renderMap.add(
                            `src/types/graphql/${snakeCase(node.name)}_schema.rs`,
                            render('graphqlEnumSchemaPage.njk', {
                                entityDocs: node.docs,
                                entityName: node.name,
                                imports: imports.toString(),
                                isFieldless,
                                variants: node.type.variants.map(v => ({
                                    name: v.name,
                                    docs: [],
                                })),
                            }),
                        );
                    } else {
                        // For type aliases, use GraphQL type manifest to get proper GraphQL types
                        const graphqlManifest = visit(node.type, getGraphQLTypeManifestVisitor());
                        const imports = graphqlManifest.imports.toString();
                        const importSection = imports ? `${imports}\n\n` : '';

                        renderMap.add(
                            `src/types/graphql/${snakeCase(node.name)}_schema.rs`,
                            `${importSection}pub type ${pascalCase(node.name)}GraphQL = ${graphqlManifest.graphqlType};\n`,
                        );
                    }

                    return renderMap;
                },

                visitInstruction(node) {
                    const imports = new ImportMap()
                        .add('carbon_core::borsh::{self, BorshDeserialize}')
                        .add('carbon_core::deserialize::ArrangeAccounts')
                        .add('carbon_core::account_utils::next_account');

                    const [discriminatorArguments, regularArguments] = partition(
                        node.arguments,
                        arg => arg.name == 'discriminator',
                    );

                    // Collect all types from arguments
                    const argumentTypes = regularArguments.map(arg => {
                        const manifest = visit(arg.type, typeManifestVisitor);
                        imports.mergeWithManifest(manifest);
                        return manifest;
                    });

                    let discriminators = node.discriminators ?? [];

                    for (const discriminatorArgument of discriminatorArguments) {
                        if (discriminatorArgument.defaultValue) {
                            for (let i = 0; i < discriminators.length; i++) {
                                const discriminator = discriminators[i];
                                if (
                                    discriminator.kind === 'fieldDiscriminatorNode' &&
                                    discriminator.name === discriminatorArgument.name
                                ) {
                                    discriminators[i] = {
                                        kind: 'constantDiscriminatorNode',
                                        offset: discriminator.offset,
                                        constant: {
                                            kind: 'constantValueNode',
                                            type: discriminatorArgument.type,
                                            value: discriminatorArgument.defaultValue as any,
                                        },
                                    };
                                }
                            }
                        }
                    }

                    const newNode = {
                        ...node,
                        arguments: regularArguments,
                        discriminators,
                    };

                    const uniqueAccounts = [];
                    const seenFieldNames = new Set();

                    for (const account of newNode.accounts) {
                        const fieldName = snakeCase(account.name);
                        if (!seenFieldNames.has(fieldName)) {
                            seenFieldNames.add(fieldName);
                            uniqueAccounts.push(account);
                        }
                    }

                    const instructionWithUniqueAccounts = {
                        ...newNode,
                        accounts: uniqueAccounts,
                    };

                    const discriminatorManifest = getDiscriminatorManifest(discriminators);

                    // Postgres generation
                    const flatFields = flattenType(
                        structTypeNode(
                            newNode.arguments.map(a =>
                                structFieldTypeNode({
                                    type: a.type,
                                    name: a.name,
                                }),
                            ),
                        ),
                        [],
                        [],
                        new Set(),
                    );
                    const postgresImports = new ImportMap()
                        .add(`crate::instructions::${pascalCase(node.name)}`)
                        .add('carbon_core::instruction::InstructionMetadata')
                        .add('carbon_core::postgres::metadata::InstructionRowMetadata');
                    flatFields.forEach(f => {
                        postgresImports.mergeWith(f.postgresManifest.imports);
                    });

                    let renderMap = new RenderMap().add(
                        `src/instructions/${snakeCase(node.name)}.rs`,
                        render('instructionsPage.njk', {
                            argumentTypes,
                            imports: imports.toString(),
                            instruction: instructionWithUniqueAccounts,
                            discriminatorManifest,
                            program: currentProgram,
                        }),
                    );

                    // Only generate postgres files if not in generic mode
                    if (options.postgresMode !== 'generic') {
                        renderMap.add(
                            `src/instructions/postgres/${snakeCase(node.name)}_row.rs`,
                            render('postgresRowPage.njk', {
                                entityDocs: node.docs,
                                entityName: node.name,
                                imports: postgresImports.toString(),
                                flatFields,
                                isAccount: false,
                            }),
                        );
                    }

                    // GraphQL generation
                    const graphqlFields = flattenTypeForGraphQL(
                        structTypeNode(
                            newNode.arguments.map(a =>
                                structFieldTypeNode({
                                    type: a.type,
                                    name: a.name,
                                }),
                            ),
                        ),
                        [],
                        [],
                        new Set(),
                    );
                    const graphqlImports = new ImportMap();
                    graphqlFields.forEach((f: FlattenedGraphQLField) => {
                        graphqlImports.mergeWith(f.graphqlManifest.imports);
                    });
                    // Ensure GraphQL derive is imported consistently via ImportMap
                    graphqlImports.add('juniper::GraphQLObject');

                    // GraphQLObject doesn't support empty structs
                    if (graphqlFields.length > 0) {
                        const schemaTemplate =
                            options.postgresMode === 'generic'
                                ? 'graphqlSchemaPageGeneric.njk'
                                : 'graphqlSchemaPage.njk';
                        renderMap.add(
                            `src/instructions/graphql/${snakeCase(node.name)}_schema.rs`,
                            render(schemaTemplate, {
                                entityDocs: node.docs,
                                entityName: node.name,
                                imports: graphqlImports.toString(),
                                graphqlFields,
                                isAccount: false,
                            }),
                        );
                    }

                    return renderMap;
                },

                visitProgram(node, { self }) {
                    currentProgram = node;
                    const renderMap = new RenderMap()
                        .mergeWith(...node.accounts.map(account => visit(account, self)))
                        .mergeWith(...node.definedTypes.map(type => visit(type, self)))
                        .mergeWith(
                            ...getAllInstructionsWithSubs(node, {
                                leavesOnly: !renderParentInstructions,
                            }).map(ix => visit(ix, self)),
                        );

                    currentProgram = null;
                    return renderMap;
                },

                visitRoot(node, { self }) {
                    // Only use the main program, ignore additionalPrograms
                    const program = node.program;

                    if (!program) {
                        throw new Error('No program found in IDL');
                    }

                    // Use getAll* functions but they will only process the main program
                    const accountsToExport = getAllAccounts(node);
                    const instructionsToExport = getAllInstructionsWithSubs(node, {
                        leavesOnly: !renderParentInstructions,
                    });
                    const definedTypesToExport = getAllDefinedTypes(node);

                    const ctx = {
                        accountsToExport,
                        definedTypesToExport,
                        instructionsToExport,
                        program,
                        root: node,
                        packageName: options.packageName,
                        hasAnchorEvents: options.anchorEvents?.length ?? 0 > 0,
                        events: options.anchorEvents ?? [],
                        postgresMode: options.postgresMode || 'typed',
                    };

                    const map = new RenderMap();

                    // Generate mod files
                    // Build mod-level imports via ImportMap
                    const accountsModImports = new ImportMap()
                        .add('crate::PROGRAM_ID')
                        .add(`crate::${pascalCase(program.name)}Decoder`);
                    map.add(
                        'src/accounts/mod.rs',
                        render('accountsMod.njk', { ...ctx, imports: accountsModImports.toString() }),
                    );
                    if (options.postgresMode !== 'generic') {
                        map.add('src/accounts/postgres/mod.rs', render('accountsPostgresMod.njk', ctx));
                    }
                    const accountsGraphqlTemplate =
                        options.postgresMode === 'generic' ? 'accountsGraphqlModGeneric.njk' : 'accountsGraphqlMod.njk';
                    map.add('src/accounts/graphql/mod.rs', render(accountsGraphqlTemplate, ctx));
                    if (instructionsToExport.length > 0) {
                        const instructionsModImports = new ImportMap()
                            .add('crate::PROGRAM_ID')
                            .add(`crate::${pascalCase(program.name)}Decoder`);
                        map.add(
                            'src/instructions/mod.rs',
                            render('instructionsMod.njk', { ...ctx, imports: instructionsModImports.toString() }),
                        );
                        if (options.postgresMode !== 'generic') {
                            map.add('src/instructions/postgres/mod.rs', render('instructionsPostgresMod.njk', ctx));
                        }
                        const instructionsGraphqlTemplate =
                            options.postgresMode === 'generic'
                                ? 'instructionsGraphqlModGeneric.njk'
                                : 'instructionsGraphqlMod.njk';
                        map.add('src/instructions/graphql/mod.rs', render(instructionsGraphqlTemplate, ctx));
                    }

                    if (options.anchorEvents?.length ?? 0 > 0) {
                        map.add('src/instructions/cpi_event.rs', render('eventInstructionPage.njk', ctx));
                        if (options.postgresMode !== 'generic') {
                            map.add(
                                'src/instructions/postgres/cpi_event_row.rs',
                                render('eventInstructionRowPage.njk', ctx),
                            );
                        }
                        const cpiEventSchemaTemplate =
                            options.postgresMode === 'generic'
                                ? 'eventInstructionGraphqlSchemaPageGeneric.njk'
                                : 'eventInstructionGraphqlSchemaPage.njk';
                        map.add('src/instructions/graphql/cpi_event_schema.rs', render(cpiEventSchemaTemplate, ctx));
                        map.add('src/events/mod.rs', render('eventsMod.njk', ctx));
                    }

                    if (definedTypesToExport.length > 0) {
                        map.add('src/types/mod.rs', render('typesMod.njk', ctx));
                        map.add('src/types/graphql/mod.rs', render('typesGraphqlMod.njk', ctx));
                    }

                    // GraphQL root (context + query)
                    map.add('src/graphql/mod.rs', render('graphqlRootMod.njk', ctx));
                    map.add('src/graphql/context.rs', render('graphqlContextPage.njk', ctx));

                    // Use different query template based on postgres mode
                    if (options.postgresMode === 'generic') {
                        map.add('src/graphql/query.rs', render('graphqlQueryPageGeneric.njk', ctx));
                    } else {
                        map.add('src/graphql/query.rs', render('graphqlQueryPage.njk', ctx));
                    }

                    // Generate lib.rs
                    map.add('src/lib.rs', render('lib.njk', ctx));

                    // Generate Cargo.toml
                    map.add('Cargo.toml', render('cargo.njk', ctx));

                    // Process all programs
                    return map.mergeWith(...getAllPrograms(node).map(p => visit(p, self)));
                },
            }),
    );

    function flattenType(
        typeNode: TypeNode,
        prefix: string[],
        docsPrefix: string[],
        seen: Set<string>,
        opts: { inOption?: boolean } = {},
    ): FlattenedField[] {
        const out: FlattenedField[] = [];

        const { inOption } = opts;

        const makeName = (nameParts: string[]) => {
            let col = snakeCase(nameParts.join('_'));
            if (seen.has(col)) {
                let i = 1;
                while (seen.has(`${col}_${i}`)) i++;
                col = `${col}_${i}` as SnakeCaseString;
            }
            seen.add(col);
            return col;
        };

        if (isNode(typeNode, 'structTypeNode')) {
            for (const field of typeNode.fields) {
                out.push(...flattenType(field.type, [...prefix, snakeCase(field.name)], [], seen, { inOption }));
            }
            return out;
        }

        if (isNode(typeNode, 'optionTypeNode')) {
            const column = makeName(prefix);
            const manifest = visit(typeNode.item, postgresTypeManifestVisitor) as PostgresTypeManifest;
            const isJson = (manifest.postgresColumnType || '').toUpperCase().startsWith('JSONB');

            const rowType = isJson
                ? manifest.sqlxType.includes('Json<')
                    ? `Option<${manifest.sqlxType}>`
                    : `Option<sqlx::types::Json<${manifest.sqlxType}>>`
                : `Option<${manifest.sqlxType}>`;

            const expr = isJson
                ? manifest.sqlxType.includes('Json<')
                    ? `${`source.${prefix.join('.')}`}.map(|value| value.into())`
                    : `${`source.${prefix.join('.')}`}.map(|value| sqlx::types::Json(value.into()))`
                : `${`source.${prefix.join('.')}`}.map(|value| value.into())`;

            // Handle reverse conversion based on inner type
            const reverseExpr = isJson
                ? `${`source.${column}`}.map(|value| value.0)` // Always single unwrap for JSONB types
                : buildReverseOptionType(typeNode, `source.${column}`, manifest);

            out.push({
                column,
                rustPath: prefix.join('.'),
                rowType,
                postgresColumnType: `${manifest.postgresColumnType}`,
                docs: docsPrefix,
                postgresManifest: manifest,
                expr,
                reverseExpr,
            });

            return out;
        }

        // Handle zeroableOptionTypeNode, remainderOptionTypeNode - same as optionTypeNode
        if (isNode(typeNode, 'zeroableOptionTypeNode') || isNode(typeNode, 'remainderOptionTypeNode')) {
            const column = makeName(prefix);
            const manifest = visit(typeNode.item, postgresTypeManifestVisitor) as PostgresTypeManifest;
            const isJson = (manifest.postgresColumnType || '').toUpperCase().startsWith('JSONB');

            const rowType = isJson
                ? manifest.sqlxType.includes('Json<')
                    ? `Option<${manifest.sqlxType}>`
                    : `Option<sqlx::types::Json<${manifest.sqlxType}>>`
                : `Option<${manifest.sqlxType}>`;

            const expr = isJson
                ? manifest.sqlxType.includes('Json<')
                    ? `${`source.${prefix.join('.')}`}.map(|value| value.into())`
                    : `${`source.${prefix.join('.')}`}.map(|value| sqlx::types::Json(value.into()))`
                : `${`source.${prefix.join('.')}`}.map(|value| value.into())`;

            // Handle reverse conversion based on inner type
            const reverseExpr = isJson
                ? `${`source.${column}`}.map(|value| value.0)` // Always single unwrap for JSONB types
                : buildReverseOptionType(typeNode, `source.${column}`, manifest);

            out.push({
                column,
                rustPath: prefix.join('.'),
                rowType,
                postgresColumnType: `${manifest.postgresColumnType}`,
                docs: docsPrefix,
                postgresManifest: manifest,
                expr,
                reverseExpr,
            });

            return out;
        }

        // Handle hiddenPrefixTypeNode - unwrap and process inner type
        if (isNode(typeNode, 'hiddenPrefixTypeNode')) {
            return flattenType(typeNode.type, prefix, docsPrefix, seen, opts);
        }

        if (isNode(typeNode, 'definedTypeLinkNode')) {
            const column = makeName(prefix);
            const manifest = visit(typeNode, postgresTypeManifestVisitor) as PostgresTypeManifest;
            const isJson = (manifest.postgresColumnType || '').toUpperCase().startsWith('JSONB');

            const rowType = isJson ? `sqlx::types::Json<${manifest.sqlxType}>` : `${manifest.sqlxType}`;

            const expr = isJson
                ? `sqlx::types::Json(${`source.${prefix.join('.')}`}.into())`
                : `${`source.${prefix.join('.')}`}.into()`;

            const reverseExpr = isJson ? `${`source.${column}`}.0` : `${`source.${column}`}.into()`;

            out.push({
                column,
                rustPath: prefix.join('.'),
                rowType,
                postgresColumnType: `${manifest.postgresColumnType} NOT NULL`,
                docs: docsPrefix,
                postgresManifest: manifest,
                expr,
                reverseExpr,
            });
            return out;
        }

        const manifest = visit(typeNode, postgresTypeManifestVisitor) as PostgresTypeManifest;
        const column = makeName(prefix);

        const field: FlattenedField = {
            column,
            rustPath: prefix.join('.'),
            rowType: manifest.sqlxType,
            postgresColumnType: `${manifest.postgresColumnType} NOT NULL`,
            docs: docsPrefix,
            postgresManifest: manifest,
        };

        field.expr = buildExpression(typeNode, `source.${field.rustPath}`);
        field.reverseExpr = buildReverse(typeNode, `source.${field.rustPath}`);

        out.push(field);

        return out;
    }

    function buildExpression(typeNode: TypeNode, prefix: string): string {
        if (isNode(typeNode, 'arrayTypeNode')) {
            if (
                isNode(typeNode.item, 'numberTypeNode') ||
                isNode(typeNode.item, 'booleanTypeNode') ||
                isNode(typeNode.item, 'bytesTypeNode') ||
                isNode(typeNode.item, 'stringTypeNode') ||
                isNode(typeNode.item, 'publicKeyTypeNode')
            ) {
                return `${prefix}.into_iter().map(|element| element.into()).collect()`;
            } else {
                return `sqlx::types::Json(${prefix}.into_iter().map(|element| ${buildExpression(typeNode.item, `element`)}).collect())`;
            }
        } else if (
            isNode(typeNode, 'optionTypeNode') ||
            isNode(typeNode, 'zeroableOptionTypeNode') ||
            isNode(typeNode, 'remainderOptionTypeNode')
        ) {
            return `${prefix}.map(|value| ${buildExpression(typeNode.item, `value`)})`;
        } else if (isNode(typeNode, 'hiddenPrefixTypeNode')) {
            return buildExpression(typeNode.type, prefix);
        } else if (isNode(typeNode, 'tupleTypeNode')) {
            if (typeNode.items.length === 1) {
                return `${buildExpression(typeNode.items[0], `${prefix}`)}`;
            }
            return `(${typeNode.items.map((item, i) => buildExpression(item, `${prefix}.${i}`)).join(', ')})`;
        } else {
            return `${prefix}.into()`;
        }
    }

    function buildReverseOptionType(typeNode: TypeNode, prefix: string, manifest: PostgresTypeManifest): string {
        if (
            !isNode(typeNode, 'optionTypeNode') &&
            !isNode(typeNode, 'zeroableOptionTypeNode') &&
            !isNode(typeNode, 'remainderOptionTypeNode')
        ) {
            throw new Error('buildReverseOptionType should only be called for option-like types');
        }

        const innerType = typeNode.item;

        if (isNode(innerType, 'booleanTypeNode')) {
            return `${prefix}.map(|value| value)`;
        } else if (isNode(innerType, 'numberTypeNode')) {
            const isPostgresPrimitive =
                manifest.sqlxType.includes('U8') ||
                manifest.sqlxType.includes('U16') ||
                manifest.sqlxType.includes('U32') ||
                manifest.sqlxType.includes('U64') ||
                manifest.sqlxType.includes('I128') ||
                manifest.sqlxType.includes('U128');

            if (isPostgresPrimitive) {
                if (manifest.sqlxType.includes('U16')) {
                    return `${prefix}.map(|value| *value as u16)`;
                } else if (manifest.sqlxType.includes('U32')) {
                    return `${prefix}.map(|value| *value as u32)`;
                } else if (manifest.sqlxType.includes('U8')) {
                    return `${prefix}.map(|value| *value as u8)`;
                } else {
                    return `${prefix}.map(|value| *value)`;
                }
            } else {
                return `${prefix}.map(|value| value)`;
            }
        } else if (isNode(innerType, 'publicKeyTypeNode')) {
            return `${prefix}.map(|value| *value)`;
        } else if (isNode(innerType, 'stringTypeNode') || isNode(innerType, 'bytesTypeNode')) {
            return `${prefix}.map(|value| *value)`;
        } else {
            return `${prefix}.map(|value| value.into())`;
        }
    }

    function buildReverse(typeNode: TypeNode, prefix: string): string {
        // Postgres reverse mapping (Row → Rust):
        // - Primitive arrays: map elements and try_into for fixed-size
        // - Json arrays: unwrap `.0` once then map recursively
        // - Fixed-size arrays: collect Result<Vec<_>, _> before try_into
        if (isNode(typeNode, 'arrayTypeNode')) {
            const isJson = !(
                isNode(typeNode.item, 'numberTypeNode') ||
                isNode(typeNode.item, 'booleanTypeNode') ||
                isNode(typeNode.item, 'bytesTypeNode') ||
                isNode(typeNode.item, 'stringTypeNode') ||
                isNode(typeNode.item, 'publicKeyTypeNode')
            );

            switch (typeNode.count.kind) {
                // our target type is [T; N], T is typeNode.item, N is typeNode.count.value - from Vec<sqlx::types::Json<T>> or Vec<PrimitiveT>
                case 'fixedCountNode':
                    if (isJson) {
                        // If elements are defined types or plain values, don't try to unwrap .0
                        if (
                            isNode(typeNode.item, 'definedTypeLinkNode') ||
                            isNode(typeNode.item, 'structTypeNode') ||
                            isNode(typeNode.item, 'enumTypeNode')
                        ) {
                            return (
                                `${prefix}.0.into_iter().collect::<Vec<_>>()` +
                                `.try_into().map_err(|_| carbon_core::error::Error::Custom("Failed to convert value from postgres primitive".to_string()))?`
                            );
                        }
                        // JSON-stored vectors of primitives/arrays need element-level reverse then try_into at this level
                        return (
                            `${prefix}.0.into_iter().map(|element| Ok(${buildReverse(typeNode.item, 'element')})).collect::<Result<Vec<_>, _>>()?` +
                            `.try_into().map_err(|_| carbon_core::error::Error::Custom("Failed to convert value from postgres primitive".to_string()))?`
                        );
                    } else {
                        return `${prefix}.into_iter().map(|element| Ok(${buildReverse(typeNode.item, 'element')})).collect::<Result<Vec<_>, _>>()?.try_into().map_err(|_| carbon_core::error::Error::Custom("Failed to convert array element to primitive".to_string()))?`;
                    }
                    break;
                // our target type is Vec<T>, T is typeNode.item - from Vec<sqlx::types::Json<T>> or Vec<PrimitiveT>
                case 'prefixedCountNode':
                    if (isJson) {
                        if (
                            isNode(typeNode.item, 'definedTypeLinkNode') ||
                            isNode(typeNode.item, 'structTypeNode') ||
                            isNode(typeNode.item, 'enumTypeNode')
                        ) {
                            return `${prefix}.0`;
                        }
                        return `${prefix}.0.into_iter().map(|element| ${buildReverse(typeNode.item, 'element')}).collect()`;
                    } else {
                        if (isNode(typeNode.item, 'publicKeyTypeNode')) {
                            return `${prefix}.into_iter().map(|element| *element).collect()`;
                        }
                        return `${prefix}.into_iter().map(|element| element.try_into()).collect::<Result<_, _>>().map_err(|_| carbon_core::error::Error::Custom("Failed to convert array element to primitive".to_string()))?`;
                    }
                    break;
                case 'remainderCountNode':
                    if (isJson) {
                        if (
                            isNode(typeNode.item, 'definedTypeLinkNode') ||
                            isNode(typeNode.item, 'structTypeNode') ||
                            isNode(typeNode.item, 'enumTypeNode')
                        ) {
                            return `${prefix}.0`;
                        }
                        return `${prefix}.0.into_iter().map(|element| ${buildReverse(typeNode.item, 'element')}).collect()`;
                    } else {
                        if (isNode(typeNode.item, 'publicKeyTypeNode')) {
                            return `${prefix}.into_iter().map(|element| *element).collect()`;
                        }
                        return `${prefix}.into_iter().map(|element| element.try_into()).collect::<Result<_, _>>().map_err(|_| carbon_core::error::Error::Custom("Failed to convert array element to primitive".to_string()))?`;
                    }
                    break;
            }
        }
        if (isNode(typeNode, 'optionTypeNode')) {
            const innerReverse = buildReverse(typeNode.item, 'value');
            if (innerReverse.includes('?')) {
                const innerWithoutQuestion = innerReverse.replace(/\?$/, '');
                return `${prefix}.map(|value| ${innerWithoutQuestion}).transpose()?`;
            }
            return `${prefix}.map(|value| ${innerReverse})`;
        }
        if (isNode(typeNode, 'tupleTypeNode')) {
            if (typeNode.items.length === 1) {
                return `${buildReverse(typeNode.items[0], `${prefix}`)}`;
            }
            return `(${typeNode.items.map((it, i) => buildReverse(it, `${prefix}.${i}`)).join(', ')})`;
        }
        if (
            isNode(typeNode, 'definedTypeLinkNode') ||
            isNode(typeNode, 'structTypeNode') ||
            isNode(typeNode, 'enumTypeNode')
        ) {
            return `${prefix}.0`;
        }
        if (isNode(typeNode, 'publicKeyTypeNode')) {
            return `*${prefix}`;
        }

        if (isNode(typeNode, 'numberTypeNode')) {
            switch (typeNode.format) {
                case 'u8':
                case 'u16':
                case 'u32':
                    return `${prefix}.try_into().map_err(|_| carbon_core::error::Error::Custom("Failed to convert value from postgres primitive".to_string()))?`;
                case 'u64':
                case 'u128':
                case 'i128':
                    return `*${prefix}`;
                default:
                    break;
            }
        }

        return `${prefix}.into()`;
    }
}
```

```plaintext
{% extends "layout.njk" %}
{% import "macros.njk" as macros %}

{% block main %}

{{ imports }}

{% if postgresMode !== 'generic' %}
#[cfg(feature = "postgres")]
pub mod postgres;
{% endif %}

#[cfg(feature = "graphql")]
pub mod graphql;

{% for account in accountsToExport | sort(false, false, 'name') %}
pub mod {{ macros.escapeRustKeyword(account.name | snakeCase) }};
{% endfor %}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "data"))]
pub enum {{ program.name | pascalCase }}Account {
{% for account in accountsToExport | sort(false, false, 'name') %}
    {{ account.name | pascalCase }}(Box<{{ account.name | snakeCase }}::{{ account.name | pascalCase }}>),
{% endfor %}
}

impl<'a> carbon_core::account::AccountDecoder<'a> for {{ program.name | pascalCase }}Decoder {
    type AccountType = {{ program.name | pascalCase }}Account;

    fn decode_account(
        &self,
        account: &'a solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if account.owner != PROGRAM_ID {
            return None;
        }

        let data = account.data.as_slice();

        {% for account in accountsToExport %}
        // Try to decode {{ account.name | pascalCase }}
        {
            if let Some(decoded) = {{ macros.escapeRustKeyword(account.name | snakeCase) }}::{{ account.name | pascalCase }}::decode(data) {
                return Some(carbon_core::account::DecodedAccount {
                    lamports: account.lamports,
                    data: {{ program.name | pascalCase }}Account::{{ account.name | pascalCase }}(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }
        {% endfor %}

        None
    }
}

{% endblock %}
```

```plaintext
{% extends "layout.njk" %}
{% import "macros.njk" as macros %}

{% block main %}

{{ imports }}

{% if postgresMode !== 'generic' %}
#[cfg(feature = "postgres")]
pub mod postgres;
{% endif %}

#[cfg(feature = "graphql")]
pub mod graphql;



{% for instruction in instructionsToExport | sort(false, false, 'name') %}
pub mod {{ macros.escapeRustKeyword(instruction.name | snakeCase) }};
{% endfor %}
{% if hasAnchorEvents %}
pub mod cpi_event;
{% endif %}

{% for instruction in instructionsToExport | sort(false, false, 'name') %}
pub use self::{{ macros.escapeRustKeyword(instruction.name | snakeCase) }}::*;
{% endfor %}
{% if hasAnchorEvents %}
pub use self::cpi_event::*;
{% endif %}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", content = "data"))]
pub enum {{ program.name | pascalCase }}Instruction {
{% for instruction in instructionsToExport | sort(false, false, 'name') %}
    {{ instruction.name | pascalCase }}({{ instruction.name | pascalCase }}),
{% endfor %}
{% if hasAnchorEvents %}
// Anchor CPI Event Instruction
    CpiEvent(CpiEvent)
{% endif %}
}

impl carbon_core::instruction::InstructionDecoder<'_> for {{ program.name | pascalCase }}Decoder {
    type InstructionType = {{ program.name | pascalCase }}Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        let data = instruction.data.as_slice();

        {% for instruction in instructionsToExport %}
        // Try to decode {{ instruction.name | pascalCase }}
        {
            if let Some(decoded) = {{ macros.escapeRustKeyword(instruction.name | snakeCase) }}::{{ instruction.name | pascalCase }}::decode(data) {
                return Some(carbon_core::instruction::DecodedInstruction {
                    program_id: instruction.program_id,
                    data: {{ program.name | pascalCase }}Instruction::{{ instruction.name | pascalCase }}(decoded),
                    accounts: instruction.accounts.clone(),
                });
            }
        }
        {% endfor %}
        {% if hasAnchorEvents %}
        {
            if let Some(decoded) = cpi_event::CpiEvent::decode(data) {
                return Some(carbon_core::instruction::DecodedInstruction {
                    program_id: instruction.program_id,
                    data: {{ program.name | pascalCase }}Instruction::CpiEvent(decoded),
                    accounts: instruction.accounts.clone(),
                });
            }
        }
        {% endif %}

        None
    }
}

{% endblock %}
```

```plaintext
{{ imports }}

{% if entityDocs and entityDocs.length > 0 %}
{% for doc in entityDocs %}
{% set lines = doc.split('\n') %}
{% for line in lines %}
/// {{ line }}
{% endfor %}
{% endfor %}
{% endif %}
#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "{{ entityName | pascalCase }}")]
pub struct {{ entityName | pascalCase }}GraphQL {
    pub {% if isAccount %}account_metadata{% else %}instruction_metadata{% endif %}: {% if isAccount %}crate::accounts::graphql::AccountMetadataGraphQL{% else %}crate::instructions::graphql::InstructionMetadataGraphQL{% endif %},
{% for field in graphqlFields %}
    {% if field.docs and field.docs.length > 0 %}
    {% for doc in field.docs %}
    /// {{ doc }}
    {% endfor %}
    {% endif %}
    pub {{ field.fieldName }}: {{ field.rustType }},
{% endfor %}
}

impl TryFrom<crate::{% if isAccount %}accounts{% else %}instructions{% endif %}::postgres::{{ entityName | pascalCase }}Row> for {{ entityName | pascalCase }}GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(row: crate::{% if isAccount %}accounts{% else %}instructions{% endif %}::postgres::{{ entityName | pascalCase }}Row) -> Result<Self, Self::Error> {
        Ok(Self {
            {% if isAccount %}account_metadata{% else %}instruction_metadata{% endif %}: row.{% if isAccount %}account_metadata{% else %}instruction_metadata{% endif %}.into(),
{% for field in graphqlFields %}
            {{ field.fieldName }}: {{ field.fromRowExpr }},
{% endfor %}
        })
    }
}
```

```plaintext
{{ imports }}

{% if entityDocs and entityDocs.length > 0 %}
{% for doc in entityDocs %}
{% set lines = doc.split('\n') %}
{% for line in lines %}
/// {{ line }}
{% endfor %}
{% endfor %}
{% endif %}
#[derive(Debug, Clone, GraphQLObject)]
#[graphql(name = "{{ entityName | pascalCase }}")]
pub struct {{ entityName | pascalCase }}GraphQL {
    pub {% if isAccount %}account_metadata{% else %}instruction_metadata{% endif %}: {% if isAccount %}crate::accounts::graphql::AccountMetadataGraphQL{% else %}crate::instructions::graphql::InstructionMetadataGraphQL{% endif %},
{% for field in graphqlFields %}
    {% if field.docs and field.docs.length > 0 %}
    {% for doc in field.docs %}
    /// {{ doc }}
    {% endfor %}
    {% endif %}
    pub {{ field.fieldName }}: {{ field.rustType }},
{% endfor %}
}

impl TryFrom<{% if isAccount %}crate::accounts::{{ entityName | snakeCase }}::{{ entityName | pascalCase }}{% else %}crate::instructions::{{ entityName | snakeCase }}::{{ entityName | pascalCase }}{% endif %}> for {{ entityName | pascalCase }}GraphQL {
    type Error = carbon_core::error::Error;
    fn try_from(original: {% if isAccount %}crate::accounts::{{ entityName | snakeCase }}::{{ entityName | pascalCase }}{% else %}crate::instructions::{{ entityName | snakeCase }}::{{ entityName | pascalCase }}{% endif %}) -> Result<Self, Self::Error> {
        Ok(Self {
            {% if isAccount %}account_metadata{% else %}instruction_metadata{% endif %}: {% if isAccount %}crate::accounts::graphql::AccountMetadataGraphQL::default(){% else %}crate::instructions::graphql::InstructionMetadataGraphQL::default(){% endif %},
{% for field in graphqlFields %}
            {{ field.fieldName }}: {{ field.fromOriginalExpr }},
{% endfor %}
        })
    }
}

```

```typescript
import { mkdirSync, writeFileSync, existsSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';
import nunjucks from 'nunjucks';
import { exitWithError } from './utils';
import { kebabCase } from 'codama';
import * as Datasources from '../datasources';
import type { DecoderMeta } from '../datasources';

export type ScaffoldOptions = {
    name: string;
    outDir: string;
    decoder: string;
    decoderMode?: 'published' | 'generate';
    decoderPath?: string; // Path to generated decoder
    dataSource: string;
    metrics: 'log' | 'prometheus';
    withPostgres: boolean;
    withGraphql: boolean;
    withSerde: boolean;
    force?: boolean;
    postgresMode?: 'generic' | 'typed';
};

function ensureDir(path: string) {
    if (!existsSync(path)) {
        mkdirSync(path, { recursive: true });
    }
}

function buildProjectImports(ctx: any): string {
    const lines: string[] = [];

    // Common
    lines.push('use std::{env, sync::Arc};');

    // Feature-dependent
    if (!ctx.withPostgres) {
        lines.push('use async_trait::async_trait;');
        lines.push('use carbon_core::deserialize::ArrangeAccounts;');
        lines.push('use carbon_core::instruction::{DecodedInstruction, InstructionMetadata, NestedInstructions};');
        lines.push('use carbon_core::metrics::MetricsCollection;');
        lines.push('use carbon_core::processor::Processor;');
    }

    lines.push('use carbon_core::error::CarbonResult;');

    if (ctx.withPostgres) {
        if (ctx.useGenericPostgres) {
            lines.push(
                'use carbon_core::postgres::processors::{PostgresJsonAccountProcessor, PostgresJsonInstructionProcessor};',
            );
            lines.push('use carbon_core::postgres::rows::{GenericAccountsMigration, GenericInstructionMigration};');
        } else {
            lines.push(
                'use carbon_core::postgres::processors::{PostgresAccountProcessor, PostgresInstructionProcessor};',
            );
        }
        lines.push('use sqlx_migrator::{Info, Migrate, Plan};');
    }

    // Metrics
    lines.push(`use carbon_${ctx.metrics.module_name}_metrics::${ctx.metrics.name}Metrics;`);

    // Decoders
    for (const d of ctx.decoders as Array<{ name: string; module_name: string }>) {
        const crate = `carbon_${d.module_name}_decoder`;
        if (ctx.withPostgres) {
            if (!ctx.useGenericPostgres) {
                lines.push(
                    `use ${crate}::accounts::postgres::{${d.name}AccountWithMetadata, ${d.name}AccountsMigration};`,
                );
                lines.push(`use ${crate}::accounts::${d.name}Account;`);
                lines.push(
                    `use ${crate}::instructions::postgres::{${d.name}InstructionWithMetadata, ${d.name}InstructionsMigration};`,
                );
                lines.push(`use ${crate}::instructions::${d.name}Instruction;`);
            } else {
                lines.push(`use ${crate}::accounts::${d.name}Account;`);
                lines.push(`use ${crate}::instructions::${d.name}Instruction;`);
            }
        } else {
            lines.push(`use ${crate}::instructions::${d.name}Instruction;`);
        }
        if (ctx.withGraphQL) {
            lines.push(`use ${crate}::graphql::{QueryRoot, context::GraphQLContext};`);
        }
        lines.push(`use ${crate}::${d.name}Decoder;`);
        lines.push(`use ${crate}::PROGRAM_ID as ${d.name.toUpperCase()}_PROGRAM_ID;`);
    }

    // Datasource-specific imports
    const ds = ctx.data_source.module_name as string;
    if (ds === 'rpc_block_subscribe') {
        lines.push('use carbon_rpc_block_subscribe_datasource::{Filters, RpcBlockSubscribe};');
        lines.push('use solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter};');
    }
    if (ds === 'yellowstone_grpc' || ds === 'helius_laserstream') {
        lines.push('use std::collections::{HashMap, HashSet};');
        if (ds === 'yellowstone_grpc') {
            lines.push(
                'use carbon_yellowstone_grpc_datasource::{YellowstoneGrpcGeyserClient, YellowstoneGrpcClientConfig};',
            );
        }
        lines.push('use tokio::sync::RwLock;');
        lines.push(
            'use yellowstone_grpc_proto::geyser::{CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions};',
        );
    }

    if (ctx.withGraphQL) {
        lines.push('use std::net::SocketAddr;');
    }

    // Include datasource-specific imports from TS builders (if any)
    if (ctx.datasource_imports) {
        lines.push(ctx.datasource_imports);
    }

    return lines.join('\n');
}

function buildIndexerCargoContext(opts: ScaffoldOptions) {
    const featureParts: string[] = [];

    if (opts.withPostgres) featureParts.push('"postgres"');
    if (opts.withGraphql) featureParts.push('"graphql"');
    if (opts.withSerde) featureParts.push('"serde"');

    const hasLocalDecoder = true;
    const decoderCrateName = kebabCase(opts.decoder);

    let decoderDependency: string = '';
    let decoderFeatures = '';
    if (featureParts.length) {
        decoderFeatures = `, features = [${featureParts.join(', ')}]`;
    }

    const dsModule = opts.dataSource.toLowerCase();
    const dsPathDir =
        dsModule.replace(/-/g, '_') === 'helius_laserstream'
            ? 'helius-laserstream-datasource'
            : dsModule === 'yellowstone-grpc'
              ? 'yellowstone-grpc-datasource'
              : dsModule === 'rpc-block-subscribe'
                ? 'rpc-block-subscribe-datasource'
                : dsModule === 'rpc-program-subscribe'
                  ? 'rpc-program-subscribe-datasource'
                  : dsModule === 'rpc-transaction-crawler'
                    ? 'rpc-transaction-crawler-datasource'
                    : dsModule === 'helius-atlas-ws'
                      ? 'helius-atlas-ws-datasource'
                      : `${dsModule}-datasource`;
    const datasourceDep = `carbon-${opts.dataSource.toLowerCase()}-datasource = { path = "../../../datasources/${dsPathDir}" }`;
    const metricsPathDir = opts.metrics.toLowerCase() === 'prometheus' ? 'prometheus-metrics' : 'log-metrics';
    const metricsDep = `carbon-${opts.metrics.toLowerCase()}-metrics = { path = "../../../metrics/${metricsPathDir}" }`;

    const grpcDeps =
        opts.dataSource === 'yellowstone-grpc' || opts.dataSource === 'helius-laserstream'
            ? `yellowstone-grpc-client = { version = "9.0.0" }\nyellowstone-grpc-proto = { version = "9.0.0" }`
            : '';

    const pgDeps = opts.withPostgres
        ? `sqlx = { version = "0.8.6", features = ["postgres", "runtime-tokio-rustls", "macros"] }\nsqlx_migrator = "0.17.0"`
        : '';

    const gqlDeps = opts.withGraphql ? `juniper = "0.15"\naxum = "0.8.4"` : '';

    const rustlsDep =
        opts.dataSource === 'yellowstone-grpc' || opts.dataSource === 'helius-laserstream' ? 'rustls = "0.23"' : '';
    const atlasDeps = opts.dataSource === 'helius-atlas-ws' ? 'helius = "0.3.2"' : '';

    const features = ['default = []', opts.withPostgres ? 'postgres = []' : '', opts.withGraphql ? 'graphql = []' : '']
        .filter(Boolean)
        .join('\n');

    const crawlerDeps = opts.dataSource === 'rpc-transaction-crawler' ? 'solana-commitment-config = "^2.2.1"' : '';
    const programDeps = opts.dataSource === 'rpc-program-subscribe' ? 'solana-account-decoder = "^2.3.6"' : '';

    return {
        projectName: opts.name,
        hasLocalDecoder,
        decoderCrateName,
        decoderFeatures,
        decoderDependency,
        datasourceDep,
        metricsDep,
        grpcDeps,
        pgDeps,
        gqlDeps,
        rustlsDep,
        crawlerDeps,
        programDeps,
        atlasDeps,
        features,
    };
}

function getEnvContent(dataSource: string, withPostgres: boolean): string {
    const dataSourceLower = dataSource.toLowerCase().replace(/-/g, '_');

    let envContent = '';

    // Add database URL if postgres is enabled
    if (withPostgres) {
        envContent = 'DATABASE_URL=postgres://user:password@localhost/dbname\n';
    }

    // Add datasource-specific env vars
    switch (dataSourceLower) {
        case 'helius_atlas_ws':
            envContent += 'HELIUS_API_KEY=your-atlas-ws-url-here';
            break;
        case 'helius_laserstream':
            envContent += 'GEYSER_URL=your-grpc-url-here\nX_TOKEN=your-x-token-here';
            break;
        case 'rpc_block_subscribe':
            envContent += 'RPC_WS_URL=your-rpc-ws-url-here';
            break;
        case 'rpc_transaction_crawler':
            envContent += 'RPC_URL=your-rpc-url-here';
            break;
        case 'yellowstone_grpc':
            envContent += 'GEYSER_URL=your-rpc-url-here\nX_TOKEN=your-x-token-here';
            break;
    }

    return envContent;
}

export function renderScaffold(opts: ScaffoldOptions) {
    const base = join(opts.outDir, opts.name);

    if (existsSync(base) && !opts.force) {
        exitWithError(`Output directory already exists: ${base} (use --force to overwrite)`);
    }

    ensureDir(base);

    // Create workspace structure
    const indexerDir = join(base, 'indexer');
    ensureDir(indexerDir);
    ensureDir(join(indexerDir, 'src'));

    const thisDir = dirname(fileURLToPath(import.meta.url));
    const templatesDir = join(thisDir, '..', 'templates');

    if (!existsSync(join(templatesDir, 'project.njk'))) {
        exitWithError('Template file not found. Please ensure cli/templates/project.njk exists.');
    }

    const env = nunjucks.configure(templatesDir, {
        autoescape: false,
        noCache: false,
    });

    const hasLocalDecoder = opts.decoderMode === 'generate';

    // Context base for main.rs
    const mainContext: any = {
        projectName: opts.name,
        decoders: [
            {
                name: opts.decoder
                    .split('-')
                    .map((w: string) => w.charAt(0).toUpperCase() + w.slice(1))
                    .join(''),
                module_name: opts.decoder.replace(/-/g, '_'),
            },
        ],
        data_source: {
            module_name: opts.dataSource.replace(/-/g, '_'),
        },
        metrics: {
            name: opts.metrics === 'prometheus' ? 'Prometheus' : 'Log',
            module_name: opts.metrics,
        },
        withPostgres: opts.withPostgres,
        withGraphQL: opts.withGraphql,
        useGenericPostgres: opts.postgresMode === 'generic',
    };

    // Build datasource artifacts from TS module
    const dsModuleName = mainContext.data_source.module_name as string;
    const builder = Datasources.getDatasourceBuilder(dsModuleName);
    if (builder) {
        const decodersMeta = mainContext.decoders as DecoderMeta[];
        const artifact = builder(decodersMeta);
        // Compose import lines
        const datasource_imports = artifact.imports.map((i: string) => `use ${i};`).join('\n');
        mainContext.datasource_imports = datasource_imports;
        mainContext.datasource_init = artifact.init;
    } else {
        // Provide a clearer error message if no builder is found
        const available = Object.keys(
            (Datasources as unknown as { getDatasourceBuilder: any }).getDatasourceBuilder
                ? {
                      helius_laserstream: true,
                      rpc_block_subscribe: true,
                      yellowstone_grpc: true,
                      helius_atlas_ws: true,
                      rpc_transaction_crawler: true,
                      rpc_program_subscribe: true,
                  }
                : {},
        );
        exitWithError(`No datasource builder found for '${dsModuleName}'. Available: ${available.join(', ')}`);
    }

    // Generate workspace Cargo.toml
    const workspaceContext = {
        hasLocalDecoder,
    };
    const workspaceToml = env.render('workspace.njk', workspaceContext);
    writeFileSync(join(base, 'Cargo.toml'), workspaceToml);

    // Compute dynamic imports for main.rs
    mainContext.imports = buildProjectImports(mainContext);

    // Generate indexer main.rs
    const rendered = env.render('project.njk', mainContext);
    writeFileSync(join(indexerDir, 'src', 'main.rs'), rendered);

    // Generate indexer Cargo.toml
    const indexerCargoContext = buildIndexerCargoContext(opts);
    const indexerCargoToml = env.render('indexer-cargo.njk', indexerCargoContext);
    writeFileSync(join(indexerDir, 'Cargo.toml'), indexerCargoToml);

    // Generate .gitignore at workspace root
    const gitignore = `debug/
target/

.env
.DS_Store
`;
    writeFileSync(join(base, '.gitignore'), gitignore);

    // Generate .env at workspace root
    const envContent = getEnvContent(opts.dataSource, opts.withPostgres);
    if (envContent) {
        writeFileSync(join(base, '.env'), envContent);
    }

    // Generate README.md at workspace root
    const readme = `# ${opts.name}

Generated by carbon-cli scaffold.

## Structure

This is a Cargo workspace containing:
- \`indexer/\` - The main indexer application${hasLocalDecoder ? '\n- `decoder/` - Generated decoder from IDL' : ''}

## Run

\`\`\`bash
cargo run -p ${opts.name}-indexer
\`\`\`

## Features
- Data source: ${opts.dataSource}
- Metrics: ${opts.metrics}
- Postgres: ${opts.withPostgres}
- GraphQL: ${opts.withGraphql}
- Decoder: ${hasLocalDecoder ? 'Generated locally' : `Published (carbon-${opts.decoder}-decoder)`}
`;

    writeFileSync(join(base, 'README.md'), readme);
}
```

```plaintext
<<'PATCH'
*** Begin Patch
*** Update File: packages/cli/templates/project.njk
@@
-use {
-    {%- if not withPostgres %}
-    async_trait::async_trait,
-    {%- endif %}
-    carbon_core::{
-        {%- if not withPostgres %}
-        deserialize::ArrangeAccounts,
-        {%- endif %}
-        error::CarbonResult,
-        {%- if not withPostgres %}
-        instruction::{DecodedInstruction, InstructionMetadata, NestedInstructions},
-        metrics::MetricsCollection,
-        processor::Processor,
-        {%- endif %}
-        {%- if withPostgres %}
-        {%- if useGenericPostgres %}
-        postgres::processors::{PostgresJsonAccountProcessor, PostgresJsonInstructionProcessor},
-        postgres::rows::{GenericAccountsMigration, GenericInstructionMigration},
-        {%- else %}
-        postgres::processors::{PostgresAccountProcessor, PostgresInstructionProcessor},
-        {%- endif %}
-        {%- endif %}
-    },
-    carbon_{{ metrics.module_name }}_metrics::{{ metrics.name }}Metrics,
-    {%- for decoder in decoders %}
-    carbon_{{ decoder.module_name }}_decoder::{
-        {%- if withPostgres %}
-        {%- if not useGenericPostgres %}
-        accounts::{
-            postgres::{
-                {{ decoder.name }}AccountWithMetadata,
-                {{ decoder.name }}AccountsMigration
-            },
-            {{ decoder.name }}Account,
-        },
-        instructions::{
-            postgres::{
-                {{ decoder.name }}InstructionWithMetadata,
-                {{ decoder.name }}InstructionsMigration
-            },
-            {{ decoder.name }}Instruction,
-        },
-        {%- else %}
-        accounts::{{ decoder.name }}Account,
-        instructions::{{ decoder.name }}Instruction,
-        {%- endif %}
-        {%- else %}
-        instructions::{{ decoder.name }}Instruction,
-        {%- endif %}
-        {%- if withGraphQL %}
-        graphql::{QueryRoot, context::GraphQLContext},
-        {%- endif %}
-        {{ decoder.name }}Decoder,
-        PROGRAM_ID as {{ decoder.name | upper }}_PROGRAM_ID,
-    },
-    {%- endfor %}
-    {%- if data_source.module_name == "rpc_block_subscribe" %}
-    carbon_rpc_block_subscribe_datasource::{Filters, RpcBlockSubscribe},
-    solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
-    {%- endif %}
-    std::{env, sync::Arc},
-    {%- if data_source.module_name == "yellowstone_grpc" or data_source.module_name == "helius_laserstream" %}
-    std::collections::{HashMap, HashSet},
-    {%- if data_source.module_name == "yellowstone_grpc" %}
-    carbon_yellowstone_grpc_datasource::{YellowstoneGrpcGeyserClient, YellowstoneGrpcClientConfig},
-    {%- endif %}
-    {%- if data_source.module_name == "helius_laserstream" %}
-    {%- endif %}
-    tokio::sync::RwLock,
-    yellowstone_grpc_proto::geyser::{
-        CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
-    },
-    {%- endif %}
-    {%- if withPostgres %}
-    sqlx_migrator::{Info, Migrate, Plan},
-    {%- endif %}
-    {%- if withGraphQL %}
-    std::net::SocketAddr,
-    {%- endif %}
-};
+{{ imports }}
*** End Patch
PATCH
```

```plaintext
{{ imports }}

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    {%- if data_source.module_name == "yellowstone_grpc" or data_source.module_name == "helius_laserstream" %}
    // NOTE: Workaround, that solving issue https://github.com/rustls/rustls/issues/1877
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Can't set crypto provider to aws_lc_rs");
    {%- endif %}

    {%- if withPostgres %}
    // Database setup and migrations
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::Pool::<sqlx::Postgres>::connect(&db_url)
        .await
        .unwrap();

    let mut migrator = sqlx_migrator::Migrator::default();
    let mut conn = pool.acquire().await.unwrap();

    // Migrations
    {%- if useGenericPostgres %}
    migrator
        .add_migration(Box::new(GenericAccountsMigration))
        .unwrap();
    migrator
        .add_migration(Box::new(GenericInstructionMigration))
        .unwrap();
    {%- else %}
    {%- for decoder in decoders %}
    migrator
        .add_migration(Box::new({{ decoder.name }}AccountsMigration))
        .unwrap();
    migrator
        .add_migration(Box::new({{ decoder.name }}InstructionsMigration))
        .unwrap();
    {%- endfor %}
    {%- endif %}

    migrator.run(&mut *conn, &Plan::apply_all()).await.unwrap();
    {%- endif %}

    let datasource = {{ datasource_init | safe }};

    let mut pipeline = carbon_core::pipeline::Pipeline::builder()
        .datasource(datasource)
        .metrics(Arc::new({{ metrics.name }}Metrics::new()))
        .metrics_flush_interval(5)
        {%- if withPostgres %}
        {%- if useGenericPostgres %}
        {%- for decoder in decoders %}
        .account(
            {{ decoder.name }}Decoder,
            PostgresJsonAccountProcessor::<{{ decoder.name }}Account>::new(pool.clone()),
        )
        .instruction(
            {{ decoder.name }}Decoder,
            PostgresJsonInstructionProcessor::<{{ decoder.name }}Instruction>::new(pool.clone()),
        )
        {%- endfor %}
        {%- else %}
        {%- for decoder in decoders %}
        .account(
            {{ decoder.name }}Decoder,
            PostgresAccountProcessor::<{{ decoder.name }}Account, {{ decoder.name }}AccountWithMetadata>::new(pool.clone()),
        )
        .instruction(
            {{ decoder.name }}Decoder,
            PostgresInstructionProcessor::<{{ decoder.name }}Instruction, {{ decoder.name }}InstructionWithMetadata>::new(pool.clone()),
        )
        {%- endfor %}
        {%- endif %}
        {%- else %}
        {%- for decoder in decoders %}
        .instruction({{ decoder.name }}Decoder, {{ decoder.name }}InstructionProcessor)
        {%- endfor %}
        {%- endif %}
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        .build()?;

    {%- if withGraphQL %}
    tokio::select! {
        res = run_graphql(Arc::new(pool.clone())) => {
            res?;
        }
        res = pipeline.run() => {
            res?;
        }
    }
    {%- else %}
    pipeline.run().await?;
    {%- endif %}

    Ok(())
}

{%- if withGraphQL %}
async fn run_graphql(pool: Arc<sqlx::PgPool>) -> CarbonResult<()> {
    {%- for decoder in decoders %}
    let schema = carbon_core::graphql::server::build_schema(QueryRoot);
    let ctx = GraphQLContext { pool: pool.clone() };
    let app = carbon_core::graphql::server::graphql_router::<QueryRoot, GraphQLContext>(schema, ctx);
    {%- endfor %}

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    println!("GraphQL: http://{addr}/graphql");
    println!("GraphiQL: http://{addr}/graphiql");
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
    Ok(())
}
{%- endif %}

{%- if not withPostgres %}
{%- for decoder in decoders %}
pub struct {{ decoder.name }}InstructionProcessor;

#[async_trait]
impl Processor for {{ decoder.name }}InstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<{{ decoder.name }}Instruction>,
        NestedInstructions,
        solana_instruction::Instruction,
    );

    async fn process(
        &mut self,
        (metadata, instruction, _nested_instructions, _raw_instruction): Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        let signature = metadata.transaction_metadata.signature;
        let accounts = instruction.accounts;

        log::info!("received the {{ decoder.name }} instruction, sig: {}, accounts len: {}", signature, accounts.len());

        match instruction.data {
            _ => {}
        };

        Ok(())
    }
}
{%- endfor %}
{%- endif %}
```

- I updated the renderer to generate imports via ImportMap for mod and GraphQL templates, then adjusted the templates to consume those imports. Next, I added a dynamic import builder to the CLI scaffold and simplified the project template to use it.

Edits applied:

- Renderer code
    - packages/renderer/src/getRenderMapVisitor.ts
        - Added GraphQLObject to ImportMap for GraphQL schema generation.
        - Injected ImportMap-generated imports into `accountsMod.njk` and `instructionsMod.njk`.
- Renderer templates
    - packages/renderer/templates/accountsMod.njk
        - Replaced hardcoded crate import with `{{ imports }}`.
    - packages/renderer/templates/instructionsMod.njk
        - Replaced hardcoded crate import with `{{ imports }}`.
    - packages/renderer/templates/graphqlSchemaPage.njk
        - Removed hardcoded `use juniper::GraphQLObject;`, relying on `{{ imports }}`.
    - packages/renderer/templates/graphqlSchemaPageGeneric.njk
        - Removed hardcoded `use juniper::GraphQLObject;`, relying on `{{ imports }}`.
- CLI scaffold
    - packages/cli/src/lib/scaffold.ts
        - Added `buildProjectImports(ctx)` to compute all import lines dynamically (Postgres typed/generic, GraphQL, metrics, decoders, datasource-specific).
        - Combined `datasource_imports` into the computed imports.
        - Injected `imports` into `mainContext` for template rendering.
- CLI template
    - packages/cli/templates/project.njk
        - Replaced the large conditional `use { ... }` block and separate `datasource_imports` section with a single `{{ imports }}`.

CLARITY INTERROGATION

- Any other templates you want to switch to ImportMap now (e.g., graphqlQueryPage\*.njk), or keep them as-is for this phase?

SELF-CHECK & INTERNAL ITERATION

- Alternatives: leaving mod and GraphQL derive imports hardcoded; current approach centralizes for consistency.
- Risks: Import mismatches for rare datasource combinations; mitigated by mirroring existing conditions and appending datasource builder imports.

FINAL STATUS
This is ready. No further iteration required unless your requirements change.
