import {
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

import { getDiscriminatorManifest, getTypeManifestVisitor } from './getTypeManifestVisitor';
import { ImportMap } from './ImportMap';
import { partition, render } from './utils';
import { getPostgresTypeManifestVisitor, PostgresTypeManifest } from './getPostgresTypeManifestVisitor';

export type GetRenderMapOptions = {
    renderParentInstructions?: boolean;
};

type FlattenedField = {
    column: string;
    rustPath: string;
    rowType: string;
    optional: boolean;
    variant?: string;
    accessorVariant?: string;
    expr?: string;
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

                    const discriminatorManifest = getDiscriminatorManifest(discriminators);

                    const flatFields = flattenType(newNode.data, [], [], new Set());
                    const sqlImports = new ImportMap().add(`crate::accounts::${pascalCase(node.name)}`);
                    flatFields.forEach(([f, type]) => {
                        sqlImports.mergeWith(f.postgresManifest.imports);
                    });

                    return new RenderMap()
                        .add(
                            `accounts/${snakeCase(node.name)}.rs`,
                            render('accountsPage.njk', {
                                account: newNode,
                                imports: imports.toString(),
                                program: currentProgram,
                                discriminatorManifest,
                                typeManifest,
                            }),
                        )
                        .add(
                            `accounts/sql/${snakeCase(node.name)}_row.rs`,
                            render('sqlRowPage.njk', {
                                entityDocs: node.docs,
                                entityName: node.name,
                                isEnum: isNode(node.data, 'enumTypeNode'),
                                enumVariants: isNode(node.data, 'enumTypeNode')
                                    ? buildEnumVariantsInfo(node.name, node.data, flatFields)
                                    : [],
                                imports: sqlImports.toString(),
                                flatFields: flatFields.map(([f, type]) => f),
                            }),
                        );
                },

                visitDefinedType(node) {
                    const typeManifest = visit(node.type, typeManifestVisitor);
                    const imports = new ImportMap().mergeWithManifest(typeManifest).add('carbon_core::borsh');

                    const flatFields = flattenType(node.type, [], [], new Set());
                    const sqlImports = new ImportMap().add(`crate::types::${pascalCase(node.name)}`);

                    flatFields.forEach(([f, type]) => {
                        sqlImports.mergeWith(f.postgresManifest.imports);
                    });

                    return new RenderMap()
                        .add(
                            `types/${snakeCase(node.name)}.rs`,
                            render('typesPage.njk', {
                                definedType: node,
                                imports: imports.toString(),
                                typeManifest,
                            }),
                        )
                        .add(
                            `types/sql/${snakeCase(node.name)}_row.rs`,
                            render('sqlRowPage.njk', {
                                entityDocs: node.docs,
                                entityName: node.name,
                                isEnum: isNode(node.type, 'enumTypeNode'),
                                enumVariants: isNode(node.type, 'enumTypeNode')
                                    ? buildEnumVariantsInfo(pascalCase(node.name), node.type, flatFields)
                                    : [],
                                imports: sqlImports.toString(),
                                flatFields: flatFields.map(([f, type]) => f),
                            }),
                        );
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

                    const discriminatorManifest = getDiscriminatorManifest(discriminators);

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
                    const sqlImports = new ImportMap().add(`crate::instructions::${pascalCase(node.name)}`);
                    flatFields.forEach(([f, type]) => {
                        sqlImports.mergeWith(f.postgresManifest.imports);
                    });

                    return new RenderMap()
                        .add(
                            `instructions/${snakeCase(node.name)}.rs`,
                            render('instructionsPage.njk', {
                                argumentTypes,
                                imports: imports.toString(),
                                instruction: newNode,
                                discriminatorManifest,
                                program: currentProgram,
                            }),
                        )
                        .add(
                            `instructions/sql/${snakeCase(node.name)}_row.rs`,
                            render('sqlRowPage.njk', {
                                entityDocs: node.docs,
                                entityName: node.name,
                                isEnum: false,
                                imports: sqlImports.toString(),
                                flatFields: flatFields.map(([f, type]) => f),
                            }),
                        );
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
                    const programsToExport = getAllPrograms(node);
                    const accountsToExport = getAllAccounts(node);
                    const instructionsToExport = getAllInstructionsWithSubs(node, {
                        leavesOnly: !renderParentInstructions,
                    });
                    const definedTypesToExport = getAllDefinedTypes(node);

                    const ctx = {
                        accountsToExport,
                        definedTypesToExport,
                        instructionsToExport,
                        programsToExport,
                        root: node,
                    };

                    const map = new RenderMap();

                    // Generate mod files
                    if (accountsToExport.length > 0) {
                        map.add('accounts/mod.rs', render('accountsMod.njk', ctx));
                        map.add('accounts/sql/mod.rs', render('accountsSqlMod.njk', ctx));
                    }
                    if (instructionsToExport.length > 0) {
                        map.add('instructions/mod.rs', render('instructionsMod.njk', ctx));
                        map.add('instructions/sql/mod.rs', render('instructionsSqlMod.njk', ctx));
                    }
                    if (definedTypesToExport.length > 0) {
                        map.add('types/mod.rs', render('typesMod.njk', ctx));
                        map.add('types/sql/mod.rs', render('typesSqlMod.njk', ctx));
                    }

                    // Generate lib.rs
                    map.add('lib.rs', render('lib.njk', ctx));

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
        opts: { variant?: string; inOption?: boolean } = {},
    ): [FlattenedField, TypeNode][] {
        const out: [FlattenedField, TypeNode][] = [];
        const { variant, inOption } = opts;

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

        if (typeNode.kind == 'structTypeNode') {
            for (const field of typeNode.fields) {
                out.push(
                    ...flattenType(field.type, [...prefix, snakeCase(field.name)], field.docs ?? [], seen, {
                        variant,
                        inOption,
                    }),
                );
            }
            return out;
        }

        if (isNode(typeNode, 'enumTypeNode')) {
            // discriminant column
            const variantCol = makeName([...prefix, 'variant']);
            out.push([
                {
                    column: variantCol,
                    rustPath: [...prefix, 'variant'].join('.'),
                    rowType: 'i16',
                    optional: false,
                    docs: ['enum variant discriminant'],
                    postgresManifest: {
                        sqlxType: 'i16',
                        sqlColumnType: 'SMALLINT',
                        imports: new ImportMap(),
                    },
                },
                typeNode,
            ]);

            typeNode.variants.forEach((v: any) => {
                const vName = snakeCase(v.name);
                if (v.kind === 'enumStructVariantTypeNode') {
                    v.struct.fields.forEach((f: any) => {
                        out.push(
                            ...flattenType(f.type, [...prefix, vName, snakeCase(f.name)], f.docs ?? [], seen, {
                                variant: vName,
                            }),
                        );
                    });
                } else if (v.kind === 'enumTupleVariantTypeNode') {
                    v.tuple.items.forEach((it: any, i: number) => {
                        out.push(...flattenType(it, [...prefix, vName, `item_${i}`], [], seen, { variant: vName }));
                    });
                }
            });
            return out;
        }

        if (isNode(typeNode, 'tupleTypeNode')) {
            for (let i = 0; i < typeNode.items.length; i++) {
                out.push(...flattenType(typeNode.items[i], [...prefix, `item_${i}`], [], seen, { variant, inOption }));
            }

            return out;
        }

        if (isNode(typeNode, 'optionTypeNode')) {
            const inner = visit(typeNode.item, postgresTypeManifestVisitor) as PostgresTypeManifest;
            const rowType = variant ? `Option<Option<${inner.sqlxType}>>` : `Option<${inner.sqlxType}>`;

            const expr = buildExpression(typeNode, `source.${prefix.join('.')}`);

            out.push([
                {
                    column: makeName(prefix),
                    rustPath: prefix.join('.'),
                    rowType,
                    optional: true,
                    docs: docsPrefix,
                    postgresManifest: inner,
                    variant,
                    accessorVariant: variant ? prefix.slice(1).join('.') : undefined,
                    expr,
                },
                typeNode,
            ]);
            return out;
        }

        const manifest = visit(typeNode, postgresTypeManifestVisitor) as PostgresTypeManifest;
        const column = makeName(prefix);

        const field: FlattenedField = {
            column,
            rustPath: prefix.join('.'),
            rowType: variant ? `Option<${manifest.sqlxType}>` : manifest.sqlxType,
            optional: !!variant,
            docs: docsPrefix,
            postgresManifest: manifest,
            variant,
            accessorVariant: variant
                ? prefix.slice(1).join('.') // after <variant>
                : undefined,
        };

        field.expr = buildExpression(typeNode, `source.${field.rustPath}`);

        out.push([field, typeNode]);
        return out;
    }

    function buildExpression(typeNode: TypeNode, prefix: string): string {
        if (isNode(typeNode, 'arrayTypeNode')) {
            if (isNode(typeNode.item, 'numberTypeNode') || isNode(typeNode.item, 'booleanTypeNode')) {
                return `${prefix}.into_iter().map(|element| element.into()).collect()`;
            } else {
                return `sqlx::types::Json(${prefix}.into_iter().map(|element| ${buildExpression(typeNode.item, `element`)}).collect())`;
            }
        } else if (isNode(typeNode, 'optionTypeNode')) {
            return `${prefix}.map(|value| ${buildExpression(typeNode.item, `value`)})`;
        } else if (isNode(typeNode, 'tupleTypeNode')) {
            return `(${typeNode.items.map((item, i) => buildExpression(item, `${prefix}.${i}`)).join(', ')})`;
        } else {
            return `${prefix}.into()`;
        }
    }

    function buildEnumVariantsInfo(
        entityPascal: string,
        enumNode: EnumTypeNode,
        flatFields: [FlattenedField, TypeNode][],
    ) {
        const byVariant: Record<string, [FlattenedField, TypeNode][]> = {};
        flatFields.forEach(([f, type]) => {
            if (!f.variant) return;
            (byVariant[f.variant] ??= []).push([f, type]);
        });

        return enumNode.variants.map((variant, idx: number) => {
            const vSnake = snakeCase(variant.name);
            let pattern = `${entityPascal}::${pascalCase(variant.name)}`;
            if (variant.kind === 'enumStructVariantTypeNode' && variant.struct.kind == 'structTypeNode') {
                const vars = variant.struct.fields.map((f: any) => snakeCase(f.name)).join(', ');
                pattern += ` { ${vars} }`;
            } else if (variant.kind === 'enumTupleVariantTypeNode' && variant.tuple.kind == 'tupleTypeNode') {
                const vars = variant.tuple.items.map((_: any, i: number) => `item_${i}`).join(', ');
                pattern += `(${vars})`;
            }

            const assignments = (byVariant[vSnake] || []).map(([f, type]) => ({
                column: f.column,
                expr: buildExpression(type, `${f.accessorVariant}`),
            }));

            return { pattern, discriminant: idx, assignments };
        });
    }
}
