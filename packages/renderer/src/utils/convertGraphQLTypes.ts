import { TypeNode, isNode } from '@codama/nodes';

export function buildConversionFromOriginal(typeNode: TypeNode, fieldAccess: string): string {
    if (isNode(typeNode, 'publicKeyTypeNode')) {
        return `carbon_gql_server::types::pubkey::Pubkey(${fieldAccess})`;
    }

    if (isNode(typeNode, 'bytesTypeNode')) {
        return `base64::engine::general_purpose::STANDARD.encode(&${fieldAccess})`;
    }

    if (isNode(typeNode, 'numberTypeNode')) {
        switch (typeNode.format) {
            case 'u8':
                return `carbon_gql_server::types::u8::U8(${fieldAccess})`;
            case 'u16':
                return `${fieldAccess} as i32`;
            case 'u32':
                return `carbon_gql_server::types::u32::U32(${fieldAccess})`;
            case 'i64':
                return `carbon_gql_server::types::i64::I64(${fieldAccess})`;
            case 'u64':
                return `carbon_gql_server::types::u64::U64(${fieldAccess})`;
            case 'i128':
                return `carbon_gql_server::types::i128::I128(${fieldAccess})`;
            case 'u128':
                return `carbon_gql_server::types::u128::U128(${fieldAccess})`;
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

    if (isNode(typeNode, 'definedTypeLinkNode')) {
        return `${fieldAccess}.into()`;
    }

    if (isNode(typeNode, 'tupleTypeNode') || isNode(typeNode, 'mapTypeNode') || isNode(typeNode, 'setTypeNode')) {
        return `serde_json::to_value(&${fieldAccess}).unwrap_or(serde_json::Value::Null)`;
    }

    return fieldAccess;
}

export function buildConversionFromPostgresRow(typeNode: TypeNode, fieldAccess: string): string {
    if (isNode(typeNode, 'publicKeyTypeNode')) {
        return `carbon_gql_server::types::pubkey::Pubkey(*${fieldAccess})`;
    }

    if (isNode(typeNode, 'bytesTypeNode')) {
        return `base64::engine::general_purpose::STANDARD.encode(&${fieldAccess})`;
    }

    if (isNode(typeNode, 'numberTypeNode')) {
        switch (typeNode.format) {
            case 'u8':
                return `carbon_gql_server::types::u8::U8((*${fieldAccess}) as u8)`;
            case 'u16':
                return `(*${fieldAccess}) as i32`;
            case 'u32':
                return `carbon_gql_server::types::u32::U32((*${fieldAccess}) as u32)`;
            case 'i64':
                return `carbon_gql_server::types::i64::I64(${fieldAccess})`;
            case 'u64':
                return `carbon_gql_server::types::u64::U64(*${fieldAccess})`;
            case 'i128':
                return `carbon_gql_server::types::i128::I128(*${fieldAccess})`;
            case 'u128':
                return `carbon_gql_server::types::u128::U128(*${fieldAccess})`;
        }
    }

    if (isNode(typeNode, 'optionTypeNode')) {
        const innerExpr = buildConversionFromPostgresRow(typeNode.item, 'v');
        return `${fieldAccess}.map(|v| ${innerExpr})`;
    }

    if (isNode(typeNode, 'arrayTypeNode')) {
        if (isNode(typeNode.item, 'definedTypeLinkNode')) {
            // Array of custom types from Json<Vec<T>>
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

    if (isNode(typeNode, 'definedTypeLinkNode')) {
        // From sqlx::types::Json<T> to TGraphQL
        return `${fieldAccess}.0.into()`;
    }

    if (isNode(typeNode, 'tupleTypeNode') || isNode(typeNode, 'mapTypeNode') || isNode(typeNode, 'setTypeNode')) {
        return fieldAccess;
    }

    return fieldAccess;
}
