import { TypeNode, isNode, snakeCase, SnakeCaseString } from '@codama/nodes';
import { visit } from '@codama/visitors-core';
import { getGraphQLTypeManifestVisitor, GraphQLTypeManifest } from '../getGraphQLTypeManifestVisitor';
import { buildConversionFromOriginal, buildConversionFromPostgresRow } from './convertGraphQLTypes';
import { escapeRustKeyword } from '../constants/rustKeywords';

export type FlattenedGraphQLField = {
    fieldName: string;
    rustPath: string;
    rustType: string;
    docs: string[];
    graphqlManifest: GraphQLTypeManifest;
    fromRowExpr: string;
    fromOriginalExpr: string;
};

export function flattenTypeForGraphQL(
    typeNode: TypeNode,
    prefix: string[],
    docsPrefix: string[],
    seen: Set<string>,
    seenRow: Set<string> = new Set(),
): FlattenedGraphQLField[] {
    const graphqlTypeManifestVisitor = getGraphQLTypeManifestVisitor();
    const out: FlattenedGraphQLField[] = [];

    const makeRowName = (nameParts: string[]) => {
        const joined = nameParts.join('_');
        let fieldName = escapeRustKeyword(snakeCase(joined));
        if (joined.startsWith('_') && !fieldName.startsWith('_')) {
            fieldName = `_${fieldName}` as SnakeCaseString;
        }
        if (seenRow.has(fieldName)) {
            let i = 0;
            while (seenRow.has(`${fieldName}_${i}`)) i++;
            fieldName = `${fieldName}_${i}` as SnakeCaseString;
        }
        seenRow.add(fieldName);
        return fieldName;
    };

    const makeGraphqlName = (rowFieldName: string) => {
        const normalized = rowFieldName.replace(/^_+/, '');
        let fieldName = escapeRustKeyword(snakeCase(normalized.length > 0 ? normalized : 'field'));
        if (seen.has(fieldName)) {
            let i = 0;
            while (seen.has(`${fieldName}_${i}`)) i++;
            fieldName = `${fieldName}_${i}` as SnakeCaseString;
        }
        seen.add(fieldName);
        return fieldName;
    };

    if (isNode(typeNode, 'structTypeNode')) {
        const baseNames: SnakeCaseString[] = typeNode.fields.map(field => {
            const rawName = field.name;
            let base = escapeRustKeyword(snakeCase(rawName));
            if (rawName.startsWith('_') && !base.startsWith('_')) {
                base = `_${base}` as SnakeCaseString;
            }
            return base as SnakeCaseString;
        });

        const totalByName = new Map<string, number>();
        const occurrenceByName = new Map<string, number>();
        const usedFieldNames = new Set<string>();

        for (const name of baseNames) {
            totalByName.set(name, (totalByName.get(name) ?? 0) + 1);
        }

        const getUniqueFieldName = (baseFieldName: SnakeCaseString): SnakeCaseString => {
            const total = totalByName.get(baseFieldName) ?? 0;
            const occurrence = occurrenceByName.get(baseFieldName) ?? 0;
            occurrenceByName.set(baseFieldName, occurrence + 1);

            let candidate = baseFieldName as string;
            if (total > 1) {
                if (occurrence === 0) {
                    candidate = `_${baseFieldName}`;
                } else if (occurrence === 1) {
                    candidate = baseFieldName;
                } else {
                    candidate = `${baseFieldName}_${occurrence - 2}`;
                }
            }

            if (!usedFieldNames.has(candidate)) {
                usedFieldNames.add(candidate);
                return candidate as SnakeCaseString;
            }

            let suffix = 0;
            let fallback = `${candidate}_${suffix++}`;
            while (usedFieldNames.has(fallback)) {
                fallback = `${candidate}_${suffix++}`;
            }
            usedFieldNames.add(fallback);
            return fallback as SnakeCaseString;
        };

        for (let i = 0; i < typeNode.fields.length; i++) {
            const field = typeNode.fields[i];
            const uniqueFieldName = getUniqueFieldName(baseNames[i]);
            out.push(
                ...flattenTypeForGraphQL(field.type, [...prefix, uniqueFieldName], field.docs || [], seen, seenRow),
            );
        }
        return out;
    }

    const manifest = visit(typeNode, graphqlTypeManifestVisitor) as GraphQLTypeManifest;
    const rowFieldName = makeRowName(prefix);
    const fieldName = makeGraphqlName(rowFieldName);

    const field: FlattenedGraphQLField = {
        fieldName,
        rustPath: prefix.join('.'),
        rustType: manifest.graphqlType,
        docs: docsPrefix,
        graphqlManifest: manifest,
        fromRowExpr: '',
        fromOriginalExpr: '',
    };

    field.fromRowExpr = buildConversionFromPostgresRow(typeNode, `row.${rowFieldName}`);
    field.fromOriginalExpr = buildConversionFromOriginal(typeNode, `original.${field.rustPath}`);

    out.push(field);

    return out;
}
