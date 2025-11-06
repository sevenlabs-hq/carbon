import { TypeNode, isNode } from '@codama/nodes';
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

export function buildConversionFromOriginal(typeNode: TypeNode, fieldAccess: string): string {
    if (isNode(typeNode, 'publicKeyTypeNode')) {
        return `carbon_core::graphql::primitives::Pubkey(${fieldAccess})`;
    }

    if (isNode(typeNode, 'bytesTypeNode')) {
        return `${fieldAccess}.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect()`;
    }

    if (isNode(typeNode, 'amountTypeNode')) {
        // AmountTypeNode wraps a NumberTypeNode - unwrap and process
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
        return `${fieldAccess}.map(|v| ${innerExpr})`;
    }

    if (isNode(typeNode, 'arrayTypeNode')) {
        // Special case: Vec<u8> should be treated like bytes
        if (isNode(typeNode.item, 'numberTypeNode') && typeNode.item.format === 'u8') {
            return `${fieldAccess}.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect()`;
        }

        const innerExpr = buildConversionFromOriginal(typeNode.item, 'item');
        return `${fieldAccess}.into_iter().map(|item| ${innerExpr}).collect()`;
    }

    if (isNode(typeNode, 'fixedSizeTypeNode')) {
        // For [u8; N] coming from original types as Vec<u8>
        if (isNode(typeNode.type, 'bytesTypeNode')) {
            return `${fieldAccess}.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect()`;
        }
        const innerExpr = buildConversionFromOriginal(typeNode.type, 'item');
        return `${fieldAccess}.into_iter().map(|item| ${innerExpr}).collect()`;
    }

    if (isNode(typeNode, 'sizePrefixTypeNode')) {
        // Size-prefixed types (like bytes with length prefix) - unwrap and convert inner type
        return buildConversionFromOriginal(typeNode.type, fieldAccess);
    }

    if (isNode(typeNode, 'remainderOptionTypeNode')) {
        const innerExpr = buildConversionFromOriginal(typeNode.item, 'v');
        return `${fieldAccess}.map(|v| ${innerExpr})`;
    }

    if (isNode(typeNode, 'zeroableOptionTypeNode')) {
        const innerExpr = buildConversionFromOriginal(typeNode.item, 'v');
        const convertedExpr = innerExpr === 'v' ? 'v.into()' : innerExpr;
        return `${fieldAccess}.map(|v| ${convertedExpr})`;
    }

    if (isNode(typeNode, 'hiddenPrefixTypeNode')) {
        return buildConversionFromOriginal(typeNode.type, fieldAccess);
    }

    if (isNode(typeNode, 'definedTypeLinkNode')) {
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
        return `${fieldAccess}.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect()`;
    }

    if (isNode(typeNode, 'amountTypeNode')) {
        // AmountTypeNode wraps a NumberTypeNode - unwrap and process
        return buildConversionFromPostgresRow(typeNode.number, fieldAccess);
    }

    if (isNode(typeNode, 'numberTypeNode')) {
        switch (typeNode.format) {
            case 'u8':
                return `carbon_core::graphql::primitives::U8((*${fieldAccess}) as u8)`;
            case 'u16':
                return `(*${fieldAccess}) as i32`;
            case 'i8':
            case 'i16':
                return `${fieldAccess} as i32`;
            case 'u32':
                return `carbon_core::graphql::primitives::U32((*${fieldAccess}) as u32)`;
            case 'i64':
                return `carbon_core::graphql::primitives::I64(${fieldAccess})`;
            case 'u64':
                return `carbon_core::graphql::primitives::U64(*${fieldAccess})`;
            case 'i128':
                return `carbon_core::graphql::primitives::I128(*${fieldAccess})`;
            case 'u128':
                return `carbon_core::graphql::primitives::U128(*${fieldAccess})`;
            case 'f32':
                return `${fieldAccess} as f64`;
            case 'f64':
                return `${fieldAccess}`;
        }
    }

    if (isNode(typeNode, 'optionTypeNode')) {
        // Check if inner is array of definedTypeLink stored as Json - needs .clone() to avoid move
        if (isNode(typeNode.item, 'arrayTypeNode') && isNode(typeNode.item.item, 'definedTypeLinkNode')) {
            const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'v.0.clone()');
            return `${fieldAccess}.map(|v| ${innerExpr})`;
        }
        const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'v');
        return `${fieldAccess}.map(|v| ${innerExpr})`;
    }

    if (isNode(typeNode, 'arrayTypeNode')) {
        // Special case: Vec<u8> should be treated like bytes
        if (isNode(typeNode.item, 'numberTypeNode') && typeNode.item.format === 'u8') {
            return `${fieldAccess}.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect()`;
        }

        // Generic nested-array handling: unwrap outer Json if present, then map inner with recursive converter
        if (isNode(typeNode.item, 'arrayTypeNode')) {
            const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'item');
            return `${fieldAccess}.0.into_iter().map(|item| ${innerExpr}).collect()`;
        }
        if (isNode(typeNode.item, 'definedTypeLinkNode')) {
            // If fieldAccess contains .0 (unwrapping Json), use as_ref to avoid move
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
            return `${fieldAccess}.into_iter().map(|item| ${innerExpr}).collect()`;
        }
    }

    if (isNode(typeNode, 'fixedSizeTypeNode')) {
        // For [u8; N] stored in Postgres as Vec<u8>, map each element to U8
        if (isNode(typeNode.type, 'bytesTypeNode')) {
            return `${fieldAccess}.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect()`;
        }
        const innerExpr = buildConversionFromPostgresRow(typeNode.type, 'item');
        return `${fieldAccess}.into_iter().map(|item| ${innerExpr}).collect()`;
    }

    if (isNode(typeNode, 'sizePrefixTypeNode')) {
        // Size-prefixed types (like bytes with length prefix) - unwrap and convert inner type
        // Special case: if the inner type is bytes, treat it as Vec<u8> -> Vec<U8>
        if (isNode(typeNode.type, 'bytesTypeNode')) {
            return `${fieldAccess}.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect()`;
        }
        return buildConversionFromPostgresRow(typeNode.type, fieldAccess);
    }

    if (isNode(typeNode, 'remainderOptionTypeNode')) {
        // Check if inner is array of definedTypeLink stored as Json - needs .clone() to avoid move
        if (isNode(typeNode.item, 'arrayTypeNode') && isNode(typeNode.item.item, 'definedTypeLinkNode')) {
            const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'v.0.clone()');
            return `${fieldAccess}.map(|v| ${innerExpr})`;
        }
        const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'v');
        return `${fieldAccess}.map(|v| ${innerExpr})`;
    }

    if (isNode(typeNode, 'zeroableOptionTypeNode')) {
        // Check if inner type is publicKeyTypeNode - needs special handling for wrapper
        if (isNode(typeNode.item, 'publicKeyTypeNode')) {
            return `${fieldAccess}.map(|v| carbon_core::graphql::primitives::Pubkey(v.0))`;
        }
        // Check if inner is array of definedTypeLink stored as Json - needs .clone() to avoid move
        if (isNode(typeNode.item, 'arrayTypeNode') && isNode(typeNode.item.item, 'definedTypeLinkNode')) {
            const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'v.0.clone()');
            return `${fieldAccess}.map(|v| ${innerExpr})`;
        }
        const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'v');
        return `${fieldAccess}.map(|v| ${innerExpr})`;
    }

    if (isNode(typeNode, 'hiddenPrefixTypeNode')) {
        return buildConversionFromPostgresRow(typeNode.type, fieldAccess);
    }

    if (isNode(typeNode, 'definedTypeLinkNode')) {
        // Special handling for decryptable/ciphertext-style byte arrays (e.g., token-2022)
        // These are defined types that resolve to Vec<u8> stored as JSONB
        const typeName = typeNode.name.toLowerCase();
        if (typeName.includes('decrypt') || typeName.includes('cipher') || typeName.includes('elgamal')) {
            return `${fieldAccess}.0.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect()`;
        }
        // Default: unwrap once then into()
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
