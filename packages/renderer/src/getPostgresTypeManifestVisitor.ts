import { isNode, REGISTERED_TYPE_NODE_KINDS, pascalCase } from '@codama/nodes';
import { extendVisitor, mergeVisitor, pipe, visit } from '@codama/visitors-core';
import { ImportMap } from './ImportMap';
/*
Postgres type manifest rules:
- Numbers/PublicKey → dedicated wrapper types for safe DB storage (e.g., U64)
- Arrays of primitives → PG arrays when possible; complex/defined types → JSONB via sqlx::types::Json<T>
- Maps/Tuples/Sets → JSONB
- Singleton tuples → inner type
- Defined types → JSONB to preserve structure
*/

export type PostgresTypeManifest = {
    imports: ImportMap;
    sqlxType: string;
    postgresColumnType: string;
};

export function getPostgresTypeManifestVisitor() {
    return pipe(
        mergeVisitor<PostgresTypeManifest>(
            (): PostgresTypeManifest => ({
                imports: new ImportMap(),
                sqlxType: '',
                postgresColumnType: '',
            }),
            (_k, vals) => ({
                imports: new ImportMap().mergeWith(...vals.map(v => v.imports)),
                sqlxType: vals.map(v => v.sqlxType).join(''),
                postgresColumnType: vals.map(v => v.postgresColumnType).join(''),
            }),
            { keys: [...REGISTERED_TYPE_NODE_KINDS, 'definedTypeLinkNode'] },
        ),
        v =>
            extendVisitor(v, {
                /* primitives */
                visitBooleanType() {
                    return m('bool', 'BOOLEAN');
                },
                visitBytesType() {
                    return m('Vec<u8>', 'BYTEA');
                },
                visitStringType() {
                    return m('String', 'TEXT');
                },
                visitPublicKeyType() {
                    return m('Pubkey', 'BYTEA', ['carbon_core::postgres::primitives::Pubkey']);
                },
                visitNumberType(node) {
                    switch (node.format) {
                        case 'i8':
                            return m('i8', 'INT2');
                        case 'i16':
                            return m('i16', 'INT2');
                        case 'u8':
                            return m('U8', 'INT2', ['carbon_core::postgres::primitives::U8']);
                        case 'u16':
                            return m('U16', 'INT4', ['carbon_core::postgres::primitives::U16']);
                        case 'i32':
                            return m('i32', 'INT4');
                        case 'u32':
                            return m('U32', 'INT8', ['carbon_core::postgres::primitives::U32']);
                        case 'i64':
                            return m('i64', 'INT8');
                        case 'u64':
                            return m('U64', 'NUMERIC(20)', ['carbon_core::postgres::primitives::U64']);
                        case 'i128':
                            return m('I128', 'NUMERIC(38)', ['carbon_core::postgres::primitives::I128']);
                        case 'u128':
                            return m('U128', 'NUMERIC(39)', ['carbon_core::postgres::primitives::U128']);
                        case 'f32':
                            return m('f32', 'REAL');
                        case 'f64':
                            return m('f64', 'DOUBLE PRECISION');
                        default:
                            throw new Error(`Unsupported number format: ${node.format}`);
                    }
                },
                /* wrappers */
                visitOptionType(node, { self }) {
                    const inner = visit(node.item, self);
                    return {
                        imports: inner.imports,
                        sqlxType: `Option<${inner.sqlxType}>`,
                        postgresColumnType: inner.postgresColumnType,
                    };
                },
                visitRemainderOptionType(node, { self }) {
                    const inner = visit(node.item, self);
                    return {
                        imports: inner.imports,
                        sqlxType: `Option<${inner.sqlxType}>`,
                        postgresColumnType: inner.postgresColumnType,
                    };
                },
                visitHiddenPrefixType(node, { self }) {
                    const inner = visit(node.type, self);
                    return inner;
                },
                visitArrayType(node, { self }) {
                    const inner = visit(node.item, self);
                    if (
                        isNode(node.item, 'numberTypeNode') ||
                        isNode(node.item, 'booleanTypeNode') ||
                        isNode(node.item, 'bytesTypeNode') ||
                        isNode(node.item, 'stringTypeNode') ||
                        isNode(node.item, 'publicKeyTypeNode')
                    ) {
                        return {
                            imports: inner.imports,
                            sqlxType: `Vec<${inner.sqlxType}>`,
                            postgresColumnType: `${inner.postgresColumnType}[]`,
                        };
                    }
                    return jsonb(inner, true);
                },
                visitMapType() {
                    return jsonb();
                },
                visitSetType() {
                    return jsonb();
                },
                visitTupleType(node, { self }) {
                    const inners = node.items.map(i => visit(i, self));
                    if (inners.length === 1) {
                        return inners[0];
                    }
                    return {
                        imports: new ImportMap().mergeWith(...inners.map(i => i.imports)),
                        sqlxType: `(${inners.map(i => i.sqlxType).join(', ')})`,
                        postgresColumnType: `(${inners.map(i => i.postgresColumnType).join(', ')})`,
                    };
                },
                visitFixedSizeType(node, { self }) {
                    // Fixed-size bytes stored as Vec<u8> (BYTEA) in Postgres
                    // The inner type visitor will handle bytesTypeNode → Vec<u8>
                    return visit(node.type, self);
                },
                visitSizePrefixType(node, { self }) {
                    return visit(node.type, self);
                },
                visitZeroableOptionType(node, { self }) {
                    const inner = visit(node.item, self);
                    return {
                        imports: inner.imports,
                        sqlxType: `Option<${inner.sqlxType}>`,
                        postgresColumnType: inner.postgresColumnType,
                    };
                },
                visitDefinedTypeLink(node) {
                    return m(`${pascalCase(node.name)}`, 'JSONB', [`crate::types::${pascalCase(node.name)}`]);
                },
            }),
    );

    function m(rust: string, sql: string, extra: string[] = []): PostgresTypeManifest {
        return { imports: new ImportMap().add(extra), sqlxType: rust, postgresColumnType: sql };
    }
    function jsonb(inner?: PostgresTypeManifest, isVec = false): PostgresTypeManifest {
        const base = inner ?? { imports: new ImportMap(), sqlxType: 'serde_json::Value', postgresColumnType: 'JSONB' };
        return {
            imports: base.imports,
            sqlxType: isVec ? `sqlx::types::Json<Vec<${base.sqlxType}>>` : `sqlx::types::Json<${base.sqlxType}>`,
            postgresColumnType: 'JSONB',
        };
    }
}
