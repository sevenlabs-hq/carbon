import { isNode, REGISTERED_TYPE_NODE_KINDS, pascalCase } from '@codama/nodes';
import { extendVisitor, mergeVisitor, pipe, visit } from '@codama/visitors-core';
import { ImportMap } from './ImportMap';

export type GraphQLTypeManifest = {
    imports: ImportMap;
    graphqlType: string;
    isNullable: boolean;
};

export function getGraphQLTypeManifestVisitor() {
    return pipe(
        mergeVisitor<GraphQLTypeManifest>(
            (): GraphQLTypeManifest => ({
                imports: new ImportMap(),
                graphqlType: '',
                isNullable: false,
            }),
            (_k, vals) => ({
                imports: new ImportMap().mergeWith(...vals.map(v => v.imports)),
                graphqlType: vals.map(v => v.graphqlType).join(''),
                isNullable: vals.some(v => v.isNullable),
            }),
            { keys: [...REGISTERED_TYPE_NODE_KINDS, 'definedTypeLinkNode'] },
        ),
        v =>
            extendVisitor(v, {
                /* primitives */
                visitBooleanType() {
                    return m('bool');
                },
                visitBytesType() {
                    return m('String');
                },
                visitStringType() {
                    return m('String');
                },
                visitPublicKeyType() {
                    // Use the existing Juniper Pubkey scalar
                    return m('Pubkey', ['carbon_gql_server::types::pubkey::Pubkey']);
                },
                visitNumberType(node) {
                    switch (node.format) {
                        case 'i8':
                        case 'i16':
                        case 'i32':
                            return m('i32');
                        case 'u8':
                            return m('U8', ['carbon_gql_server::types::u8::U8']);
                        case 'u16':
                            return m('i32'); // Fits in i32
                        case 'u32':
                            return m('U32', ['carbon_gql_server::types::u32::U32']);
                        case 'i64':
                            return m('I64', ['carbon_gql_server::types::i64::I64']);
                        case 'u64':
                            return m('U64', ['carbon_gql_server::types::u64::U64']);
                        case 'i128':
                            return m('I128', ['carbon_gql_server::types::i128::I128']);
                        case 'u128':
                            return m('U128', ['carbon_gql_server::types::u128::U128']);
                        case 'f32':
                            return m('f32');
                        case 'f64':
                            return m('f64');
                        default:
                            throw new Error(`Unsupported number format: ${node.format}`);
                    }
                },
                /* wrappers */
                visitOptionType(node, { self }) {
                    const inner = visit(node.item, self);
                    return {
                        imports: inner.imports,
                        graphqlType: `Option<${inner.graphqlType}>`,
                        isNullable: true,
                    };
                },
                visitArrayType(node, { self }) {
                    const inner = visit(node.item, self);
                    return {
                        imports: inner.imports,
                        graphqlType: `Vec<${inner.graphqlType}>`,
                        isNullable: false,
                    };
                },
                visitMapType() {
                    return m('serde_json::Value', ['serde_json']);
                },
                visitSetType(node, { self }) {
                    const inner = visit(node.item, self);
                    return {
                        imports: inner.imports,
                        graphqlType: `Vec<${inner.graphqlType}>`,
                        isNullable: false,
                    };
                },
                visitTupleType() {
                    return m('serde_json::Value', ['serde_json']);
                },
                visitSizePrefixType(node, { self }) {
                    return visit(node.type, self);
                },
                visitZeroableOptionType(node, { self }) {
                    const inner = visit(node.item, self);
                    return {
                        imports: inner.imports,
                        graphqlType: `Option<${inner.graphqlType}>`,
                        isNullable: true,
                    };
                },
                visitDefinedTypeLink(node) {
                    // Reference the GraphQL version of custom types
                    const typeName = pascalCase(node.name);
                    return {
                        imports: new ImportMap().add([`crate::types::graphql::${typeName}GraphQL`]),
                        graphqlType: `${typeName}GraphQL`,
                        isNullable: false,
                    };
                },
            }),
    );

    function m(rustType: string, extraImports: string[] = []): GraphQLTypeManifest {
        return {
            imports: new ImportMap().add(extraImports),
            graphqlType: rustType,
            isNullable: false,
        };
    }
}
