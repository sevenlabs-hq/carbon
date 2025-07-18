import {
    getAllAccounts,
    getAllDefinedTypes,
    getAllInstructionsWithSubs,
    getAllPrograms,
    ProgramNode,
    snakeCase,
} from '@codama/nodes';
import { RenderMap } from '@codama/renderers-core';
import { extendVisitor, pipe, staticVisitor, visit } from '@codama/visitors-core';

import { getTypeManifestVisitor } from './getTypeManifestVisitor';
import { ImportMap } from './ImportMap';
import { render } from './utils';

export type GetRenderMapOptions = {
    renderParentInstructions?: boolean;
};

export function getRenderMapVisitor(options: GetRenderMapOptions = {}) {
    const renderParentInstructions = options.renderParentInstructions ?? false;
    const typeManifestVisitor = getTypeManifestVisitor();
    let currentProgram: ProgramNode | null = null;

    return pipe(
        staticVisitor(() => new RenderMap(), {
            keys: ['rootNode', 'programNode', 'instructionNode', 'accountNode', 'definedTypeNode'],
        }),
        v =>
            extendVisitor(v, {
                visitAccount(node) {
                    const typeManifest = visit(node.data, typeManifestVisitor);
                    const imports = new ImportMap()
                        .mergeWithManifest(typeManifest)
                        .add('carbon_core::account::AccountDecoder')
                        .add('carbon_core::account::DecodedAccount');

                    return new RenderMap().add(
                        `accounts/${snakeCase(node.name)}.rs`,
                        render('accountsPage.njk', {
                            account: node,
                            imports: imports.toString(),
                            program: currentProgram,
                            typeManifest,
                        }),
                    );
                },

                visitDefinedType(node) {
                    const typeManifest = visit(node.type, typeManifestVisitor);
                    const imports = new ImportMap().mergeWithManifest(typeManifest).add('carbon_core::borsh');

                    return new RenderMap().add(
                        `types/${snakeCase(node.name)}.rs`,
                        render('typesPage.njk', {
                            definedType: node,
                            imports: imports.toString(),
                            typeManifest,
                        }),
                    );
                },

                visitInstruction(node) {
                    const imports = new ImportMap()
                        .add('carbon_core::instruction::InstructionDecoder')
                        .add('carbon_core::instruction::DecodedInstruction')
                        .add('carbon_core::deserialize::ArrangeAccounts');

                    // Collect all types from arguments
                    const argumentTypes = node.arguments.map(arg => {
                        const manifest = visit(arg.type, typeManifestVisitor);
                        imports.mergeWithManifest(manifest);
                        return manifest;
                    });

                    return new RenderMap().add(
                        `instructions/${snakeCase(node.name)}.rs`,
                        render('instructionsPage.njk', {
                            argumentTypes,
                            imports: imports.toString(),
                            instruction: node,
                            program: currentProgram,
                        }),
                    );
                },

                visitProgram(node, { self }) {
                    currentProgram = node;
                    const renderMap = new RenderMap()
                        .mergeWith(...node.accounts.map(account => visit(account, self)))
                        .mergeWith(...node.definedTypes.map(type => visit(type, self)))
                        .mergeWith(
                            ...getAllInstructionsWithSubs(node, {
                                leavesOnly: !renderParentInstructions,
                            }).map(ix => visit(ix, self)),
                        );

                    currentProgram = null;
                    return renderMap;
                },

                visitRoot(node, { self }) {
                    const programsToExport = getAllPrograms(node);
                    const accountsToExport = getAllAccounts(node);
                    const instructionsToExport = getAllInstructionsWithSubs(node, {
                        leavesOnly: !renderParentInstructions,
                    });
                    const definedTypesToExport = getAllDefinedTypes(node);

                    const ctx = {
                        accountsToExport,
                        definedTypesToExport,
                        instructionsToExport,
                        programsToExport,
                        root: node,
                    };

                    const map = new RenderMap();

                    // Generate mod files
                    if (accountsToExport.length > 0) {
                        map.add('accounts/mod.rs', render('accountsMod.njk', ctx));
                    }
                    if (instructionsToExport.length > 0) {
                        map.add('instructions/mod.rs', render('instructionsMod.njk', ctx));
                    }
                    if (definedTypesToExport.length > 0) {
                        map.add('types/mod.rs', render('typesMod.njk', ctx));
                    }

                    // Generate lib.rs
                    map.add('lib.rs', render('lib.njk', ctx));

                    // Process all programs
                    return map.mergeWith(...getAllPrograms(node).map(p => visit(p, self)));
                },
            }),
    );
}
