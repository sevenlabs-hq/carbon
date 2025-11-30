import { TypeNode, isNode } from '@codama/nodes';
import { visit } from '@codama/visitors-core';
import { getPostgresTypeManifestVisitor, PostgresTypeManifest } from '../getPostgresTypeManifestVisitor';
/*
GraphQL conversion rules (Rust → GraphQL values):

- PublicKey → Pubkey scalar (string form)
- Numbers → custom scalars for large/signed (U8,U32,I64,U64,I128,U128) or native i32 where safe
- Options → Option<...>
- Arrays → Vec<...> (fixed-size rendered as Vec in GraphQL)
  - Nested arrays from Postgres Json: unwrap outer `.0`, then map inner elements recursively
- Maps/Tuples/Sets → Json scalar (lossless container)
- Defined types → <TypeName>GraphQL with Into conversions

Postgres → GraphQL specifics:
- sqlx::types::Json<T> unwrap with `.0` once; do not unwrap elements
- For fixed-size conversions elsewhere we collect Result<Vec<_>, _> before try_into in Postgres code (see getRenderMapVisitor)
*/

function handleBooleanTypeConversion(fieldAccess: string, isJson: boolean, isOption: boolean): string {
    if (isOption && isJson) {
        return `${fieldAccess}.map(|v| v.0)`;
    } else if (isJson) {
        return `${fieldAccess}.0`;
    } else {
        return fieldAccess;
    }
}

function isBooleanTypeAlias(typeName: string): boolean {
    return typeName.toLowerCase().includes('bool');
}

function requiresClosureForPrimitive(funcRef: string, innerExpr: string, param: string): boolean {
    return innerExpr.includes(`${param}.`);
}

function shortenFunctionReference(funcRef: string): string {
    const graphqlPrimitives = ['Pubkey', 'U8', 'U32', 'I64', 'U64', 'I128', 'U128'];
    for (const primitive of graphqlPrimitives) {
        if (funcRef.endsWith(`::${primitive}`) || funcRef === primitive) {
            return primitive;
        }
    }
    return funcRef;
}

function buildOptionMapExpression(fieldAccess: string, innerExpr: string, param: string = 'v'): string {
    if (innerExpr === param) {
        return fieldAccess;
    }
    if (innerExpr.includes(`*${param}`)) {
        return `${fieldAccess}.map(|${param}| ${innerExpr})`;
    }

    const funcRef = extractFunctionReference(innerExpr, param);
    if (funcRef) {
        if (requiresClosureForPrimitive(funcRef, innerExpr, param)) {
            return `${fieldAccess}.map(|${param}| ${innerExpr})`;
        }
        const shortenedRef = shortenFunctionReference(funcRef);
        return `${fieldAccess}.map(${shortenedRef})`;
    }

    return `${fieldAccess}.map(|${param}| ${innerExpr})`;
}

function buildArrayMapExpression(fieldAccess: string, innerExpr: string, param: string = 'item'): string {
    if (innerExpr === param) {
        return `${fieldAccess}.to_vec()`;
    }
    if (innerExpr.includes(`*${param}`) || innerExpr.includes('*inner_item')) {
        return `${fieldAccess}.into_iter().map(|${param}| ${innerExpr.replace(/inner_item/g, param)}).collect()`;
    }

    const funcRef = extractFunctionReference(innerExpr, param);
    if (funcRef) {
        if (requiresClosureForPrimitive(funcRef, innerExpr, param)) {
            return `${fieldAccess}.into_iter().map(|${param}| ${innerExpr.replace(/inner_item/g, param)}).collect()`;
        }
        const shortenedRef = shortenFunctionReference(funcRef);
        return `${fieldAccess}.into_iter().map(${shortenedRef}).collect()`;
    }

    return `${fieldAccess}.into_iter().map(|${param}| ${innerExpr.replace(/inner_item/g, param)}).collect()`;
}

