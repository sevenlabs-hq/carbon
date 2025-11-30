import {
    DiscriminatorNode,
    isNode,
    pascalCase,
    REGISTERED_TYPE_NODE_KINDS,
    resolveNestedTypeNode,
    snakeCase,
    TypeNode,
} from '@codama/nodes';
import { extendVisitor, mergeVisitor, pipe, visit } from '@codama/visitors-core';

import { ImportMap } from './ImportMap';
import { getDiscriminatorBytes } from './utils';
import { formatDocComments } from './utils/render';

export type TypeManifest = {
    imports: ImportMap;
    type: string;
    borshType: string;
    requiredBigArray?: number;
};

export type DiscriminatorManifest = {
    bytes: string;
    size: number;
    checkCode: string;
};

export function getTypeManifestVisitor(definedTypesMap?: Map<string, any> | null, newtypeWrapperTypes?: Set<string>) {
    const baseVisitor = pipe(
        mergeVisitor(
            (): TypeManifest => ({
                imports: new ImportMap(),
                type: '',
                borshType: '',
            }),
            (_, values) => ({
                imports: new ImportMap().mergeWith(...values.map(v => v.imports)),
                type: values.map(v => v.type).join('\n'),
                borshType: values.map(v => v.borshType).join('\n'),
            }),
            { keys: [...REGISTERED_TYPE_NODE_KINDS, 'definedTypeLinkNode'] },
        ),
        v => {
            const extended = extendVisitor(v, {
                visitArrayType(arrayType, { self }) {
                    const childManifest = visit(arrayType.item, self);

                    if (isNode(arrayType.count, 'fixedCountNode')) {
                        return {
                            imports: childManifest.imports,
                            type: `[${childManifest.type}; ${arrayType.count.value}]`,
                            borshType: `[${childManifest.borshType}; ${arrayType.count.value}]`,
                            requiredBigArray: arrayType.count.value > 32 ? arrayType.count.value : undefined,
                        };
                    }

                    if (isNode(arrayType.count, 'remainderCountNode')) {
                        return {
                            imports: childManifest.imports,
                            type: `Vec<${childManifest.type}>`,
                            borshType: `Vec<${childManifest.borshType}>`,
                        };
                    }

                    return {
                        imports: childManifest.imports,
                        type: `Vec<${childManifest.type}>`,
                        borshType: `Vec<${childManifest.borshType}>`,
                    };
                },

                visitBooleanType() {
                    return {
                        imports: new ImportMap(),
                        type: 'bool',
                        borshType: 'bool',
                    };
                },

                visitBytesType() {
                    return {
                        imports: new ImportMap(),
                        type: 'Vec<u8>',
                        borshType: 'Vec<u8>',
                    };
                },

                visitDefinedTypeLink(node) {
                    const pascalCaseType = pascalCase(node.name);
                    // Resolve the underlying type to check if it needs BigArray
                    // But skip if it's a newtype wrapper (which handles serialization itself)
                    let requiredBigArray: number | undefined = undefined;
                    // If this type was converted to a newtype wrapper, don't set requiredBigArray
                    if (!newtypeWrapperTypes || !newtypeWrapperTypes.has(node.name)) {
                        // Prioritize definedTypesMap lookup when available, then fall back to resolveNestedTypeNode
                        let resolvedRaw: any = undefined;
                        if (definedTypesMap) {
                            const typeName = node.name;
                            const definedType = definedTypesMap.get(typeName);
                            if (definedType && definedType.type) {
                                resolvedRaw = definedType.type;
                            }
                        }
                        if (!resolvedRaw) {
                            resolvedRaw = resolveNestedTypeNode(node);
                        }
                        if (resolvedRaw) {
                            const resolved = resolvedRaw as TypeNode;
                            if (isNode(resolved, 'fixedSizeTypeNode')) {
                                if (isNode(resolved.type, 'bytesTypeNode') && resolved.size > 32) {
                                    requiredBigArray = resolved.size;
                                }
                            } else if (isNode(resolved, 'arrayTypeNode')) {
                                if (isNode(resolved.count, 'fixedCountNode') && resolved.count.value > 32) {
                                    requiredBigArray = resolved.count.value;
                                }
                            }
                        }
                    }
                    return {
                        imports: new ImportMap().add(`crate::types::${pascalCaseType}`),
                        type: pascalCaseType,
                        borshType: pascalCaseType,
                        requiredBigArray,
                    };
                },

                visitEnumEmptyVariantType(node) {
                    const name = pascalCase(node.name);
                    return {
                        imports: new ImportMap(),
                        type: `${name},`,
                        borshType: `${name},`,
                    };
                },

                visitEnumStructVariantType(node, { self }) {
                    const name = pascalCase(node.name);
                    // Get the definedTypesMap and newtypeWrapperTypes from the original visitor (self)
                    // This is a workaround since we can't pass it through visitor context
                    const getDefinedTypesMap = (self as any).__definedTypesMap;
                    const enumDefinedTypesMap =
                        typeof getDefinedTypesMap === 'function' ? getDefinedTypesMap() : undefined;
                    const getNewtypeWrapperTypes = (self as any).__newtypeWrapperTypes;
                    const enumNewtypeWrapperTypes =
                        typeof getNewtypeWrapperTypes === 'function' ? getNewtypeWrapperTypes() : undefined;
                    // Store reference to original self for accessing definedTypesMap
                    const originalSelf = self;

                    // Create a custom visitor for enum struct variant fields that doesn't add 'pub'
                    // but includes BigArray detection logic
                    // Use originalSelf for visiting field types to ensure access to __definedTypesMap
                    const enumStructVisitor = extendVisitor(originalSelf, {
                        visitStructFieldType(node, { self: fieldSelf }) {
                            // Visit the field type with originalSelf to get proper type resolution
                            const fieldManifest = visit(node.type, originalSelf);
                            const fieldName = snakeCase(node.name);

                            // visitDefinedTypeLink already sets requiredBigArray appropriately (including undefined for newtype wrappers)
                            // Only need to override if it's a newtype wrapper to ensure it's undefined
                            let needsBigArray = fieldManifest.requiredBigArray !== undefined;
                            if (
                                isNode(node.type, 'definedTypeLinkNode') &&
                                enumNewtypeWrapperTypes &&
                                enumNewtypeWrapperTypes.has(node.type.name)
                            ) {
                                needsBigArray = false; // Newtype wrapper handles serialization itself
                            }

                            // Also check direct array types (not defined type links)
                            if (
                                !needsBigArray &&
                                !isNode(node.type, 'definedTypeLinkNode') &&
                                isNode(node.type, 'arrayTypeNode') &&
                                isNode(node.type.count, 'fixedCountNode') &&
                                node.type.count.value > 32
                            ) {
                                needsBigArray = true;
                            }

                            const docs = node.docs || [];
                            // Use formatDocComments to properly handle list item indentation
                            const docComments = docs.length > 0 ? formatDocComments(docs) + '\n' : '';

                            return {
                                imports: fieldManifest.imports,
                                type: `${docComments}${needsBigArray ? '#[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]\n' : ''}${fieldName}: ${fieldManifest.type},`,
                                borshType: `${fieldName}: ${fieldManifest.borshType},`,
                            };
                        },
                    });

                    const structManifest = visit(node.struct, enumStructVisitor);
                    return {
                        imports: structManifest.imports,
                        type: `${name} ${structManifest.type},`,
                        borshType: `${name} ${structManifest.borshType},`,
                    };
                },

                visitEnumTupleVariantType(node, { self }) {
                    const name = pascalCase(node.name);
                    const tupleManifest = visit(node.tuple, self);

                    const needsParens = !tupleManifest.type.startsWith('(');
                    const wrappedType = needsParens ? `(${tupleManifest.type})` : tupleManifest.type;
                    const wrappedBorshType = needsParens ? `(${tupleManifest.borshType})` : tupleManifest.borshType;

                    return {
                        imports: tupleManifest.imports,
                        type: `${name}${wrappedType},`,
                        borshType: `${name}${wrappedBorshType},`,
                    };
                },

                visitEnumType(node, { self }) {
                    const variants = node.variants.map(variant => visit(variant, self));
                    const mergedImports = new ImportMap().mergeWith(...variants.map(v => v.imports));
                    const variantTypes = variants.map(v => v.type).join('\n');
                    const variantBorshTypes = variants.map(v => v.borshType).join('\n');

                    return {
                        imports: mergedImports,
                        type: `{\n${variantTypes}\n}`,
                        borshType: `{\n${variantBorshTypes}\n}`,
                    };
                },

                visitFixedSizeType(node, { self }) {
                    // Special case: fixed-size bytes should be [u8; N] not Vec<u8>
                    if (isNode(node.type, 'bytesTypeNode')) {
                        return {
                            imports: new ImportMap(),
                            type: `[u8; ${node.size}]`,
                            borshType: `[u8; ${node.size}]`,
                            requiredBigArray: node.size > 32 ? node.size : undefined,
                        };
                    }
                    return visit(node.type, self);
                },

                visitMapType(node, { self }) {
                    const key = visit(node.key, self);
                    const value = visit(node.value, self);
                    const imports = new ImportMap()
                        .add('std::collections::HashMap')
                        .mergeWith(key.imports, value.imports);

                    return {
                        imports,
                        type: `HashMap<${key.type}, ${value.type}>`,
                        borshType: `HashMap<${key.borshType}, ${value.borshType}>`,
                    };
                },

                visitNumberType(node) {
                    return {
                        imports: new ImportMap(),
                        type: node.format,
                        borshType: node.format,
                    };
                },

                visitOptionType(node, { self }) {
                    const childManifest = visit(node.item, self);
                    return {
                        imports: childManifest.imports,
                        type: `Option<${childManifest.type}>`,
                        borshType: `Option<${childManifest.borshType}>`,
                    };
                },

                visitRemainderOptionType(node, { self }) {
                    const itemManifest = visit(node.item, self);
                    return {
                        imports: itemManifest.imports,
                        type: `Option<${itemManifest.type}>`,
                        borshType: `Option<${itemManifest.borshType}>`,
                    };
                },

                visitHiddenPrefixType(node, { self }) {
                    const typeManifest = visit(node.type, self);
                    return typeManifest;
                },

                visitPublicKeyType() {
                    return {
                        imports: new ImportMap().add('solana_pubkey::Pubkey'),
                        type: 'Pubkey',
                        borshType: 'Pubkey',
                    };
                },

                visitSetType(node, { self }) {
                    const childManifest = visit(node.item, self);
                    const imports = new ImportMap().add('std::collections::HashSet').mergeWith(childManifest.imports);

                    return {
                        imports,
                        type: `HashSet<${childManifest.type}>`,
                        borshType: `HashSet<${childManifest.borshType}>`,
                    };
                },

                visitSizePrefixType(node, { self }) {
                    return visit(node.type, self);
                },

                visitStringType() {
                    return {
                        imports: new ImportMap(),
                        type: 'String',
                        borshType: 'String',
                    };
                },

                visitStructFieldType(node, { self }) {
                    const fieldManifest = visit(node.type, self);
                    const fieldName = snakeCase(node.name);

                    // visitDefinedTypeLink already sets requiredBigArray appropriately (including undefined for newtype wrappers)
                    // Only check direct array types and use manifest's requiredBigArray
                    let needsBigArray = fieldManifest.requiredBigArray !== undefined;
                    // Also check direct array types (not defined type links, as those are handled by visitDefinedTypeLink)
                    if (
                        !needsBigArray &&
                        !isNode(node.type, 'definedTypeLinkNode') &&
                        isNode(node.type, 'arrayTypeNode') &&
                        isNode(node.type.count, 'fixedCountNode') &&
                        node.type.count.value > 32
                    ) {
                        needsBigArray = true;
                    }

                    const docs = node.docs || [];
                    // Use formatDocComments to properly handle list item indentation
                    const docComments = docs.length > 0 ? formatDocComments(docs) + '\n' : '';

                    return {
                        imports: fieldManifest.imports,
                        type: `${docComments}${needsBigArray ? '#[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]\n' : ''}pub ${fieldName}: ${fieldManifest.type},`,
                        borshType: `${fieldName}: ${fieldManifest.borshType},`,
                    };
                },

                visitStructType(node, { self }) {
                    const fields = node.fields.map(field => visit(field, self));
                    const mergedImports = new ImportMap().mergeWith(...fields.map(f => f.imports));
                    const fieldTypes = fields.map(f => f.type).join('\n');
                    const fieldBorshTypes = fields.map(f => f.borshType).join('\n');

                    return {
                        imports: mergedImports,
                        type: `{\n${fieldTypes}\n}`,
                        borshType: `{\n${fieldBorshTypes}\n}`,
                    };
                },

                visitTupleType(node, { self }) {
                    const items = node.items.map(item => visit(item, self));
                    const mergedImports = new ImportMap().mergeWith(...items.map(i => i.imports));
                    if (items.length === 1) {
                        return {
                            imports: mergedImports,
                            type: items[0].type,
                            borshType: items[0].borshType,
                        };
                    }
                    const itemTypes = items.map(i => i.type).join(', ');
                    const itemBorshTypes = items.map(i => i.borshType).join(', ');

                    return {
                        imports: mergedImports,
                        type: `(${itemTypes})`,
                        borshType: `(${itemBorshTypes})`,
                    };
                },

                visitZeroableOptionType(node, { self }) {
                    const childManifest = visit(node.item, self);
                    return {
                        imports: childManifest.imports,
                        type: `Option<${childManifest.type}>`,
                        borshType: `Option<${childManifest.borshType}>`,
                    };
                },
            });
            // Attach definedTypesMap and newtypeWrapperTypes getters to the visitor so enum variants can access them
            (extended as any).__definedTypesMap = () => definedTypesMap;
            (extended as any).__newtypeWrapperTypes = () => newtypeWrapperTypes;
            return extended;
        },
    );
    return baseVisitor;
}

