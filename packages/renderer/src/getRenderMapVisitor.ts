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
import { VERSIONS } from '@sevenlabs-hq/carbon-versions';

export type GetRenderMapOptions = {
    renderParentInstructions?: boolean;
    packageName?: string;
    anchorEvents?: {
        name: string,
        discriminator: number[];
    }[];
    postgresMode?: 'generic' | 'typed';
    withPostgres?: boolean;
    withGraphql?: boolean;
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

                    const discriminatorManifest = discriminators.length > 0
                        ? getDiscriminatorManifest(discriminators)
                        : undefined;

                    // Postgres generation
                    const flatFields = flattenType(newNode.data, [], [], new Set());
                    const postgresImports = new ImportMap()
                        .add(`crate::accounts::${snakeCase(node.name)}::${pascalCase(node.name)}`)
                        .add('carbon_core::account::AccountMetadata')
                        .add('carbon_core::postgres::metadata::AccountRowMetadata');
                    flatFields.forEach(f => {
                        postgresImports.mergeWith(f.postgresManifest.imports);
                    });

                    let renderMap = new RenderMap()
                        .add(
                            `src/accounts/${snakeCase(node.name)}.rs`,
                            render('accountsPage.njk', {
                                account: newNode,
                                imports: imports.toString(),
                                program: currentProgram,
                                discriminatorManifest,
                                typeManifest,
                            }),
                        );

                    // Only generate postgres files if not in generic mode and withPostgres is enabled
                    if (options.postgresMode !== 'generic' && options.withPostgres !== false) {
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

                    // GraphQL generation - only if withGraphql is enabled
                    if (options.withGraphql !== false) {
                        const graphqlFields = flattenTypeForGraphQL(newNode.data, [], [], new Set());
                        const graphqlImports = new ImportMap();
                        graphqlFields.forEach((f: FlattenedGraphQLField) => {
                            graphqlImports.mergeWith(f.graphqlManifest.imports);
                        });
                        // Ensure GraphQL derive is imported consistently via ImportMap
                        graphqlImports.add('juniper::GraphQLObject');

                        // GraphQLObject doesn't support empty structs
                        if (graphqlFields.length > 0) {
                            const schemaTemplate = options.postgresMode === 'generic' ? 'graphqlSchemaPageGeneric.njk' : 'graphqlSchemaPage.njk';
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
                                bytes: `[${event.discriminator.join(", ")}]`,
                                size: event.discriminator.length,
                                checkCode: `        if data.len() < ${event.discriminator.length} {
            return None;
        }
        let discriminator = &data[0..${event.discriminator.length}];
        if discriminator != &[${event.discriminator.join(", ")}] {
            return None;
        }`,
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

                    // GraphQL generation for structs and enums - only if withGraphql is enabled
                    if (options.withGraphql !== false) {
                        if (node.type.kind === 'structTypeNode') {
                            if (node.type.fields.length > 0) {
                                const graphqlFields = flattenTypeForGraphQL(node.type, [], [], new Set());
                                const graphqlImports = new ImportMap()
                                    .add('juniper::GraphQLObject');
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
                                const emptyStructImports = new ImportMap()
                                    .add('carbon_core::graphql::primitives::Json');
                                renderMap.add(
                                    `src/types/graphql/${snakeCase(node.name)}_schema.rs`,
                                    render('graphqlEmptyStructSchemaPage.njk', {
                                        entityDocs: node.docs,
                                        entityName: node.name,
                                        imports: emptyStructImports.toString(),
                                    }),
                                );
                            }
                        } else if (node.type.kind === 'enumTypeNode') {
                            const isFieldless = node.type.variants.every(v => v.kind === 'enumEmptyVariantTypeNode');
                            const imports = new ImportMap();
                            if (isFieldless) {
                                imports.add('juniper::GraphQLEnum');
                            } else {
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
                    }

                    return renderMap;
                },

                visitInstruction(node) {
                    const imports = new ImportMap()
                        .add('carbon_core::borsh::{self, BorshDeserialize}')
                        .add('carbon_core::deserialize::ArrangeAccounts');

                    if (node.accounts && node.accounts.length > 0) {
                        imports.add('carbon_core::account_utils::next_account');
                    }

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

                    let renderMap = new RenderMap()
                        .add(
                            `src/instructions/${snakeCase(node.name)}.rs`,
                            render('instructionsPage.njk', {
                                argumentTypes,
                                imports: imports.toString(),
                                instruction: instructionWithUniqueAccounts,
                                discriminatorManifest,
                                program: currentProgram,
                            }),
                        );

                    // Only generate postgres files if not in generic mode and withPostgres is enabled
                    if (options.postgresMode !== 'generic' && options.withPostgres !== false) {
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

                    // GraphQL generation - only if withGraphql is enabled
                    if (options.withGraphql !== false) {
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
                            const schemaTemplate = options.postgresMode === 'generic' ? 'graphqlSchemaPageGeneric.njk' : 'graphqlSchemaPage.njk';
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
                        withGraphQL: options.withGraphql !== false,
                        versions: VERSIONS,
                    };

                    const map = new RenderMap();

                    // Generate mod files
                        // Build mod-level imports via ImportMap
                        const accountsModImports = new ImportMap()
                            .add('crate::PROGRAM_ID')
                            .add(`crate::${pascalCase(program.name)}Decoder`);
                        map.add('src/accounts/mod.rs', render('accountsMod.njk', { ...ctx, imports: accountsModImports.toString() }));
                        if (options.postgresMode !== 'generic' && options.withPostgres !== false) {
                            map.add('src/accounts/postgres/mod.rs', render('accountsPostgresMod.njk', ctx));
                        }
                        if (options.withGraphql !== false) {
                            const accountsGraphqlTemplate = options.postgresMode === 'generic' ? 'accountsGraphqlModGeneric.njk' : 'accountsGraphqlMod.njk';
                            const accountsGraphqlImports = new ImportMap()
                                .add('juniper::GraphQLObject');
                            map.add('src/accounts/graphql/mod.rs', render(accountsGraphqlTemplate, { ...ctx, imports: accountsGraphqlImports.toString() }));
                        }
                    if (instructionsToExport.length > 0) {
                        const instructionsModImports = new ImportMap()
                            .add('crate::PROGRAM_ID')
                            .add(`crate::${pascalCase(program.name)}Decoder`);
                        map.add('src/instructions/mod.rs', render('instructionsMod.njk', { ...ctx, imports: instructionsModImports.toString() }));
                        if (options.postgresMode !== 'generic' && options.withPostgres !== false) {
                            map.add('src/instructions/postgres/mod.rs', render('instructionsPostgresMod.njk', ctx));
                        }
                        if (options.withGraphql !== false) {
                            const instructionsGraphqlTemplate = options.postgresMode === 'generic' ? 'instructionsGraphqlModGeneric.njk' : 'instructionsGraphqlMod.njk';
                            const instructionsGraphqlImports = new ImportMap()
                                .add('juniper::GraphQLObject');
                            map.add('src/instructions/graphql/mod.rs', render(instructionsGraphqlTemplate, { ...ctx, imports: instructionsGraphqlImports.toString() }));
                        }
                    }
                    
                    if (options.anchorEvents?.length ?? 0 > 0) {
                        const eventInstructionImports = new ImportMap()
                            .add('carbon_core::borsh')
                            .add('carbon_core::deserialize::ArrangeAccounts');
                        map.add('src/instructions/cpi_event.rs', render('eventInstructionPage.njk', { ...ctx, imports: eventInstructionImports.toString() }));
                        if (options.postgresMode !== 'generic' && options.withPostgres !== false) {
                            const eventInstructionRowImports = new ImportMap()
                                .add('carbon_core::postgres::metadata::InstructionRowMetadata')
                                .add('carbon_core::instruction::InstructionMetadata')
                                .add('super::super::cpi_event::CpiEvent');
                            map.add('src/instructions/postgres/cpi_event_row.rs', render('eventInstructionRowPage.njk', { ...ctx, imports: eventInstructionRowImports.toString() }));
                        }
                        if (options.withGraphql !== false) {
                            const cpiEventSchemaTemplate = options.postgresMode === 'generic' ? 'eventInstructionGraphqlSchemaPageGeneric.njk' : 'eventInstructionGraphqlSchemaPage.njk';
                            const cpiEventSchemaImports = new ImportMap()
                                .add('juniper::GraphQLObject');
                            map.add('src/instructions/graphql/cpi_event_schema.rs', render(cpiEventSchemaTemplate, { ...ctx, imports: cpiEventSchemaImports.toString() }));
                        }
                        map.add('src/events/mod.rs', render('eventsMod.njk', ctx));
                    }

                    if (definedTypesToExport.length > 0) {
                        map.add('src/types/mod.rs', render('typesMod.njk', ctx));
                        if (options.withGraphql !== false) {
                            map.add('src/types/graphql/mod.rs', render('typesGraphqlMod.njk', ctx));
                        }
                    }

                    // GraphQL root (context + query) - only if withGraphql is enabled
                    if (options.withGraphql !== false) {
                        map.add('src/graphql/mod.rs', render('graphqlRootMod.njk', ctx));
                        map.add('src/graphql/context.rs', render('graphqlContextPage.njk', ctx));
                        
                        // Use different query template based on postgres mode
                        if (options.postgresMode === 'generic') {
                            const graphqlQueryGenericImports = new ImportMap()
                                .add('juniper::{graphql_object, FieldResult}')
                                .add('carbon_core::postgres::rows::{AccountRow, InstructionRow}');
                            map.add('src/graphql/query.rs', render('graphqlQueryPageGeneric.njk', { ...ctx, imports: graphqlQueryGenericImports.toString() }));
                        } else {
                            const graphqlQueryImports = new ImportMap()
                                .add('juniper::{graphql_object, FieldResult}')
                                .add('std::str::FromStr');
                            map.add('src/graphql/query.rs', render('graphqlQueryPage.njk', { ...ctx, imports: graphqlQueryImports.toString() }));
                        }
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
                ? `${`source.${column}`}.map(|value| value.0)`  // Always single unwrap for JSONB types
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
                ? `${`source.${column}`}.map(|value| value.0)`  // Always single unwrap for JSONB types
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
        } else if (isNode(typeNode, 'optionTypeNode') || isNode(typeNode, 'zeroableOptionTypeNode') || isNode(typeNode, 'remainderOptionTypeNode')) {
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
        if (!isNode(typeNode, 'optionTypeNode') && !isNode(typeNode, 'zeroableOptionTypeNode') && !isNode(typeNode, 'remainderOptionTypeNode')) {
            throw new Error('buildReverseOptionType should only be called for option-like types');
        }
        
        const innerType = typeNode.item;
        
        if (isNode(innerType, 'booleanTypeNode')) {
            return `${prefix}.map(|value| value)`;
        } else if (isNode(innerType, 'numberTypeNode')) {
            const isPostgresPrimitive = manifest.sqlxType.includes('U8') || 
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
        } else if (isNode(innerType, 'stringTypeNode') || 
                   isNode(innerType, 'bytesTypeNode')) {
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