function extractFunctionReference(expr: string, param: string): string | null {
    const trimmed = expr.trim();
    const escapedParam = param.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

    const exactMatchPattern = `^([a-zA-Z_][a-zA-Z0-9_]*(?:::[a-zA-Z_][a-zA-Z0-9_]*)+)\\(${escapedParam}\\)$`;
    const exactMatch = new RegExp(exactMatchPattern);
    const match = trimmed.match(exactMatch);
    if (match) {
        return match[1];
    }

    const simpleMatchPattern = `^([a-zA-Z_][a-zA-Z0-9_]*)\\(${escapedParam}\\)$`;
    const simpleMatch = new RegExp(simpleMatchPattern);
    const simpleMatchResult = trimmed.match(simpleMatch);
    if (simpleMatchResult) {
        return simpleMatchResult[1];
    }

    const derefMatchPattern = `^([a-zA-Z_][a-zA-Z0-9_]*(?:::[a-zA-Z_][a-zA-Z0-9_]*)+)\\(\\\\*${escapedParam}\\)$`;
    const derefMatch = new RegExp(derefMatchPattern);
    const derefMatchResult = trimmed.match(derefMatch);
    if (derefMatchResult) {
        return derefMatchResult[1];
    }

    const fieldMatchPattern = `^([a-zA-Z_][a-zA-Z0-9_]*(?:::[a-zA-Z_][a-zA-Z0-9_]*)+)\\(${escapedParam}\\.[a-zA-Z0-9_]+\\)$`;
    const fieldMatch = new RegExp(fieldMatchPattern);
    const fieldMatchResult = trimmed.match(fieldMatch);
    if (fieldMatchResult) {
        return fieldMatchResult[1];
    }

    return null;
}

export function buildConversionFromOriginal(typeNode: TypeNode, fieldAccess: string): string {
    if (isNode(typeNode, 'publicKeyTypeNode')) {
        return `carbon_core::graphql::primitives::Pubkey(${fieldAccess})`;
    }

    if (isNode(typeNode, 'bytesTypeNode')) {
        return `${fieldAccess}.into_iter().map(carbon_core::graphql::primitives::U8).collect()`;
    }

    if (isNode(typeNode, 'amountTypeNode')) {
        return buildConversionFromOriginal(typeNode.number, fieldAccess);
    }

    if (isNode(typeNode, 'numberTypeNode')) {
        switch (typeNode.format) {
            case 'u8':
                return `carbon_core::graphql::primitives::U8(${fieldAccess})`;
            case 'u16':
            case 'i8':
            case 'i16':
                return `${fieldAccess} as i32`;
            case 'u32':
                return `carbon_core::graphql::primitives::U32(${fieldAccess})`;
            case 'i64':
                return `carbon_core::graphql::primitives::I64(${fieldAccess})`;
            case 'u64':
                return `carbon_core::graphql::primitives::U64(${fieldAccess})`;
            case 'i128':
                return `carbon_core::graphql::primitives::I128(${fieldAccess})`;
            case 'u128':
                return `carbon_core::graphql::primitives::U128(${fieldAccess})`;
            case 'f32':
                return `${fieldAccess} as f64`;
            case 'f64':
                return `${fieldAccess}`;
        }
    }

    if (isNode(typeNode, 'optionTypeNode')) {
        const innerExpr = buildConversionFromOriginal(typeNode.item, 'v');
        return buildOptionMapExpression(fieldAccess, innerExpr, 'v');
    }

    if (isNode(typeNode, 'arrayTypeNode')) {
        if (isNode(typeNode.item, 'numberTypeNode') && typeNode.item.format === 'u8') {
            return `${fieldAccess}.into_iter().map(carbon_core::graphql::primitives::U8).collect()`;
        }

        const innerExpr = buildConversionFromOriginal(typeNode.item, 'item');
        const funcRef = extractFunctionReference(innerExpr, 'item');
        if (funcRef) {
            return `${fieldAccess}.into_iter().map(${funcRef}).collect()`;
        }
        if (innerExpr === 'item') {
            return `${fieldAccess}.to_vec()`;
        }
        return `${fieldAccess}.into_iter().map(|item| ${innerExpr}).collect()`;
    }

    if (isNode(typeNode, 'fixedSizeTypeNode')) {
        if (isNode(typeNode.type, 'bytesTypeNode')) {
            return `${fieldAccess}.into_iter().map(carbon_core::graphql::primitives::U8).collect()`;
        }
        const innerExpr = buildConversionFromOriginal(typeNode.type, 'item');
        const funcRef = extractFunctionReference(innerExpr, 'item');
        if (funcRef) {
            return `${fieldAccess}.into_iter().map(${funcRef}).collect()`;
        }
        if (innerExpr === 'item') {
            return `${fieldAccess}.to_vec()`;
        }
        return `${fieldAccess}.into_iter().map(|item| ${innerExpr}).collect()`;
    }

    if (isNode(typeNode, 'sizePrefixTypeNode')) {
        return buildConversionFromOriginal(typeNode.type, fieldAccess);
    }

    if (isNode(typeNode, 'remainderOptionTypeNode')) {
        const innerExpr = buildConversionFromOriginal(typeNode.item, 'v');
        return buildOptionMapExpression(fieldAccess, innerExpr, 'v');
    }

    if (isNode(typeNode, 'zeroableOptionTypeNode')) {
        const innerExpr = buildConversionFromOriginal(typeNode.item, 'v');
        const convertedExpr = innerExpr === 'v' ? 'v.into()' : innerExpr;
        if (convertedExpr !== 'v.into()') {
            return buildOptionMapExpression(fieldAccess, convertedExpr, 'v');
        }
        return `${fieldAccess}.map(|v| ${convertedExpr})`;
    }

    if (isNode(typeNode, 'hiddenPrefixTypeNode')) {
        return buildConversionFromOriginal(typeNode.type, fieldAccess);
    }

    if (isNode(typeNode, 'booleanTypeNode')) {
        return fieldAccess;
    }

    if (isNode(typeNode, 'definedTypeLinkNode')) {
        const typeName = typeNode.name.toLowerCase();
        if (isBooleanTypeAlias(typeName)) {
            return fieldAccess;
        }
        if (typeName.includes('decrypt') || typeName.includes('cipher') || typeName.includes('elgamal')) {
            return `${fieldAccess}.0.into_iter().map(carbon_core::graphql::primitives::U8).collect()`;
        }
        return `${fieldAccess}.into()`;
    }

    if (isNode(typeNode, 'tupleTypeNode') || isNode(typeNode, 'mapTypeNode') || isNode(typeNode, 'setTypeNode')) {
        return `carbon_core::graphql::primitives::Json(serde_json::to_value(&${fieldAccess}).unwrap_or(serde_json::Value::Null))`;
    }

    return fieldAccess;
}