export function getDiscriminatorManifest(
    discriminators: DiscriminatorNode[] | undefined,
    programName?: string | null,
    discriminatorNames?: Map<number, string>,
): DiscriminatorManifest | null {
    if (!discriminators || discriminators.length === 0) return null;

    // Handle multiple discriminators explicitly (for any IDL with nested discriminators)
    if (discriminators.length > 1) {
        const constantDiscriminators = discriminators.filter(d => d.kind === 'constantDiscriminatorNode');

        if (constantDiscriminators.length > 1) {
            // Sort by offset to ensure correct order
            const sorted = [...constantDiscriminators].sort((a, b) => {
                if (a.kind === 'constantDiscriminatorNode' && b.kind === 'constantDiscriminatorNode') {
                    return a.offset - b.offset;
                }
                return 0;
            });

            // Calculate maximum required size
            const maxRequiredSize = Math.max(
                ...sorted.map(d => {
                    if (d.kind === 'constantDiscriminatorNode') {
                        const bytes = getDiscriminatorBytes(d.constant);
                        return d.offset + bytes.length;
                    }
                    return 0;
                }),
            );

            // Generate check code for all discriminators
            const checkParts: string[] = [];
            const lengthCheck = maxRequiredSize === 1 ? 'data.is_empty()' : `data.len() < ${maxRequiredSize}`;
            checkParts.push(`if ${lengthCheck} {`);
            checkParts.push(`    return None;`);
            checkParts.push(`}`);

            // Check each discriminator in sequence
            for (const discriminator of sorted) {
                if (discriminator.kind === 'constantDiscriminatorNode') {
                    const bytes = getDiscriminatorBytes(discriminator.constant);
                    const size = bytes.length;
                    // Get variable name: use provided name map, or default based on offset
                    let varName: string;
                    if (discriminator.offset === 0) {
                        varName = 'discriminator';
                    } else if (discriminatorNames && discriminatorNames.has(discriminator.offset)) {
                        const name = discriminatorNames.get(discriminator.offset)!;
                        // Convert camelCase to snake_case
                        varName = name.replace(/([A-Z])/g, '_$1').toLowerCase();
                    } else {
                        varName = `discriminator_${discriminator.offset}`;
                    }

                    if (size === 1) {
                        // Single byte discriminator
                        checkParts.push(`let ${varName} = data[${discriminator.offset}];`);
                        checkParts.push(`if ${varName} != ${bytes[0]} {`);
                        checkParts.push(`    return None;`);
                        checkParts.push(`}`);
                    } else {
                        // Multi-byte discriminator
                        checkParts.push(
                            `let ${varName} = &data[${discriminator.offset}..${discriminator.offset + size}];`,
                        );
                        checkParts.push(`if ${varName} != [${bytes.join(', ')}] {`);
                        checkParts.push(`    return None;`);
                        checkParts.push(`}`);
                    }
                }
            }

            // Use the size of the first discriminator for slicing (as per manually modified version)
            const firstDiscriminator = sorted[0];
            if (firstDiscriminator.kind === 'constantDiscriminatorNode') {
                const firstBytes = getDiscriminatorBytes(firstDiscriminator.constant);
                const firstSize = firstBytes.length;
                const checkCode = checkParts.join('\n');
                return {
                    bytes: `[${firstBytes.join(', ')}]`,
                    size: firstSize,
                    checkCode,
                };
            }
        }
    }

    // For other programs or single discriminator, use original logic
    const discriminator = discriminators[0];

    switch (discriminator.kind) {
        case 'constantDiscriminatorNode': {
            const bytes = getDiscriminatorBytes(discriminator.constant);
            const size = bytes.length;
            const requiredSize = discriminator.offset + size;
            // Use is_empty() when checking for size 1, otherwise use len() < size
            const lengthCheck = requiredSize === 1 ? 'data.is_empty()' : `data.len() < ${requiredSize}`;
            const checkCode = `if ${lengthCheck} {
    return None;
}
let discriminator = &data[${discriminator.offset}..${discriminator.offset + size}];
if discriminator != [${bytes.join(', ')}] {
    return None;
}`;
            return { bytes: `[${bytes.join(', ')}]`, size, checkCode };
        }

        case 'fieldDiscriminatorNode': {
            // Field discriminators check a specific field value after deserialization
            const checkCode = `// Field discriminator: ${discriminator.name} at offset ${discriminator.offset}
// This check happens after deserialization`;
            return { bytes: '[]', size: 0, checkCode };
        }

        case 'sizeDiscriminatorNode': {
            const checkCode = `if data.len() != ${discriminator.size} {
    return None;
}`;
            return { bytes: '[]', size: 0, checkCode };
        }

        default:
            return null;
    }
}
