import { isNode, resolveNestedTypeNode, snakeCase, SnakeCaseString, TypeNode } from '@codama/nodes';
import { visit } from '@codama/visitors-core';
import { PostgresTypeManifest } from './getPostgresTypeManifestVisitor';
import { isCopyType, isPostgresPrimitiveType, typesMatch } from './utils/postgresHelpers';

export type FlattenedField = {
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

/**
 * Mutable state shared between the visitor lifecycle and the row mapper.
 * Uses getter functions so the mapper always sees the latest assignments
 * (both `typeManifestVisitor` and `definedTypesMap` are reassigned in visitRoot).
 */
export type MapperContext = {
    getTypeManifestVisitor: () => any;
    postgresTypeManifestVisitor: any;
    newtypeWrapperTypes: Set<string>;
    getDefinedTypesMap: () => Map<string, any> | null;
};

/**
 * Maps Codama type nodes to flat Postgres row field descriptors and the
 * Rust expression strings that convert between Rust struct fields and their
 * Postgres column representations.
 *
 * All methods access visitor state via MapperContext getters so they
 * always use the current visitor instances after visitRoot reinitialises them.
 */
export class PostgresRowMapper {
    constructor(private ctx: MapperContext) {}

    private get tmv() {
        return this.ctx.getTypeManifestVisitor();
    }
    private get ptmv() {
        return this.ctx.postgresTypeManifestVisitor;
    }

    flattenType(
        typeNode: TypeNode,
        prefix: string[],
        docsPrefix: string[],
        seen: Set<string>,
        opts: { inOption?: boolean } = {},
    ): FlattenedField[] {
        const out: FlattenedField[] = [];
        const { inOption } = opts;

        const makeName = (nameParts: string[]): string => {
            let col = snakeCase(nameParts.join('_'));
            if (seen.has(col)) {
                let i = 1;
                while (seen.has(`${col}_${i}`)) i++;
                col = `${col}_${i}` as SnakeCaseString;
            }
            seen.add(col);
            return col;
        };

        const flattenOptionType = (
            optNode: TypeNode,
            optPrefix: string[],
            optDocsPrefix: string[],
        ): FlattenedField[] => {
            const column = makeName(optPrefix);
            const itemType = isNode(optNode, 'optionTypeNode')
                ? optNode.item
                : isNode(optNode, 'zeroableOptionTypeNode')
                  ? optNode.item
                  : (optNode as any).item; // remainderOptionTypeNode
            const manifest = visit(itemType, this.ptmv) as PostgresTypeManifest;
            const isJson = (manifest.postgresColumnType || '').toUpperCase().startsWith('JSONB');

            const rowType = isJson
                ? manifest.sqlxType.includes('Json<')
                    ? `Option<${manifest.sqlxType}>`
                    : `Option<sqlx::types::Json<${manifest.sqlxType}>>`
                : `Option<${manifest.sqlxType}>`;

            const innerRustManifest = visit(itemType, this.tmv) as any;
            const needsConversion = isNode(itemType, 'publicKeyTypeNode')
                ? true
                : !typesMatch(manifest.sqlxType, innerRustManifest.type);

            const sourceField = `source.${optPrefix.join('.')}`;
            const sourceColumn = `source.${column}`;
            const needsSpecialHandling = isNode(itemType, 'arrayTypeNode') || isNode(itemType, 'tupleTypeNode');

            let expr: string;
            if (needsSpecialHandling) {
                const innerExpr = this.buildExpression(itemType, 'value');
                expr = innerExpr === 'value' ? sourceField : `${sourceField}.map(|value| ${innerExpr})`;
            } else {
                expr = isJson
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
            }

            let reverseExpr: string;
            if (needsSpecialHandling) {
                const innerReverseExpr = this.buildReverse(itemType, 'value');
                if (innerReverseExpr.includes('?')) {
                    const exprWithoutQuestion = innerReverseExpr.replace(/\?$/, '');
                    reverseExpr = `${sourceColumn}.map(|value| ${exprWithoutQuestion}).transpose()?`;
                } else if (innerReverseExpr === 'value' || innerReverseExpr === 'value.0') {
                    reverseExpr = isJson ? `${sourceColumn}.map(|value| value.0)` : sourceColumn;
                } else {
                    reverseExpr = `${sourceColumn}.map(|value| ${innerReverseExpr})`;
                }
            } else {
                reverseExpr = isJson
                    ? `${sourceColumn}.map(|value| value.0)`
                    : this.buildReverseOptionType(optNode, sourceColumn, manifest);
            }

            return [
                {
                    column,
                    rustPath: optPrefix.join('.'),
                    rowType,
                    postgresColumnType: `${manifest.postgresColumnType}`,
                    docs: optDocsPrefix,
                    postgresManifest: manifest,
                    expr,
                    reverseExpr,
                    isCopyType: isCopyType(rowType),
                },
            ];
        };

        if (isNode(typeNode, 'structTypeNode')) {
            for (const field of typeNode.fields) {
                out.push(...this.flattenType(field.type, [...prefix, snakeCase(field.name)], [], seen, { inOption }));
            }
            return out;
        }

        if (isNode(typeNode, 'optionTypeNode')) {
            return flattenOptionType(typeNode, prefix, docsPrefix);
        }

        if (isNode(typeNode, 'zeroableOptionTypeNode') || isNode(typeNode, 'remainderOptionTypeNode')) {
            return flattenOptionType(typeNode, prefix, docsPrefix);
        }

        if (isNode(typeNode, 'hiddenPrefixTypeNode')) {
            return this.flattenType(typeNode.type, prefix, docsPrefix, seen, opts);
        }

        if (isNode(typeNode, 'definedTypeLinkNode')) {
            const column = makeName(prefix);
            const manifest = visit(typeNode, this.ptmv) as PostgresTypeManifest;
            const isJson = (manifest.postgresColumnType || '').toUpperCase().startsWith('JSONB');
            const typeManifest = visit(typeNode, this.tmv) as any;
            const originalTypeName = typeManifest.type;

            const rowType = isJson ? `sqlx::types::Json<${manifest.sqlxType}>` : `${manifest.sqlxType}`;
            const needsConversion = manifest.sqlxType !== originalTypeName;
            const sourceField = `source.${prefix.join('.')}`;
            const expr = isJson
                ? `sqlx::types::Json(${needsConversion ? `${sourceField}.into()` : sourceField})`
                : needsConversion
                  ? `${sourceField}.into()`
                  : sourceField;

            const sourceColumn = `source.${column}`;
            const reverseExpr = isJson
                ? `${sourceColumn}.0`
                : needsConversion
                  ? `${sourceColumn}.into()`
                  : sourceColumn;

            let needsBigArray = false;
            if (!this.ctx.newtypeWrapperTypes.has(typeNode.name)) {
                let resolvedRaw: any = undefined;
                const dm = this.ctx.getDefinedTypesMap();
                if (dm) {
                    const definedType = dm.get(typeNode.name);
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

        const manifest = visit(typeNode, this.ptmv) as PostgresTypeManifest;
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

        field.expr = this.buildExpression(typeNode, `source.${field.rustPath}`);
        field.reverseExpr = this.buildReverse(typeNode, `source.${field.rustPath}`);

        out.push(field);
        return out;
    }

    buildExpression(typeNode: TypeNode, prefix: string): string {
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

                const postgresManifest = visit(typeNode.item, this.ptmv) as PostgresTypeManifest;
                const rustManifest = visit(typeNode.item, this.tmv) as any;
                const postgresType = postgresManifest.sqlxType;
                const rustType = rustManifest.type;
                const isPostgresWrapper = isPostgresPrimitiveType(postgresType);

                if (postgresType === rustType && !isPostgresWrapper) {
                    return `${prefix}.to_vec()`;
                }
                return `${prefix}.into_iter().map(|element| element.into()).collect()`;
            } else {
                const innerExpr = this.buildExpression(typeNode.item, `element`);
                if (innerExpr === 'element') {
                    return `sqlx::types::Json(${prefix}.to_vec())`;
                }
                return `sqlx::types::Json(${prefix}.into_iter().map(|element| ${innerExpr}).collect())`;
            }
        } else if (isNode(typeNode, 'fixedSizeTypeNode')) {
            if (isNode(typeNode.type, 'bytesTypeNode')) {
                return `${prefix}.to_vec()`;
            }
            return this.buildExpression(typeNode.type, prefix);
        } else if (
            isNode(typeNode, 'optionTypeNode') ||
            isNode(typeNode, 'zeroableOptionTypeNode') ||
            isNode(typeNode, 'remainderOptionTypeNode')
        ) {
            return `${prefix}.map(|value| ${this.buildExpression(typeNode.item, `value`)})`;
        } else if (isNode(typeNode, 'hiddenPrefixTypeNode')) {
            return this.buildExpression(typeNode.type, prefix);
        } else if (isNode(typeNode, 'tupleTypeNode')) {
            if (typeNode.items.length === 1) {
                return `${this.buildExpression(typeNode.items[0], `${prefix}`)}`;
            }
            return `sqlx::types::Json(serde_json::to_value(${prefix}).unwrap())`;
        } else if (isNode(typeNode, 'publicKeyTypeNode')) {
            return `${prefix}.into()`;
        } else if (isNode(typeNode, 'booleanTypeNode') || isNode(typeNode, 'stringTypeNode')) {
            return prefix;
        } else {
            try {
                const postgresManifest = visit(typeNode, this.ptmv) as PostgresTypeManifest;
                const rustManifest = visit(typeNode, this.tmv) as any;
                if (typesMatch(postgresManifest.sqlxType, rustManifest.type)) {
                    return prefix;
                }
            } catch (e) {
                // If we can't get the manifests, fall back to .into()
            }
            return `${prefix}.into()`;
        }
    }

    buildReverseOptionType(typeNode: TypeNode, prefix: string, manifest: PostgresTypeManifest): string {
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
            const rustManifest = visit(innerType, this.tmv) as any;
            const rustType = rustManifest.type;
            const postgresType = manifest.sqlxType;

            if (typesMatch(postgresType, rustType)) {
                return prefix;
            }

            const isPostgresPrimitive = isPostgresPrimitiveType(manifest.sqlxType);

            if (isPostgresPrimitive) {
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
            return `${prefix}.map(|value| *value)`;
        } else if (isNode(innerType, 'stringTypeNode') || isNode(innerType, 'bytesTypeNode')) {
            return `${prefix}.map(|value| *value)`;
        } else {
            const rustManifest = visit(innerType, this.tmv) as any;
            if (typesMatch(manifest.sqlxType, rustManifest.type)) {
                return prefix;
            }
            return `${prefix}.map(|value| value.into())`;
        }
    }

    buildReverse(typeNode: TypeNode, prefix: string): string {
        if (isNode(typeNode, 'arrayTypeNode')) {
            const isJson = !(
                isNode(typeNode.item, 'numberTypeNode') ||
                isNode(typeNode.item, 'booleanTypeNode') ||
                isNode(typeNode.item, 'bytesTypeNode') ||
                isNode(typeNode.item, 'stringTypeNode') ||
                isNode(typeNode.item, 'publicKeyTypeNode')
            );

            switch (typeNode.count.kind) {
                case 'fixedCountNode':
                    if (isJson) {
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
                        const innerReverse = this.buildReverse(typeNode.item, 'element');
                        if (innerReverse.includes('?')) {
                            const innerWithoutQuestion = innerReverse.replace(/\?$/, '');
                            return (
                                `${prefix}.0.into_iter().map(|element| ${innerWithoutQuestion}).collect::<Result<Vec<_>, carbon_core::error::Error>>()?` +
                                `.try_into().map_err(|_| carbon_core::error::Error::Custom("Failed to convert value from postgres primitive".to_string()))?`
                            );
                        } else {
                            const mapExpr = innerReverse === 'element' ? 'Ok' : `|element| Ok(${innerReverse})`;
                            return (
                                `${prefix}.0.into_iter().map(${mapExpr}).collect::<Result<Vec<_>, carbon_core::error::Error>>()` +
                                `.map_err(|_| carbon_core::error::Error::Custom("Failed to collect array elements".to_string()))?` +
                                `.try_into().map_err(|_| carbon_core::error::Error::Custom("Failed to convert value from postgres primitive".to_string()))?`
                            );
                        }
                    } else {
                        const innerReverse = this.buildReverse(typeNode.item, 'element');
                        if (innerReverse.includes('?')) {
                            const innerWithoutQuestion = innerReverse.replace(/\?$/, '');
                            return `${prefix}.into_iter().map(|element| ${innerWithoutQuestion}).collect::<Result<Vec<_>, carbon_core::error::Error>>()?.try_into().map_err(|_| carbon_core::error::Error::Custom("Failed to convert array element to primitive".to_string()))?`;
                        } else {
                            const mapExpr = innerReverse === 'element' ? 'Ok' : `|element| Ok(${innerReverse})`;
                            return `${prefix}.into_iter().map(${mapExpr}).collect::<Result<Vec<_>, carbon_core::error::Error>>().map_err(|_| carbon_core::error::Error::Custom("Failed to collect array elements".to_string()))?.try_into().map_err(|_| carbon_core::error::Error::Custom("Failed to convert array element to primitive".to_string()))?`;
                        }
                    }
                    break;
                case 'prefixedCountNode':
                    if (isJson) {
                        if (
                            isNode(typeNode.item, 'definedTypeLinkNode') ||
                            isNode(typeNode.item, 'structTypeNode') ||
                            isNode(typeNode.item, 'enumTypeNode')
                        ) {
                            return `${prefix}.0`;
                        }
                        const innerReverse = this.buildReverse(typeNode.item, 'element');
                        if (innerReverse === 'element' || innerReverse === '*element') {
                            return `${prefix}.0`;
                        }
                        return `${prefix}.0.into_iter().map(|element| ${innerReverse}).collect()`;
                    } else {
                        if (isNode(typeNode.item, 'publicKeyTypeNode')) {
                            return `${prefix}.into_iter().map(|element| *element).collect()`;
                        }

                        const postgresManifest = visit(typeNode.item, this.ptmv) as PostgresTypeManifest;
                        const rustManifest = visit(typeNode.item, this.tmv) as any;

                        if (typesMatch(postgresManifest.sqlxType, rustManifest.type)) {
                            return `${prefix}.to_vec()`;
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
                        const innerReverse = this.buildReverse(typeNode.item, 'element');
                        if (
                            (innerReverse === 'element' || innerReverse === '*element') &&
                            !isNode(typeNode.item, 'publicKeyTypeNode')
                        ) {
                            return `${prefix}.0`;
                        }
                        return `${prefix}.0.into_iter().map(|element| ${innerReverse}).collect()`;
                    } else {
                        if (isNode(typeNode.item, 'publicKeyTypeNode')) {
                            return `${prefix}.into_iter().map(|element| *element).collect()`;
                        }
                        const postgresManifest = visit(typeNode.item, this.ptmv) as PostgresTypeManifest;
                        const rustManifest = visit(typeNode.item, this.tmv) as any;

                        if (typesMatch(postgresManifest.sqlxType, rustManifest.type)) {
                            return `${prefix}.to_vec()`;
                        }
                        return `${prefix}.into_iter().map(|element| element.try_into()).collect::<Result<_, _>>().map_err(|_| carbon_core::error::Error::Custom("Failed to convert array element to primitive".to_string()))?`;
                    }
                    break;
            }
        }
        if (isNode(typeNode, 'fixedSizeTypeNode')) {
            if (isNode(typeNode.type, 'bytesTypeNode')) {
                return `${prefix}.as_slice().try_into().map_err(|_| carbon_core::error::Error::Custom("Failed to convert padding from postgres primitive: expected ${typeNode.size} bytes".to_string()))?`;
            }
            return this.buildReverse(typeNode.type, prefix);
        }
        if (isNode(typeNode, 'optionTypeNode')) {
            const innerReverse = this.buildReverse(typeNode.item, 'value');
            if (innerReverse.includes('?')) {
                const innerWithoutQuestion = innerReverse.replace(/\?$/, '');
                return `${prefix}.map(|value| ${innerWithoutQuestion}).transpose()?`;
            }
            if (innerReverse === 'value' || innerReverse === '*value') {
                return prefix;
            }
            return `${prefix}.map(|value| ${innerReverse})`;
        }
        if (isNode(typeNode, 'tupleTypeNode')) {
            if (typeNode.items.length === 1) {
                return `${this.buildReverse(typeNode.items[0], `${prefix}`)}`;
            }
            return `serde_json::from_value(${prefix}.0).map_err(|_| carbon_core::error::Error::Custom("Failed to deserialize tuple from JSON".to_string()))?`;
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
            const postgresManifest = visit(typeNode, this.ptmv) as PostgresTypeManifest;
            const rustManifest = visit(typeNode, this.tmv) as any;
            const postgresType = postgresManifest.sqlxType;
            const rustType = rustManifest.type;

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

        try {
            const postgresManifest = visit(typeNode, this.ptmv) as PostgresTypeManifest;
            const rustManifest = visit(typeNode, this.tmv) as any;
            if (typesMatch(postgresManifest.sqlxType, rustManifest.type)) {
                return prefix;
            }
        } catch (e) {
            // If we can't get manifests, fall back to .into()
        }

        return `${prefix}.into()`;
    }
}
