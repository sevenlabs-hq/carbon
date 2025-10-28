import { isNode, REGISTERED_TYPE_NODE_KINDS, pascalCase } from '@codama/nodes';
/*
GraphQL type manifest rules:
- PublicKey → Pubkey scalar
- Numbers → i32 where safe; U8,U32,I64,U64,I128,U128 scalars to keep values lossless
- Options → Option<T> (nullable)
- Arrays → Vec<T>
- Tuples/Sets → Json (singleton tuple collapses to inner type)
- Maps → Json
- Defined types → <TypeName>GraphQL
*/
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
                    return m('Vec<U8>', ['carbon_core::graphql::primitives::U8']);
                },
                visitStringType() {
                    return m('String');
                },
                visitPublicKeyType() {
                    return m('Pubkey', ['carbon_core::graphql::primitives::Pubkey']);
                },
                visitNumberType(node) {
                    switch (node.format) {
                        case 'i8':
                        case 'i16':
                        case 'i32':
                            return m('i32');
                        case 'u8':
                            return m('U8', ['carbon_core::graphql::primitives::U8']);
                        case 'u16':
                            return m('i32');
                        case 'u32':
                            return m('U32', ['carbon_core::graphql::primitives::U32']);
                        case 'i64':
                            return m('I64', ['carbon_core::graphql::primitives::I64']);
                        case 'u64':
                            return m('U64', ['carbon_core::graphql::primitives::U64']);
                        case 'i128':
                            return m('I128', ['carbon_core::graphql::primitives::I128']);
                        case 'u128':
                            return m('U128', ['carbon_core::graphql::primitives::U128']);
                        case 'f32':
                            return m('f64');
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
                visitRemainderOptionType(node, { self }) {
                    const inner = visit(node.item, self);
                    return {
                        imports: inner.imports,
                        graphqlType: `Option<${inner.graphqlType}>`,
                        isNullable: true,
                    };
                },
                visitHiddenPrefixType(node, { self }) {
                    const inner = visit(node.type, self);
                    return inner;
                },
                visitArrayType(node, { self }) {
                    const inner = visit(node.item, self);
                    return {
                        imports: inner.imports,
                        graphqlType: `Vec<${inner.graphqlType}>`,
                        isNullable: false,
                    };
                },
                visitFixedSizeType(node, { self }) {
                    // array that has 'bytes' inside is vec<u8>, not vec<vec<u8>>
                    if (node.type.kind === 'bytesTypeNode') {
                        return m('Vec<U8>', ['carbon_core::graphql::primitives::U8']);
                    }
                    const inner = visit(node.type, self);
                    return {
                        imports: inner.imports,
                        graphqlType: `Vec<${inner.graphqlType}>`,
                        isNullable: false,
                    };
                },
                visitMapType() {
                    return m('Json', ['carbon_core::graphql::primitives::Json']);
                },
                visitSetType(node, { self }) {
                    const inner = visit(node.item, self);
                    return {
                        imports: inner.imports,
                        graphqlType: `Vec<${inner.graphqlType}>`,
                        isNullable: false,
                    };
                },
                visitTupleType(node, { self }) {
                    // Fixes wrappers -> collapse to inner type (OptionBool(pub bool) should be just a bool)
                    if (node.items.length === 1) {
                        return visit(node.items[0], self) as GraphQLTypeManifest;
                    }
                    return m('Json', ['carbon_core::graphql::primitives::Json']);
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
