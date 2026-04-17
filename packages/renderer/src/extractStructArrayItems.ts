// Extracts anonymous struct array items into named defined types.
// Rust can't express Vec<{anonymous struct}>, so arrayTypeNode(structTypeNode(...))
// becomes {InstructionName}Item in definedTypes, replaced with a definedTypeLinkNode.

import {
    arrayTypeNode,
    definedTypeLinkNode,
    definedTypeNode,
    instructionArgumentNode,
    instructionNode,
    isNode,
    pascalCase,
    programNode,
    rootNode,
    RootNode,
} from '@codama/nodes';
import { rootNodeVisitor } from '@codama/visitors-core';

export function extractStructArrayItems() {
    return rootNodeVisitor((root: RootNode) => {
        const program = root.program;
        const extracted: ReturnType<typeof definedTypeNode>[] = [];

        const newInstructions = program.instructions.map(instr => {
            const newArguments = instr.arguments.map(arg => {
                if (!isNode(arg.type, 'arrayTypeNode') || !isNode(arg.type.item, 'structTypeNode')) return arg;

                const typeName = pascalCase(instr.name) + 'Item';
                if (!extracted.some(t => t.name === typeName)) {
                    extracted.push(definedTypeNode({ name: typeName, type: arg.type.item }));
                }

                return instructionArgumentNode({
                    name: arg.name,
                    type: arrayTypeNode(definedTypeLinkNode(typeName), arg.type.count),
                    docs: arg.docs,
                    defaultValue: arg.defaultValue,
                    defaultValueStrategy: arg.defaultValueStrategy,
                });
            });

            return instructionNode({
                name: instr.name,
                docs: instr.docs,
                accounts: instr.accounts,
                arguments: newArguments,
                extraArguments: instr.extraArguments,
                remainingAccounts: instr.remainingAccounts,
                byteDeltas: instr.byteDeltas,
                discriminators: instr.discriminators,
                subInstructions: instr.subInstructions,
                optionalAccountStrategy: instr.optionalAccountStrategy,
            });
        });

        if (extracted.length === 0) return root;

        return rootNode(
            programNode({
                ...program,
                instructions: newInstructions,
                definedTypes: [...program.definedTypes, ...extracted],
            }),
            root.additionalPrograms,
        );
    });
}
