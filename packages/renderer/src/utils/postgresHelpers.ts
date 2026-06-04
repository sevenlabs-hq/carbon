import { isNode, TypeNode } from '@codama/nodes';

/**
 * Returns true when the Postgres sqlx type is one of the custom wrapper primitives
 * that cannot be used interchangeably with the plain Rust numeric types.
 */
export function isPostgresPrimitiveType(type: string): boolean {
    return (
        type.includes('U8') ||
        type.includes('U16') ||
        type.includes('U32') ||
        type.includes('U64') ||
        type.includes('U128') ||
        type.includes('I128')
    );
}

/** Returns true when the Rust row type implements Copy. */
export function isCopyType(rowType: string): boolean {
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
        'Pubkey',
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
            return isCopyType(innerMatch[1].trim());
        }
    }

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

    return copyPrimitives.includes(trimmed);
}

/**
 * Checks if two type strings refer to the same underlying type, stripping Json<> wrappers
 * and comparing base names.
 */
export function typesMatch(postgresType: string, rustType: string): boolean {
    let pgType = postgresType.trim();
    const jsonMatch = pgType.match(/^Json<(.+)>$/);
    if (jsonMatch) {
        pgType = jsonMatch[1].trim();
    }

    const rustTypeBase = rustType.trim();

    if (pgType === rustTypeBase) {
        return true;
    }

    if (pgType.endsWith(rustTypeBase) || rustTypeBase.endsWith(pgType)) {
        const pgBase = pgType.split('::').pop() || pgType;
        const rustBase = rustTypeBase.split('::').pop() || rustTypeBase;
        if (pgBase === rustBase) {
            return true;
        }
    }

    return false;
}

/**
 * Recursively checks whether a type node requires the serde-big-array crate
 * (i.e. it contains a fixed array with more than 32 elements or a fixed-size byte buffer > 32 bytes).
 */
export function checkRequiresBigArray(typeNode: TypeNode): boolean {
    if (isNode(typeNode, 'arrayTypeNode')) {
        if (isNode(typeNode.count, 'fixedCountNode') && typeNode.count.value > 32) {
            return true;
        }
        return checkRequiresBigArray(typeNode.item);
    }
    if (isNode(typeNode, 'fixedSizeTypeNode')) {
        if (isNode(typeNode.type, 'bytesTypeNode') && typeNode.size > 32) {
            return true;
        }
        return checkRequiresBigArray(typeNode.type);
    }
    if (isNode(typeNode, 'structTypeNode')) {
        return typeNode.fields.some(field => checkRequiresBigArray(field.type));
    }
    if (isNode(typeNode, 'enumTypeNode')) {
        return typeNode.variants.some(variant => {
            const v = variant as any;
            if (v.kind === 'enumStructVariantTypeNode') {
                const fields = v.fields ?? v.struct?.fields;
                return Array.isArray(fields) && fields.some((field: any) => checkRequiresBigArray(field.type));
            }
            if (v.kind === 'enumTupleVariantTypeNode' && v.tuple?.items) {
                return v.tuple.items.some((item: any) => checkRequiresBigArray(item));
            }
            return false;
        });
    }
    if (isNode(typeNode, 'optionTypeNode') || isNode(typeNode, 'zeroableOptionTypeNode')) {
        return checkRequiresBigArray(typeNode.item);
    }
    return false;
}
