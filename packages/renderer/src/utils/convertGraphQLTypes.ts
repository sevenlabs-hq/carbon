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

    if (isNode(typeNode, 'numberTypeNode')) {
        switch (typeNode.format) {
            case 'u8':
                return `carbon_core::graphql::primitives::U8(${fieldAccess})`;
            case 'u16':
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
        }
    }

    if (isNode(typeNode, 'optionTypeNode')) {
        const innerExpr = buildConversionFromOriginal(typeNode.item, 'v');
        return `${fieldAccess}.map(|v| ${innerExpr})`;
    }

    if (isNode(typeNode, 'arrayTypeNode')) {
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
        return `carbon_core::graphql::primitives::Pubkey(*${fieldAccess})`;
    }

    if (isNode(typeNode, 'bytesTypeNode')) {
        return `${fieldAccess}.into_iter().map(|item| carbon_core::graphql::primitives::U8(item)).collect()`;
    }

    if (isNode(typeNode, 'numberTypeNode')) {
        switch (typeNode.format) {
            case 'u8':
                return `carbon_core::graphql::primitives::U8((*${fieldAccess}) as u8)`;
            case 'u16':
                return `(*${fieldAccess}) as i32`;
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
        }
    }

    if (isNode(typeNode, 'optionTypeNode')) {
        const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'v');
        return `${fieldAccess}.map(|v| ${innerExpr})`;
    }

    if (isNode(typeNode, 'arrayTypeNode')) {
        // Generic nested-array handling: unwrap outer Json if present, then map inner with recursive converter
        if (isNode(typeNode.item, 'arrayTypeNode')) {
            const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'item');
            return `${fieldAccess}.0.into_iter().map(|item| ${innerExpr}).collect()`;
        }
        if (isNode(typeNode.item, 'definedTypeLinkNode')) {
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

    if (isNode(typeNode, 'definedTypeLinkNode')) {
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