export function buildConversionFromPostgresRow(typeNode: TypeNode, fieldAccess: string): string {
    if (isNode(typeNode, 'publicKeyTypeNode')) {
        return `carbon_core::graphql::primitives::Pubkey(${fieldAccess}.0)`;
    }

    if (isNode(typeNode, 'bytesTypeNode')) {
        return `${fieldAccess}.into_iter().map(carbon_core::graphql::primitives::U8).collect()`;
    }

    if (isNode(typeNode, 'amountTypeNode')) {
        return buildConversionFromPostgresRow(typeNode.number, fieldAccess);
    }

    if (isNode(typeNode, 'numberTypeNode')) {
        const postgresManifest = visit(typeNode, getPostgresTypeManifestVisitor()) as PostgresTypeManifest;
        const postgresType = postgresManifest.sqlxType;

        switch (typeNode.format) {
            case 'u8': {
                if (postgresType === 'u8') {
                    return `carbon_core::graphql::primitives::U8(*${fieldAccess})`;
                }
                return `carbon_core::graphql::primitives::U8((*${fieldAccess}) as u8)`;
            }
            case 'u16': {
                if (postgresType === 'U16') {
                    return `*${fieldAccess}`;
                }
                if (postgresType === 'i32') {
                    return fieldAccess;
                }
                return `*${fieldAccess}`;
            }
            case 'i8':
            case 'i16': {
                if (postgresType === 'i32') {
                    return `${fieldAccess} as i32`;
                }
                if (postgresType === 'i8' || postgresType === 'i16') {
                    return `${fieldAccess} as i32`;
                }
                return `(*${fieldAccess}) as i32`;
            }
            case 'i32': {
                if (postgresType === 'i32') {
                    return fieldAccess;
                }
                return fieldAccess;
            }
            case 'u32': {
                if (postgresType.includes('u32') && !postgresType.includes('U32')) {
                    return `carbon_core::graphql::primitives::U32(*${fieldAccess})`;
                }
                return `carbon_core::graphql::primitives::U32((*${fieldAccess}) as u32)`;
            }
            case 'i64':
                return `carbon_core::graphql::primitives::I64(${fieldAccess})`;
            case 'u64':
                return `carbon_core::graphql::primitives::U64(*${fieldAccess})`;
            case 'i128':
                return `carbon_core::graphql::primitives::I128(*${fieldAccess})`;
            case 'u128':
                return `carbon_core::graphql::primitives::U128(*${fieldAccess})`;
            case 'f32': {
                if (postgresType.includes('f64')) {
                    return fieldAccess;
                }
                return `${fieldAccess} as f64`;
            }
            case 'f64':
                return `${fieldAccess}`;
        }
    }

    if (isNode(typeNode, 'optionTypeNode')) {
        const postgresManifest = visit(typeNode.item, getPostgresTypeManifestVisitor()) as PostgresTypeManifest;
        const isJson = (postgresManifest.postgresColumnType || '').toUpperCase().startsWith('JSONB');

        if (isNode(typeNode.item, 'arrayTypeNode') && isNode(typeNode.item.item, 'definedTypeLinkNode')) {
            const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'v.0.clone()');
            return `${fieldAccess}.map(|v| ${innerExpr})`;
        }
        if (isNode(typeNode.item, 'booleanTypeNode')) {
            return handleBooleanTypeConversion(fieldAccess, isJson, true);
        }
        const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'v');
        return buildOptionMapExpression(fieldAccess, innerExpr, 'v');
    }

    if (isNode(typeNode, 'arrayTypeNode')) {
        // Special case: Vec<u8> should be treated like bytes
        if (isNode(typeNode.item, 'numberTypeNode') && typeNode.item.format === 'u8') {
            return `${fieldAccess}.into_iter().map(carbon_core::graphql::primitives::U8).collect()`;
        }

        if (isNode(typeNode.item, 'arrayTypeNode')) {
            const innerItemType = typeNode.item.item;
            let innerItemExpr: string;
            if (isNode(innerItemType, 'numberTypeNode')) {
                const postgresManifest = visit(innerItemType, getPostgresTypeManifestVisitor()) as PostgresTypeManifest;
                const postgresType = postgresManifest.sqlxType;
                switch (innerItemType.format) {
                    case 'u64':
                        innerItemExpr = `carbon_core::graphql::primitives::U64(*item)`;
                        break;
                    case 'u128':
                        innerItemExpr = `carbon_core::graphql::primitives::U128(*item)`;
                        break;
                    case 'i128':
                        innerItemExpr = `carbon_core::graphql::primitives::I128(*item)`;
                        break;
                    case 'i64':
                        innerItemExpr = `carbon_core::graphql::primitives::I64(item)`;
                        break;
                    default:
                        innerItemExpr = buildConversionFromPostgresRow(innerItemType, 'item');
                }
            } else if (isNode(innerItemType, 'publicKeyTypeNode')) {
                innerItemExpr = `carbon_core::graphql::primitives::Pubkey(item.0)`;
            } else {
                const tempExpr = buildConversionFromPostgresRow(innerItemType, 'item');
                const funcRef = extractFunctionReference(tempExpr, 'item');
                if (funcRef) {
                    if (requiresClosureForPrimitive(funcRef, tempExpr, 'item') || tempExpr.includes('*item')) {
                        innerItemExpr = `${funcRef}(*item)`;
                    } else {
                        innerItemExpr = tempExpr;
                    }
                } else if (tempExpr.includes('*item')) {
                    innerItemExpr = tempExpr;
                } else {
                    innerItemExpr = tempExpr;
                }
            }
            return `${fieldAccess}.0.into_iter().map(|item| item.into_iter().map(|item| ${innerItemExpr}).collect()).collect()`;
        }
        if (isNode(typeNode.item, 'definedTypeLinkNode')) {
            if (fieldAccess.includes('.0')) {
                return `${fieldAccess}.as_ref().clone().into_iter().map(|item| item.into()).collect()`;
            }
            return `${fieldAccess}.0.into_iter().map(|item| item.into()).collect()`;
        }
        if (
            isNode(typeNode.item, 'numberTypeNode') ||
            isNode(typeNode.item, 'booleanTypeNode') ||
            isNode(typeNode.item, 'bytesTypeNode') ||
            isNode(typeNode.item, 'stringTypeNode') ||
            isNode(typeNode.item, 'publicKeyTypeNode')
        ) {
            const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'item');
            if (innerExpr === 'item' && !isNode(typeNode.item, 'publicKeyTypeNode')) {
                return `${fieldAccess}.to_vec()`;
            }
            return buildArrayMapExpression(fieldAccess, innerExpr, 'item');
        }
    }

    if (isNode(typeNode, 'fixedSizeTypeNode')) {
        if (isNode(typeNode.type, 'bytesTypeNode')) {
            return `${fieldAccess}.into_iter().map(carbon_core::graphql::primitives::U8).collect()`;
        }
        const innerExpr = buildConversionFromPostgresRow(typeNode.type, 'item');
        if ((innerExpr === 'item' || innerExpr === '*item') && !isNode(typeNode.type, 'publicKeyTypeNode')) {
            return `${fieldAccess}.to_vec()`;
        }
        return `${fieldAccess}.into_iter().map(|item| ${innerExpr}).collect()`;
    }

    if (isNode(typeNode, 'sizePrefixTypeNode')) {
        if (isNode(typeNode.type, 'bytesTypeNode')) {
            return `${fieldAccess}.into_iter().map(carbon_core::graphql::primitives::U8).collect()`;
        }
        return buildConversionFromPostgresRow(typeNode.type, fieldAccess);
    }

    if (isNode(typeNode, 'remainderOptionTypeNode')) {
        if (isNode(typeNode.item, 'arrayTypeNode') && isNode(typeNode.item.item, 'definedTypeLinkNode')) {
            const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'v.0.clone()');
            return `${fieldAccess}.map(|v| ${innerExpr})`;
        }
        const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'v');
        return `${fieldAccess}.map(|v| ${innerExpr})`;
    }

    if (isNode(typeNode, 'zeroableOptionTypeNode')) {
        const postgresManifest = visit(typeNode.item, getPostgresTypeManifestVisitor()) as PostgresTypeManifest;
        const isJson = (postgresManifest.postgresColumnType || '').toUpperCase().startsWith('JSONB');

        if (isJson) {
            if (isNode(typeNode.item, 'booleanTypeNode')) {
                return handleBooleanTypeConversion(fieldAccess, true, false);
            }
            if (isNode(typeNode.item, 'publicKeyTypeNode')) {
                return `carbon_core::graphql::primitives::Pubkey(${fieldAccess}.0)`;
            }
            if (isNode(typeNode.item, 'arrayTypeNode') && isNode(typeNode.item.item, 'definedTypeLinkNode')) {
                const innerExpr = buildConversionFromPostgresRow(typeNode.item, `${fieldAccess}.0.clone()`);
                return innerExpr;
            }
            const innerExpr = buildConversionFromPostgresRow(typeNode.item, `${fieldAccess}.0`);
            return innerExpr;
        } else {
            if (isNode(typeNode.item, 'publicKeyTypeNode')) {
                return `${fieldAccess}.map(|v| carbon_core::graphql::primitives::Pubkey(v.0))`;
            }
            if (isNode(typeNode.item, 'booleanTypeNode')) {
                return handleBooleanTypeConversion(fieldAccess, false, true);
            }
            if (isNode(typeNode.item, 'arrayTypeNode') && isNode(typeNode.item.item, 'definedTypeLinkNode')) {
                const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'v.0.clone()');
                return `${fieldAccess}.map(|v| ${innerExpr})`;
            }
            const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'v');
            return buildOptionMapExpression(fieldAccess, innerExpr, 'v');
        }
    }

    if (isNode(typeNode, 'hiddenPrefixTypeNode')) {
        return buildConversionFromPostgresRow(typeNode.type, fieldAccess);
    }

    if (isNode(typeNode, 'definedTypeLinkNode')) {
        const typeName = typeNode.name.toLowerCase();
        if (typeName.includes('decrypt') || typeName.includes('cipher') || typeName.includes('elgamal')) {
            return `${fieldAccess}.0.into_iter().map(carbon_core::graphql::primitives::U8).collect()`;
        }
        if (isBooleanTypeAlias(typeName)) {
            return handleBooleanTypeConversion(fieldAccess, true, false);
        }
        return `${fieldAccess}.0.into()`;
    }

    if (isNode(typeNode, 'tupleTypeNode')) {
        if (typeNode.items.length === 1) {
            return `${fieldAccess}.into()`;
        }
        return fieldAccess;
    }
    if (isNode(typeNode, 'mapTypeNode') || isNode(typeNode, 'setTypeNode')) {
        return fieldAccess;
    }

    return fieldAccess;
}
