import { dirname as pathDirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

import { camelCase, kebabCase, pascalCase, snakeCase, titleCase } from '@codama/nodes';
import nunjucks, { ConfigureOptions as NunJucksOptions } from 'nunjucks';

export function rustDocblock(docs: string[]): string {
    if (docs.length <= 0) return '';
    const lines = docs.map(doc => `/// ${doc}`);
    return `${lines.join('\n')}\n`;
}

export function formatDocComments(docs: string[], indent: string = ''): string {
    if (docs.length === 0) {
        return '';
    }
    // Process all docs together as one unit to maintain list context across doc strings
    // This is critical for proper indentation of continuation lines after list items
    const allLines = docs.join('\n').split('\n');
    const result: string[] = [];
    let inListContext = false;

    for (let i = 0; i < allLines.length; i++) {
        const line = allLines[i];
        const trimmed = line.trim();

        // Skip empty lines completely - never output them
        if (trimmed === '') {
            // Empty line resets list context
            inListContext = false;
            continue;
        }

        const isListItem = trimmed.startsWith('*') || trimmed.startsWith('-');

        if (isListItem) {
            // List item - output trimmed version, set context to true
            result.push(`${indent}/// ${trimmed}`);
            inListContext = true;
        } else {
            // Not a list item
            // First check if previous non-empty line was a list item (for cases where context was lost)
            if (!inListContext) {
                let prevWasListItem = false;
                let prevWasContinuation = false;
                for (let j = i - 1; j >= 0; j--) {
                    const prevTrimmed = allLines[j].trim();
                    if (prevTrimmed !== '') {
                        prevWasListItem = prevTrimmed.startsWith('*') || prevTrimmed.startsWith('-');
                        // Check if previous line was a continuation (indented)
                        // Look at the result array to see if the last line was indented
                        if (result.length > 0) {
                            const lastResult = result[result.length - 1];
                            prevWasContinuation = lastResult.includes('///   ');
                        }
                        break;
                    }
                }
                if (prevWasListItem || prevWasContinuation) {
                    // Previous line was a list item or continuation, so this is a continuation
                    inListContext = true;
                }
            }

            if (inListContext) {
                // Continuation line after list item - indent it with 3 spaces after ///
                // CRITICAL: Must use exactly 3 spaces after /// for clippy doc_lazy_continuation
                result.push(`${indent}///   ${trimmed}`);
                // Keep context true for subsequent continuation lines
                // Only reset when we see a new list item (handled above) or empty line (handled in the empty line check)
            } else {
                // Regular line, not in list context
                result.push(`${indent}/// ${trimmed}`);
                inListContext = false;
            }
        }
    }

    // Join with newlines - no trailing newline to avoid empty_line_after_doc_comment errors
    return result.join('\n');
}

export const render = (template: string, context?: object, options?: NunJucksOptions): string => {
    // Get templates directory from the package directory
    const isESM = typeof import.meta !== 'undefined';
    const dirname = isESM ? pathDirname(fileURLToPath(import.meta.url)) : __dirname;
    const templates = join(dirname, '..', 'templates'); // Path to templates from src/utils

    const env = nunjucks.configure(templates, { autoescape: false, trimBlocks: true, ...options });
    env.addFilter('pascalCase', pascalCase);
    env.addFilter('camelCase', camelCase);
    env.addFilter('snakeCase', snakeCase);
    env.addFilter('kebabCase', kebabCase);
    env.addFilter('titleCase', titleCase);
    env.addFilter('rustDocblock', rustDocblock);
    env.addFilter('formatDocComments', (docs: string[], indent: string = '') => formatDocComments(docs, indent));
    return env.render(template, context);
};
