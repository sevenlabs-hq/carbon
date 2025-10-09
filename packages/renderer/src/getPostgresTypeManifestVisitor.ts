import { isNode, REGISTERED_TYPE_NODE_KINDS, pascalCase } from '@codama/nodes';
import { extendVisitor, mergeVisitor, pipe, visit } from '@codama/visitors-core';
import { ImportMap } from './ImportMap';

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
                            return m('i8', 'SMALLINT');
                        case 'i16':
                            return m('i16', 'SMALLINT');
                        case 'u8':
                            return m('U8', 'SMALLINT', ['carbon_core::postgres::primitives::U8']);
                        case 'u16':
                            return m('U16', 'INTEGER', ['carbon_core::postgres::primitives::U16']);
                        case 'i32':
                            return m('i32', 'INTEGER');
                        case 'u32':
                            return m('U32', 'BIGINT', ['carbon_core::postgres::primitives::U32']);
                        case 'i64':
                            return m('i64', 'BIGINT');
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
                // TODO: What about wrapper types OptionBool(pub bool)? Rn it's as (bool)
                visitTupleType(node, { self }) {
                    const inners = node.items.map(i => visit(i, self));
                    return {
                        imports: new ImportMap().mergeWith(...inners.map(i => i.imports)),
                        sqlxType: `(${inners.map(i => i.sqlxType).join(', ')})`,
                        postgresColumnType: `(${inners.map(i => i.postgresColumnType).join(', ')})`,
                    };
                },
                visitSizePrefixType(node, { self }) {
                    return visit(node.type, self);
                },
                visitZeroableOptionType(node, { self }) {
                    return visit(node.item, self);
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
