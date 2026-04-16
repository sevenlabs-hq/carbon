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
import { getPostgresTypeManifestVisitor } from './getPostgresTypeManifestVisitor';
import { FlattenedGraphQLField, flattenTypeForGraphQL } from './utils/flattenGraphqlFields';
import {
    generateDecoderCargoToml,
    getReadmeDisplayName,
    hasPackageMetadata,
    type PackageMetadata,
} from './cargoTomlGenerator';
import { formatDocComments } from './utils/render';
import { PostgresRowMapper, type FlattenedField } from './postgresRowMapper';
import { checkRequiresBigArray } from './utils/postgresHelpers';

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
    withBase58?: boolean;
    standalone?: boolean;
    workspaceDeps?: boolean;
    packageMetadata?: PackageMetadata;
    version?: string;
    versionName?: string;
};

export function getRenderMapVisitor(options: GetRenderMapOptions = {}) {
    const renderParentInstructions = options.renderParentInstructions ?? false;
    let definedTypesMap: Map<string, any> | null = null;
    const newtypeWrapperTypes = new Set<string>(); // Track which types were converted to newtype wrappers
    const createTypeManifestVisitor = () =>
        getTypeManifestVisitor(definedTypesMap, newtypeWrapperTypes, options.withBase58 ?? false);
    let typeManifestVisitor = createTypeManifestVisitor();
    const postgresTypeManifestVisitor = getPostgresTypeManifestVisitor();

    const rowMapper = new PostgresRowMapper({
        getTypeManifestVisitor: () => typeManifestVisitor,
        postgresTypeManifestVisitor,
        newtypeWrapperTypes,
        getDefinedTypesMap: () => definedTypesMap,
    });

    let currentProgram: ProgramNode | null = null;
    // Track which instructions have GraphQL schemas generated
    const instructionsWithGraphQLSchemas = new Set<string>();

    // Track if any types require serde-big-array
    let requiresSerdeBigArray = false;

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

                    // Check if this account's fields require serde-big-array
                    if (checkRequiresBigArray(newNode.data)) {
                        requiresSerdeBigArray = true;
                    }

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

                    const flatFields = rowMapper.flattenType(newNode.data, [], [], new Set());
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
                                options.postgresMode === 'generic' || options.withPostgres === false
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

                    // Check if this type requires serde-big-array
                    if (checkRequiresBigArray(node.type)) {
                        requiresSerdeBigArray = true;
                    }

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

                        // Track if any argument requires serde-big-array
                        if (checkRequiresBigArray(arg.type)) {
                            requiresSerdeBigArray = true;
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
                    const flatFields = rowMapper.flattenType(
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
                            withBase58: options.withBase58 ?? false,
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
                            options.postgresMode === 'generic' || options.withPostgres === false
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
                        withBase58: options.withBase58 ?? false,
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
                            options.postgresMode === 'generic' || options.withPostgres === false
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
                                options.postgresMode === 'generic' || options.withPostgres === false
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
                                options.postgresMode === 'generic' || options.withPostgres === false
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
                            options.postgresMode === 'generic' || options.withPostgres === false
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
                            // Use generic query template when postgres mode is generic or when postgres is disabled
                            // (typed template needs per-instruction/account postgres rows which are only generated with postgres)
                            if (options.postgresMode === 'generic' || options.withPostgres === false) {
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
                        version: options.version,
                        versionName: options.versionName,
                        withPostgres: options.withPostgres !== false,
                        withGraphQL: options.withGraphql !== false,
                        withSerde: options.withSerde ?? false,
                        withBase58: options.withBase58 ?? false,
                        withSerdeBigArray: requiresSerdeBigArray,
                        standalone: options.standalone !== false,
                        workspaceDeps: options.workspaceDeps,
                        packageMetadata: options.packageMetadata,
                    });
                    map.add('Cargo.toml', cargoToml);

                    if (hasPackageMetadata(options.packageMetadata)) {
                        const readmeDisplayName = getReadmeDisplayName(
                            options.packageName,
                            programName,
                            options.packageMetadata,
                        );
                        map.add('README.md', `# Carbon ${readmeDisplayName} Decoder\n`);
                    }

                    return map.mergeWith(programRenderMap);
                },
            }),
    );
}
