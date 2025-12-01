import {
    camelCase,
    DefinedTypeNode,
    EnumTypeNode,
    getAllAccounts,
    getAllDefinedTypes,
    getAllInstructionsWithSubs,
    isNode,
    pascalCase,
    ProgramNode,
    resolveNestedTypeNode,
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
import { isToken2022Program } from './utils/helpers';
import { getPostgresTypeManifestVisitor, PostgresTypeManifest } from './getPostgresTypeManifestVisitor';
import { FlattenedGraphQLField, flattenTypeForGraphQL } from './utils/flattenGraphqlFields';
import { generateDecoderCargoToml } from './cargoTomlGenerator';
import { formatDocComments } from './utils/render';

export type GetRenderMapOptions = {
    renderParentInstructions?: boolean;
    packageName?: string;
    anchorEvents?: {
        name: string;
        discriminator: number[];
    }[];
    postgresMode?: 'generic' | 'typed';
    withPostgres?: boolean;
    withGraphql?: boolean;
    withSerde?: boolean;
    standalone?: boolean;
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
    needsBigArray?: boolean;
    isCopyType?: boolean;
};

export function getRenderMapVisitor(options: GetRenderMapOptions = {}) {
    const renderParentInstructions = options.renderParentInstructions ?? false;
    let definedTypesMap: Map<string, any> | null = null;
    const newtypeWrapperTypes = new Set<string>(); // Track which types were converted to newtype wrappers
    const createTypeManifestVisitor = () => getTypeManifestVisitor(definedTypesMap, newtypeWrapperTypes);
    let typeManifestVisitor = createTypeManifestVisitor();
    const postgresTypeManifestVisitor = getPostgresTypeManifestVisitor();

    let currentProgram: ProgramNode | null = null;
    // Track which instructions have GraphQL schemas generated
    const instructionsWithGraphQLSchemas = new Set<string>();

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
                    const imports = new ImportMap().mergeWithManifest(typeManifest);

                    // Add token-2022 specific imports for accounts that need extension handling
                    // Note: StateWithExtensions is used in accounts/mod.rs, not in individual account files
                    // Extension is only needed for Mint and Token accounts (not Multisig)
                    if (isToken2022Program(currentProgram)) {
                        const accountNameLower = node.name.toLowerCase();
                        if (accountNameLower === 'mint' || accountNameLower === 'token') {
                            imports.add('spl_token_2022::extension::BaseStateWithExtensions');
                            imports.add('crate::types::Extension');
                        }
                        if (accountNameLower === 'token') {
                            imports.add('crate::types::AccountState');
                        }
                        // Multisig doesn't need Extension import - remove it if added
                    }

                    // Build a map of offset -> field name for nested discriminator support
                    const discriminatorNames = new Map<number, string>();
                    for (const discriminator of discriminators) {
                        if (discriminator.kind === 'fieldDiscriminatorNode') {
                            discriminatorNames.set(discriminator.offset, discriminator.name);
                        }
                    }

                    const discriminatorManifest =
                        discriminators.length > 0
                            ? getDiscriminatorManifest(discriminators, currentProgram?.name, discriminatorNames)
                            : undefined;

                    const flatFields = flattenType(newNode.data, [], [], new Set());
                    const postgresImports = new ImportMap()
                        .add('carbon_core::account::AccountMetadata')
                        .add('carbon_core::postgres::metadata::AccountRowMetadata');
                    flatFields.forEach(f => {
                        postgresImports.mergeWith(f.postgresManifest.imports);
                    });

                    // Process all docs together to maintain list context across doc strings
                    const formattedAccountDocs =
                        newNode.docs && newNode.docs.length > 0 ? formatDocComments(newNode.docs, '') : '';

                    let renderMap = new RenderMap().add(
                        `src/accounts/${snakeCase(node.name)}.rs`,
                        render('accountsPage.njk', {
                            account: { ...newNode, formattedDocs: formattedAccountDocs },
                            imports: imports.toString(),
                            program: currentProgram,
                            originalProgramName: currentProgram?.name,
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
                    }

                    return renderMap;
                },

                visitDefinedType(node) {
                    const typeManifest = visit(node.type, typeManifestVisitor);
                    const imports = new ImportMap().mergeWithManifest(typeManifest);

                    // Use newtype wrapper instead of type alias to allow BigArray attribute for large arrays
                    let needsNewtypeWrapper = false;
                    let arraySize: number | undefined = undefined;
                    if (node.type.kind !== 'structTypeNode' && node.type.kind !== 'enumTypeNode') {
                        // Resolve the underlying type to check if it's a large array
                        let resolvedRaw: any = undefined;
                        if (isNode(node.type, 'fixedSizeTypeNode')) {
                            resolvedRaw = node.type;
                        } else if (isNode(node.type, 'arrayTypeNode')) {
                            resolvedRaw = node.type;
                        } else if (isNode(node.type, 'definedTypeLinkNode')) {
                            // For type aliases, resolve through the map or nested resolution
                            if (definedTypesMap) {
                                const typeName = node.type.name;
                                const definedType = definedTypesMap.get(typeName);
                                if (definedType && definedType.type) {
                                    resolvedRaw = definedType.type;
                                }
                            }
                            if (!resolvedRaw) {
                                resolvedRaw = resolveNestedTypeNode(node.type);
                            }
                        }
                        if (resolvedRaw) {
                            const resolved = resolvedRaw as TypeNode;
                            if (isNode(resolved, 'fixedSizeTypeNode')) {
                                if (isNode(resolved.type, 'bytesTypeNode') && resolved.size > 32) {
                                    needsNewtypeWrapper = true;
                                    arraySize = resolved.size;
                                }
                            } else if (isNode(resolved, 'arrayTypeNode')) {
                                if (isNode(resolved.count, 'fixedCountNode') && resolved.count.value > 32) {
                                    needsNewtypeWrapper = true;
                                    arraySize = resolved.count.value;
                                }
                            }
                        }
                    }

                    if (needsNewtypeWrapper) {
                        newtypeWrapperTypes.add(node.name);
                    }

                    // Add token-2022 specific imports for Extension type
                    // Use case-insensitive check to match impl block condition
                    const isToken2022ForImports = isToken2022Program(currentProgram);
                    const isExtensionTypeForImports = node.name === 'Extension' || node.name === 'extension';
                    if (isToken2022ForImports && isExtensionTypeForImports) {
                        imports.add('spl_token_2022::extension::StateWithExtensions');
                        imports.add('spl_token_2022::extension::BaseStateWithExtensions as _');
                        imports.add('spl_token_2022::extension::ExtensionType');
                        // solana_zk_sdk is used via full path spl_token_2022::solana_zk_sdk::*, no direct import needed
                        imports.add('spl_token_2022::solana_zk_sdk::encryption::elgamal::ElGamalPubkey');
                        imports.add('spl_token_2022::solana_zk_sdk::encryption::pod::elgamal::PodElGamalPubkey');
                        imports.add('spl_pod::primitives::PodI64');
                        imports.add('spl_type_length_value::variable_len_pack::VariableLenPack');
                        // bytemuck is used via full path spl_pod::bytemuck::*, no direct import needed
                    }

                    let typeContent = render('typesPage.njk', {
                        definedType: node,
                        imports: imports.toString(),
                        typeManifest,
                        needsNewtypeWrapper,
                        arraySize,
                        program: currentProgram,
                        originalProgramName: currentProgram?.name,
                    });

                    // Add Extension impl block for token-2022
                    // Check both program name and node name (case-insensitive for node name)
                    const isToken2022 = isToken2022Program(currentProgram);
                    const isExtensionType = node.name === 'Extension' || node.name === 'extension';
                    if (isToken2022 && isExtensionType) {
                        const extensionImpl = render('extensionImpl.njk', {});
                        typeContent += '\n\n' + extensionImpl;
                    }

                    let renderMap = new RenderMap().add(`src/types/${snakeCase(node.name)}.rs`, typeContent);

                    for (let event of options.anchorEvents ?? []) {
                        if (camelCase(event.name) == node.name) {
                            let discriminatorManifest: DiscriminatorManifest = {
                                bytes: `[${event.discriminator.join(', ')}]`,
                                size: event.discriminator.length,
                                checkCode: `        if data.len() < ${event.discriminator.length} {
            return None;
        }
        let discriminator = &data[0..${event.discriminator.length}];
        if discriminator != [${event.discriminator.join(', ')}] {
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

                    // GraphQL generation for structs and enums - if withGraphql is enabled
                    if (options.withGraphql !== false) {
                        if (node.type.kind === 'structTypeNode') {
                            if (node.type.fields.length > 0) {
                                const graphqlFields = flattenTypeForGraphQL(node.type, [], [], new Set());
                                const graphqlImports = new ImportMap().add('juniper::GraphQLObject');
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
                                const emptyStructImports = new ImportMap().add(
                                    'carbon_core::graphql::primitives::Json',
                                );
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
                    const imports = new ImportMap().add('carbon_core::deserialize::ArrangeAccounts');

                    if (node.accounts && node.accounts.length > 0) {
                        imports.add('carbon_core::account_utils::next_account');
                    }

                    let discriminators = node.discriminators ?? [];

                    // Build a set of discriminator field names to identify all discriminator arguments
                    const discriminatorFieldNames = new Set<string>();
                    for (const discriminator of discriminators) {
                        if (discriminator.kind === 'fieldDiscriminatorNode') {
                            discriminatorFieldNames.add(discriminator.name);
                        }
                    }

                    // Build a map of offset -> field name for nested discriminator support
                    const discriminatorNames = new Map<number, string>();
                    for (const discriminator of discriminators) {
                        if (discriminator.kind === 'fieldDiscriminatorNode') {
                            discriminatorNames.set(discriminator.offset, discriminator.name);
                        }
                    }

                    const [discriminatorArguments, regularArguments] = partition(node.arguments, arg =>
                        discriminatorFieldNames.has(arg.name),
                    );

                    const argumentTypes = regularArguments.map(arg => {
                        const manifest = visit(arg.type, typeManifestVisitor);
                        imports.mergeWithManifest(manifest);

                        // Newtype wrapper handles serialization itself, so don't apply BigArray
                        let requiredBigArray = manifest.requiredBigArray;
                        if (isNode(arg.type, 'definedTypeLinkNode') && newtypeWrapperTypes.has(arg.type.name)) {
                            requiredBigArray = undefined;
                        }

                        return {
                            ...manifest,
                            requiredBigArray,
                        };
                    });

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

                    const discriminatorManifest = getDiscriminatorManifest(
                        discriminators,
                        currentProgram?.name,
                        discriminatorNames,
                    );

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
                        graphqlImports.add('juniper::GraphQLObject');
                        graphqlImports.add('serde_json');

                        // Always generate schema files for all instructions, even if they have no arguments
                        // Instructions without arguments will only have instruction_metadata field
                        instructionsWithGraphQLSchemas.add(node.name);
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

                    // Build a map of defined types for type resolution
                    definedTypesMap = new Map();
                    const allDefinedTypes = getAllDefinedTypes(node);
                    for (const definedType of allDefinedTypes) {
                        definedTypesMap.set(definedType.name, definedType);
                    }
                    // Recreate the type manifest visitor with the defined types map
                    typeManifestVisitor = createTypeManifestVisitor();

                    // Override program.name with packageName if provided (for custom decoder names)
                    // Keep original format - template filter will handle PascalCase conversion
                    const originalProgramName = program.name || '';
                    const programName = options.packageName || originalProgramName || 'decoder';

                    const programWithCustomName = {
                        ...program,
                        name: programName,
                    };

                    // Visit the program first to populate instructionsWithGraphQLSchemas Set
                    const programRenderMap = visit(program, self);

                    // Use getAll* functions but they will only process the main program
                    const accountsToExport = getAllAccounts(node);
                    const instructionsToExport = getAllInstructionsWithSubs(program, {
                        leavesOnly: !renderParentInstructions,
                    });
                    const definedTypesToExport = allDefinedTypes;

                    // Compute hasGraphQLFields: whether any GraphQL query fields will be generated
                    const hasAnchorEvents = (options.anchorEvents?.length ?? 0) > 0;
                    const hasGraphQLFields = (() => {
                        if (hasAnchorEvents) return true;
                        if (options.postgresMode === 'generic') {
                            // Generic mode: any accounts or instructions exist
                            return accountsToExport.length > 0 || instructionsToExport.length > 0;
                        } else {
                            // Typed mode: accounts with structTypeNode and fields, or any instructions exist
                            return (
                                accountsToExport.some(
                                    acc => acc.data.kind === 'structTypeNode' && acc.data.fields.length > 0,
                                ) || instructionsToExport.length > 0
                            );
                        }
                    })();

                    const ctx = {
                        accountsToExport,
                        definedTypesToExport,
                        instructionsToExport,
                        program: programWithCustomName,
                        root: node,
                        packageName: options.packageName,
                        originalProgramName: originalProgramName, // Keep original program name for token-2022 checks
                        hasAnchorEvents,
                        hasGraphQLFields,
                        events: options.anchorEvents ?? [],
                        postgresMode: options.postgresMode || 'typed',
                        withPostgres: options.withPostgres !== false,
                        withGraphQL: options.withGraphql !== false,
                        withSerde: options.withSerde ?? false,
                    };

                    const map = new RenderMap();

                    // Generate mod files
                    // Build mod-level imports via ImportMap
                    const accountsModImports = new ImportMap()
                        .add('crate::PROGRAM_ID')
                        .add(`crate::${pascalCase(programName)}Decoder`);

                    // Add token-2022 specific imports for StateWithExtensions unpacking
                    if (isToken2022Program(program, originalProgramName)) {
                        accountsModImports.add('solana_program_pack::Pack');
                        // StateWithExtensions is used directly in unpack() calls, no import needed
                    }

                    map.add(
                        'src/accounts/mod.rs',
                        render('accountsMod.njk', { ...ctx, imports: accountsModImports.toString() }),
                    );
                    if (options.postgresMode !== 'generic' && options.withPostgres !== false) {
                        map.add('src/accounts/postgres/mod.rs', render('accountsPostgresMod.njk', ctx));
                    }
                    if (options.withGraphql !== false) {
                        const accountsGraphqlTemplate =
                            options.postgresMode === 'generic'
                                ? 'accountsGraphqlModGeneric.njk'
                                : 'accountsGraphqlMod.njk';
                        const accountsGraphqlImports = new ImportMap().add('juniper::GraphQLObject');
                        map.add(
                            'src/accounts/graphql/mod.rs',
                            render(accountsGraphqlTemplate, { ...ctx, imports: accountsGraphqlImports.toString() }),
                        );
                    }
                    if (instructionsToExport.length > 0) {
                        const instructionsModImports = new ImportMap()
                            .add('crate::PROGRAM_ID')
                            .add(`crate::${pascalCase(programName)}Decoder`);
                        map.add(
                            'src/instructions/mod.rs',
                            render('instructionsMod.njk', { ...ctx, imports: instructionsModImports.toString() }),
                        );
                        if (options.postgresMode !== 'generic' && options.withPostgres !== false) {
                            map.add('src/instructions/postgres/mod.rs', render('instructionsPostgresMod.njk', ctx));
                        }
                        if (options.withGraphql !== false) {
                            const instructionsGraphqlTemplate =
                                options.postgresMode === 'generic'
                                    ? 'instructionsGraphqlModGeneric.njk'
                                    : 'instructionsGraphqlMod.njk';
                            const instructionsGraphqlImports = new ImportMap().add('juniper::GraphQLObject');
                            // Filter instructions to only include those that have GraphQL schemas generated
                            const instructionsWithSchemas = instructionsToExport.filter(inst =>
                                instructionsWithGraphQLSchemas.has(inst.name),
                            );
                            map.add(
                                'src/instructions/graphql/mod.rs',
                                render(instructionsGraphqlTemplate, {
                                    ...ctx,
                                    instructionsToExport: instructionsWithSchemas,
                                    imports: instructionsGraphqlImports.toString(),
                                }),
                            );
                        }
                    }

                    if (options.anchorEvents?.length ?? 0 > 0) {
                        const eventInstructionImports = new ImportMap()
                            .add('carbon_core::borsh')
                            .add('carbon_core::deserialize::ArrangeAccounts');
                        map.add(
                            'src/instructions/cpi_event.rs',
                            render('eventInstructionPage.njk', { ...ctx, imports: eventInstructionImports.toString() }),
                        );
                        if (options.postgresMode !== 'generic' && options.withPostgres !== false) {
                            const eventInstructionRowImports = new ImportMap()
                                .add('carbon_core::postgres::metadata::InstructionRowMetadata')
                                .add('carbon_core::instruction::InstructionMetadata')
                                .add('super::super::cpi_event::CpiEvent');
                            map.add(
                                'src/instructions/postgres/cpi_event_row.rs',
                                render('eventInstructionRowPage.njk', {
                                    ...ctx,
                                    imports: eventInstructionRowImports.toString(),
                                }),
                            );
                        }
                        if (options.withGraphql !== false) {
                            const cpiEventSchemaTemplate =
                                options.postgresMode === 'generic'
                                    ? 'eventInstructionGraphqlSchemaPageGeneric.njk'
                                    : 'eventInstructionGraphqlSchemaPage.njk';
                            const cpiEventSchemaImports = new ImportMap()
                                .add('juniper::GraphQLObject')
                                .add('serde_json');
                            map.add(
                                'src/instructions/graphql/cpi_event_schema.rs',
                                render(cpiEventSchemaTemplate, { ...ctx, imports: cpiEventSchemaImports.toString() }),
                            );
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
                        // Filter instructions to only include those that have GraphQL schemas generated
                        const instructionsWithSchemas = instructionsToExport.filter(inst =>
                            instructionsWithGraphQLSchemas.has(inst.name),
                        );

                        // Check if there are actually any GraphQL fields to generate
                        const hasAccountsWithFields =
                            options.postgresMode === 'generic'
                                ? accountsToExport.length > 0
                                : accountsToExport.some(
                                      acc => acc.data.kind === 'structTypeNode' && acc.data.fields.length > 0,
                                  );
                        const hasActualGraphQLFields =
                            hasAnchorEvents || hasAccountsWithFields || instructionsWithSchemas.length > 0;

                        map.add('src/graphql/mod.rs', render('graphqlRootMod.njk', { ...ctx, hasActualGraphQLFields }));
                        map.add('src/graphql/context.rs', render('graphqlContextPage.njk', ctx));

                        // Only generate query.rs if there are GraphQL fields to expose
                        if (hasActualGraphQLFields) {
                            // Use different query template based on postgres mode
                            if (options.postgresMode === 'generic') {
                                const graphqlQueryGenericImports = new ImportMap().add(
                                    'juniper::{graphql_object, FieldResult}',
                                );
                                map.add(
                                    'src/graphql/query.rs',
                                    render('graphqlQueryPageGeneric.njk', {
                                        ...ctx,
                                        instructionsToExport: instructionsWithSchemas,
                                        imports: graphqlQueryGenericImports.toString(),
                                        hasAccountsWithFields,
                                    }),
                                );
                            } else {
                                const graphqlQueryImports = new ImportMap().add(
                                    'juniper::{graphql_object, FieldResult}',
                                );
                                // Only add FromStr if there are accounts with fields that need it
                                if (hasAccountsWithFields) {
                                    graphqlQueryImports.add('std::str::FromStr');
                                }
                                map.add(
                                    'src/graphql/query.rs',
                                    render('graphqlQueryPage.njk', {
                                        ...ctx,
                                        instructionsToExport: instructionsWithSchemas,
                                        imports: graphqlQueryImports.toString(),
                                        hasAccountsWithFields,
                                    }),
                                );
                            }
                        }
                    }

                    map.add('src/lib.rs', render('lib.njk', ctx));

                    const cargoToml = generateDecoderCargoToml({
                        packageName: options.packageName,
                        programName: programName,
                        originalProgramName: originalProgramName, // Pass original program name for token-2022 checks
                        withPostgres: options.withPostgres !== false,
                        withGraphQL: options.withGraphql !== false,
                        withSerde: options.withSerde ?? false,
                        standalone: options.standalone !== false,
                    });
                    map.add('Cargo.toml', cargoToml);

                    return map.mergeWith(programRenderMap);
                },
            }),
    );

    function isPostgresPrimitiveType(type: string): boolean {
        return (
            type.includes('U8') ||
            type.includes('U16') ||
            type.includes('U32') ||
            type.includes('U64') ||
            type.includes('U128') ||
            type.includes('I128')
        );
    }

    // Determines if a Rust type is Copy based on its string representation.
    function isCopyType(rowType: string): boolean {
        const trimmed = rowType.trim();

        const copyPrimitives = [
            'bool',
            'i8',
            'i16',
            'i32',
            'i64',
            'f32',
            'f64',
            'carbon_core::postgres::primitives::Pubkey',
            'Pubkey', // May be used without full path
            'carbon_core::postgres::primitives::U8',
            'U8',
            'carbon_core::postgres::primitives::U16',
            'U16',
            'carbon_core::postgres::primitives::U32',
            'U32',
        ];

        if (trimmed.startsWith('Option<')) {
            const innerMatch = trimmed.match(/^Option<(.+)>$/);
            if (innerMatch) {
                const innerType = innerMatch[1].trim();
                return isCopyType(innerType);
            }
        }

        // Json<T> implements Copy if T implements Copy
        const jsonMatch = trimmed.match(/(?:sqlx::types::)?Json<(.+)>/);
        if (jsonMatch) {
            const innerType = jsonMatch[1].trim();
            const unwrappedType = innerType.replace(/^Option<(.+)>$/, '$1');
            const lowerType = unwrappedType.toLowerCase();
            if (lowerType === 'bool' || lowerType.endsWith('bool') || lowerType.includes('::bool')) {
                return true;
            }
            return isCopyType(unwrappedType);
        }

        if (copyPrimitives.includes(trimmed)) {
            return true;
        }

        return false;
    }

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

        const flattenOptionType = (typeNode: TypeNode, prefix: string[], docsPrefix: string[]): FlattenedField[] => {
            const column = makeName(prefix);
            const itemType = isNode(typeNode, 'optionTypeNode')
                ? typeNode.item
                : isNode(typeNode, 'zeroableOptionTypeNode')
                  ? typeNode.item
                  : (typeNode as any).item; // remainderOptionTypeNode
            const manifest = visit(itemType, postgresTypeManifestVisitor) as PostgresTypeManifest;
            const isJson = (manifest.postgresColumnType || '').toUpperCase().startsWith('JSONB');

            const rowType = isJson
                ? manifest.sqlxType.includes('Json<')
                    ? `Option<${manifest.sqlxType}>`
                    : `Option<sqlx::types::Json<${manifest.sqlxType}>>`
                : `Option<${manifest.sqlxType}>`;

            const innerRustManifest = visit(itemType, typeManifestVisitor);
            // Pubkey always needs conversion
            const needsConversion = isNode(itemType, 'publicKeyTypeNode')
                ? true
                : !typesMatch(manifest.sqlxType, innerRustManifest.type);

            const sourceField = `source.${prefix.join('.')}`;
            const sourceColumn = `source.${column}`;

            const expr = isJson
                ? manifest.sqlxType.includes('Json<')
                    ? needsConversion
                        ? `${sourceField}.map(|value| value.into())`
                        : sourceField
                    : needsConversion
                      ? `${sourceField}.map(|value| sqlx::types::Json(value.into()))`
                      : `${sourceField}.map(sqlx::types::Json)`
                : needsConversion
                  ? `${sourceField}.map(|value| value.into())`
                  : sourceField;

            const reverseExpr = isJson
                ? `${sourceColumn}.map(|value| value.0)`
                : buildReverseOptionType(typeNode, sourceColumn, manifest);

            return [
                {
                    column,
                    rustPath: prefix.join('.'),
                    rowType,
                    postgresColumnType: `${manifest.postgresColumnType}`,
                    docs: docsPrefix,
                    postgresManifest: manifest,
                    expr,
                    reverseExpr,
                    isCopyType: isCopyType(rowType),
                },
            ];
        };

        if (isNode(typeNode, 'structTypeNode')) {
            for (const field of typeNode.fields) {
                out.push(...flattenType(field.type, [...prefix, snakeCase(field.name)], [], seen, { inOption }));
            }
            return out;
        }

        if (isNode(typeNode, 'optionTypeNode')) {
            return flattenOptionType(typeNode, prefix, docsPrefix);
        }

        // Handle zeroableOptionTypeNode, remainderOptionTypeNode - same as optionTypeNode
        if (isNode(typeNode, 'zeroableOptionTypeNode') || isNode(typeNode, 'remainderOptionTypeNode')) {
            return flattenOptionType(typeNode, prefix, docsPrefix);
        }

        // Handle hiddenPrefixTypeNode - unwrap and process inner type
        if (isNode(typeNode, 'hiddenPrefixTypeNode')) {
            return flattenType(typeNode.type, prefix, docsPrefix, seen, opts);
        }

        if (isNode(typeNode, 'definedTypeLinkNode')) {
            const column = makeName(prefix);
            const manifest = visit(typeNode, postgresTypeManifestVisitor) as PostgresTypeManifest;
            const isJson = (manifest.postgresColumnType || '').toUpperCase().startsWith('JSONB');
            const typeManifest = visit(typeNode, typeManifestVisitor);
            const originalTypeName = typeManifest.type;

            const rowType = isJson ? `sqlx::types::Json<${manifest.sqlxType}>` : `${manifest.sqlxType}`;

            // Check if Postgres type matches original type (no conversion needed)
            const needsConversion = manifest.sqlxType !== originalTypeName;
            const sourceField = `source.${prefix.join('.')}`;
            const expr = isJson
                ? `sqlx::types::Json(${needsConversion ? `${sourceField}.into()` : sourceField})`
                : needsConversion
                  ? `${sourceField}.into()`
                  : sourceField;

            // For reverse conversion, check if types match
            const sourceColumn = `source.${column}`;
            const reverseExpr = isJson
                ? `${sourceColumn}.0`
                : needsConversion
                  ? `${sourceColumn}.into()`
                  : sourceColumn;

            // Check if this defined type alias resolves to a large fixed-size array that needs BigArray
            // But skip if it's a newtype wrapper (which handles serialization itself)
            let needsBigArray = false;
            // If this type was converted to a newtype wrapper, don't apply BigArray attribute
            if (!newtypeWrapperTypes.has(typeNode.name)) {
                // Resolve to check if underlying type is a large array
                let resolvedRaw: any = undefined;
                if (definedTypesMap) {
                    const definedType = definedTypesMap.get(typeNode.name);
                    if (definedType && definedType.type) {
                        resolvedRaw = definedType.type;
                    }
                }
                if (!resolvedRaw) {
                    resolvedRaw = resolveNestedTypeNode(typeNode);
                }
                if (resolvedRaw) {
                    const resolved = resolvedRaw as TypeNode;
                    if (
                        isNode(resolved, 'fixedSizeTypeNode') &&
                        isNode(resolved.type, 'bytesTypeNode') &&
                        resolved.size > 32
                    ) {
                        needsBigArray = true;
                    } else if (
                        isNode(resolved, 'arrayTypeNode') &&
                        isNode(resolved.count, 'fixedCountNode') &&
                        resolved.count.value > 32
                    ) {
                        needsBigArray = true;
                    }
                }
            }

            out.push({
                column,
                rustPath: prefix.join('.'),
                rowType,
                postgresColumnType: `${manifest.postgresColumnType} NOT NULL`,
                docs: docsPrefix,
                postgresManifest: manifest,
                expr,
                reverseExpr,
                needsBigArray,
                isCopyType: isCopyType(rowType),
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
            isCopyType: isCopyType(manifest.sqlxType),
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
                if (isNode(typeNode.item, 'publicKeyTypeNode')) {
                    return `${prefix}.into_iter().map(|element| element.into()).collect()`;
                }

                const postgresManifest = visit(typeNode.item, postgresTypeManifestVisitor) as PostgresTypeManifest;
                const rustManifest = visit(typeNode.item, typeManifestVisitor);
                const postgresType = postgresManifest.sqlxType;
                const rustType = rustManifest.type;
                const isPostgresWrapper = isPostgresPrimitiveType(postgresType);

                if (postgresType === rustType && !isPostgresWrapper) {
                    return `${prefix}.to_vec()`;
                }
                return `${prefix}.into_iter().map(|element| element.into()).collect()`;
            } else {
                const innerExpr = buildExpression(typeNode.item, `element`);
                if (innerExpr === 'element') {
                    return `sqlx::types::Json(${prefix}.to_vec())`;
                }
                return `sqlx::types::Json(${prefix}.into_iter().map(|element| ${innerExpr}).collect())`;
            }
        } else if (isNode(typeNode, 'fixedSizeTypeNode')) {
            if (isNode(typeNode.type, 'bytesTypeNode')) {
                return `${prefix}.to_vec()`;
            }
            return buildExpression(typeNode.type, prefix);
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
        } else if (isNode(typeNode, 'publicKeyTypeNode')) {
            // Pubkey from Rust needs conversion to Postgres Pubkey (solana_pubkey::Pubkey)
            return `${prefix}.into()`;
        } else if (isNode(typeNode, 'booleanTypeNode') || isNode(typeNode, 'stringTypeNode')) {
            return prefix;
        } else {
            try {
                const postgresManifest = visit(typeNode, postgresTypeManifestVisitor) as PostgresTypeManifest;
                const rustManifest = visit(typeNode, typeManifestVisitor);
                if (typesMatch(postgresManifest.sqlxType, rustManifest.type)) {
                    return prefix;
                }
            } catch (e) {
                // If we can't get the manifests, fall back to .into()
            }
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
            return prefix;
        } else if (isNode(innerType, 'numberTypeNode')) {
            const rustManifest = visit(innerType, typeManifestVisitor);
            const rustType = rustManifest.type;
            const postgresType = manifest.sqlxType;

            if (typesMatch(postgresType, rustType)) {
                return prefix;
            }

            const isPostgresPrimitive = isPostgresPrimitiveType(manifest.sqlxType);

            if (isPostgresPrimitive) {
                // U8, U16, U32 need try_into() with transpose() to handle Result
                if (
                    manifest.sqlxType.includes('U16') ||
                    manifest.sqlxType.includes('U32') ||
                    manifest.sqlxType.includes('U8')
                ) {
                    return `${prefix}.map(|value| value.try_into().map_err(|_| carbon_core::error::Error::Custom("Failed to convert value from postgres primitive".to_string()))).transpose()?`;
                } else {
                    return `${prefix}.map(|value| *value)`;
                }
            } else {
                return prefix;
            }
        } else if (isNode(innerType, 'publicKeyTypeNode')) {
            // Use * to dereference since carbon_core::postgres::primitives::Pubkey implements Deref
            return `${prefix}.map(|value| *value)`;
        } else if (isNode(innerType, 'stringTypeNode') || isNode(innerType, 'bytesTypeNode')) {
            return `${prefix}.map(|value| *value)`;
        } else {
            const rustManifest = visit(innerType, typeManifestVisitor);
            if (typesMatch(manifest.sqlxType, rustManifest.type)) {
                return prefix;
            }
            return `${prefix}.map(|value| value.into())`;
        }
    }

    /**
     * Checks if two types match, accounting for Json wrappers and type aliases.
     * Strips Json<...> wrappers and compares base types.
     */
    function typesMatch(postgresType: string, rustType: string): boolean {
        let pgType = postgresType.trim();
        const jsonMatch = pgType.match(/^Json<(.+)>$/);
        if (jsonMatch) {
            pgType = jsonMatch[1].trim();
        }

        const rustTypeBase = rustType.trim();

        if (pgType === rustTypeBase) {
            return true;
        }

        // Handle cases like sqlx::types::Json<T> vs T
        if (pgType.endsWith(rustTypeBase) || rustTypeBase.endsWith(pgType)) {
            const pgBase = pgType.split('::').pop() || pgType;
            const rustBase = rustTypeBase.split('::').pop() || rustTypeBase;
            if (pgBase === rustBase) {
                return true;
            }
        }

        return false;
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
                        const innerReverse = buildReverse(typeNode.item, 'element');
                        // If buildReverse already returns a Result (contains '?'), the collect gives us Result<Vec<T>, E>
                        // Then we need to convert Vec<T> to [T; N] using try_into()
                        if (innerReverse.includes('?')) {
                            // innerReverse contains ?, so it returns Result<T, E> for each element
                            // Collect gives Result<Vec<T>, E>, then convert Vec<T> to [T; N]
                            // Remove the ? from innerReverse and handle Result properly
                            const innerWithoutQuestion = innerReverse.replace(/\?$/, '');
                            return (
                                `${prefix}.0.into_iter().map(|element| ${innerWithoutQuestion}).collect::<Result<Vec<_>, carbon_core::error::Error>>()?` +
                                `.try_into().map_err(|_| carbon_core::error::Error::Custom("Failed to convert value from postgres primitive".to_string()))?`
                            );
                        } else {
                            // For simple expressions, wrap in Ok() for collect::<Result<Vec<_>, carbon_core::error::Error>>()
                            // Then convert Vec<T> to [T; N] using try_into()
                            // Check if innerReverse is just the element itself - if so, use Ok directly instead of closure
                            const mapExpr = innerReverse === 'element' ? 'Ok' : `|element| Ok(${innerReverse})`;
                            return (
                                `${prefix}.0.into_iter().map(${mapExpr}).collect::<Result<Vec<_>, carbon_core::error::Error>>()` +
                                `.map_err(|_| carbon_core::error::Error::Custom("Failed to collect array elements".to_string()))?` +
                                `.try_into().map_err(|_| carbon_core::error::Error::Custom("Failed to convert value from postgres primitive".to_string()))?`
                            );
                        }
                    } else {
                        const innerReverse = buildReverse(typeNode.item, 'element');
                        // If buildReverse already returns a Result (contains '?'), don't wrap in Ok()
                        if (innerReverse.includes('?')) {
                            // innerReverse contains ?, so it returns Result<T, E> for each element
                            // Collect gives Result<Vec<T>, E>, then convert Vec<T> to [T; N]
                            // Remove the ? from innerReverse and handle Result properly
                            const innerWithoutQuestion = innerReverse.replace(/\?$/, '');
                            return `${prefix}.into_iter().map(|element| ${innerWithoutQuestion}).collect::<Result<Vec<_>, carbon_core::error::Error>>()?.try_into().map_err(|_| carbon_core::error::Error::Custom("Failed to convert array element to primitive".to_string()))?`;
                        } else {
                            // For simple expressions, wrap in Ok() for collect::<Result<Vec<_>, carbon_core::error::Error>>()
                            // Then convert Vec<T> to [T; N] using try_into()
                            // Check if innerReverse is just the element itself - if so, use Ok directly instead of closure
                            const mapExpr = innerReverse === 'element' ? 'Ok' : `|element| Ok(${innerReverse})`;
                            return `${prefix}.into_iter().map(${mapExpr}).collect::<Result<Vec<_>, carbon_core::error::Error>>().map_err(|_| carbon_core::error::Error::Custom("Failed to collect array elements".to_string()))?.try_into().map_err(|_| carbon_core::error::Error::Custom("Failed to convert array element to primitive".to_string()))?`;
                        }
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
                        const innerReverse = buildReverse(typeNode.item, 'element');
                        // Check if innerReverse is just the element itself (identity function)
                        if (innerReverse === 'element' || innerReverse === '*element') {
                            return `${prefix}.0`;
                        }
                        return `${prefix}.0.into_iter().map(|element| ${innerReverse}).collect()`;
                    } else {
                        if (isNode(typeNode.item, 'publicKeyTypeNode')) {
                            // Pubkey from Postgres (carbon_core::postgres::primitives::Pubkey) needs conversion to Rust Pubkey (solana_pubkey::Pubkey)
                            // Use * to dereference since carbon_core::postgres::primitives::Pubkey implements Deref to solana_pubkey::Pubkey
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
                        const innerReverse = buildReverse(typeNode.item, 'element');
                        // Check if innerReverse is just the element itself (identity function)
                        // But exclude Pubkey and other types that need conversion
                        if (
                            (innerReverse === 'element' || innerReverse === '*element') &&
                            !isNode(typeNode.item, 'publicKeyTypeNode')
                        ) {
                            return `${prefix}.0`;
                        }
                        return `${prefix}.0.into_iter().map(|element| ${innerReverse}).collect()`;
                    } else {
                        if (isNode(typeNode.item, 'publicKeyTypeNode')) {
                            // Pubkey from Postgres (carbon_core::postgres::primitives::Pubkey) needs conversion to Rust Pubkey (solana_pubkey::Pubkey)
                            // Use * to dereference since carbon_core::postgres::primitives::Pubkey implements Deref to solana_pubkey::Pubkey
                            return `${prefix}.into_iter().map(|element| *element).collect()`;
                        }
                        // Check if conversion is needed by comparing Postgres and Rust types
                        const postgresManifest = visit(
                            typeNode.item,
                            postgresTypeManifestVisitor,
                        ) as PostgresTypeManifest;
                        const rustManifest = visit(typeNode.item, typeManifestVisitor);

                        // If Postgres type matches Rust type, no conversion needed
                        if (typesMatch(postgresManifest.sqlxType, rustManifest.type)) {
                            return `${prefix}.to_vec()`;
                        }
                        // Conversion needed, use try_into()
                        return `${prefix}.into_iter().map(|element| element.try_into()).collect::<Result<_, _>>().map_err(|_| carbon_core::error::Error::Custom("Failed to convert array element to primitive".to_string()))?`;
                    }
                    break;
            }
        }
        if (isNode(typeNode, 'fixedSizeTypeNode')) {
            // Fixed-size bytes: Vec<u8> → [u8; N] with proper error handling
            if (isNode(typeNode.type, 'bytesTypeNode')) {
                return `${prefix}.as_slice().try_into().map_err(|_| carbon_core::error::Error::Custom("Failed to convert padding from postgres primitive: expected ${typeNode.size} bytes".to_string()))?`;
            }
            return buildReverse(typeNode.type, prefix);
        }
        if (isNode(typeNode, 'optionTypeNode')) {
            const innerReverse = buildReverse(typeNode.item, 'value');
            if (innerReverse.includes('?')) {
                const innerWithoutQuestion = innerReverse.replace(/\?$/, '');
                return `${prefix}.map(|value| ${innerWithoutQuestion}).transpose()?`;
            }
            // Check if innerReverse is just the variable (no conversion needed)
            if (innerReverse === 'value' || innerReverse === '*value') {
                return prefix;
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
            // Pubkey from Postgres (carbon_core::postgres::primitives::Pubkey) needs conversion to Rust Pubkey (solana_pubkey::Pubkey)
            // Use * to dereference since carbon_core::postgres::primitives::Pubkey implements Deref to solana_pubkey::Pubkey
            return `*${prefix}`;
        }

        if (isNode(typeNode, 'numberTypeNode')) {
            // Get Postgres type manifest to check if conversion is needed
            const postgresManifest = visit(typeNode, postgresTypeManifestVisitor) as PostgresTypeManifest;
            const rustManifest = visit(typeNode, typeManifestVisitor);
            const postgresType = postgresManifest.sqlxType;
            const rustType = rustManifest.type;

            // Check if types match (no conversion needed)
            if (typesMatch(postgresType, rustType)) {
                return prefix;
            }

            switch (typeNode.format) {
                case 'u8':
                case 'u16':
                case 'u32':
                    return `${prefix}.try_into().map_err(|_| carbon_core::error::Error::Custom("Failed to convert value from postgres primitive".to_string()))?`;
                case 'u64':
                case 'u128':
                case 'i128':
                    return `*${prefix}`;
                case 'i32':
                case 'i64':
                    // Direct types that match between Postgres and Rust
                    if (
                        postgresType === rustType ||
                        (postgresType.includes(rustType) && !postgresType.includes('U') && !postgresType.includes('I'))
                    ) {
                        return prefix;
                    }
                    break;
                default:
                    break;
            }
        }

        // Check if conversion is needed by comparing types
        try {
            const postgresManifest = visit(typeNode, postgresTypeManifestVisitor) as PostgresTypeManifest;
            const rustManifest = visit(typeNode, typeManifestVisitor);
            if (typesMatch(postgresManifest.sqlxType, rustManifest.type)) {
                return prefix;
            }
        } catch (e) {
            // If we can't get manifests, fall back to .into()
        }

        return `${prefix}.into()`;
    }
}
