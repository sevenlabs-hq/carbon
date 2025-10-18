import { TypeNode, isNode, snakeCase, SnakeCaseString } from '@codama/nodes';
import { visit } from '@codama/visitors-core';
import { getGraphQLTypeManifestVisitor, GraphQLTypeManifest } from '../getGraphQLTypeManifestVisitor';
import { buildConversionFromOriginal, buildConversionFromPostgresRow } from './convertGraphQLTypes';

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
): FlattenedGraphQLField[] {
    const graphqlTypeManifestVisitor = getGraphQLTypeManifestVisitor();
    const out: FlattenedGraphQLField[] = [];

    const makeName = (nameParts: string[]) => {
        let fieldName = snakeCase(nameParts.join('_'));
        if (seen.has(fieldName)) {
            let i = 1;
            while (seen.has(`${fieldName}_${i}`)) i++;
            fieldName = `${fieldName}_${i}` as SnakeCaseString;
        }
        seen.add(fieldName);
        return fieldName;
    };

    if (isNode(typeNode, 'structTypeNode')) {
        for (const field of typeNode.fields) {
            out.push(...flattenTypeForGraphQL(field.type, [...prefix, snakeCase(field.name)], field.docs || [], seen));
        }
        return out;
    }

    const manifest = visit(typeNode, graphqlTypeManifestVisitor) as GraphQLTypeManifest;
    const fieldName = makeName(prefix);

    const field: FlattenedGraphQLField = {
        fieldName,
        rustPath: prefix.join('.'),
        rustType: manifest.graphqlType,
        docs: docsPrefix,
        graphqlManifest: manifest,
        fromRowExpr: '',
        fromOriginalExpr: '',
    };

    field.fromRowExpr = buildConversionFromPostgresRow(typeNode, `row.${field.fieldName}`);
    field.fromOriginalExpr = buildConversionFromOriginal(typeNode, `original.${field.fieldName}`);

    out.push(field);

    return out;
}
